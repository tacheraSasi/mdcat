// Copyright 2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Command line application to render markdown to TTYs.
//!
//! Note that as of version 2.0.0 mdcat itself no longer contains the core rendering functions.
//! Use [`pulldown_cmark_mdcat`] instead.

#![deny(warnings, missing_docs, clippy::all)]
#![forbid(unsafe_code)]

use std::fs::File;
use std::io::stdin;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;

use anyhow::{Context, Result};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark_mdcat::resources::{
    DispatchingResourceHandler, FileResourceHandler, ResourceUrlHandler,
};
use pulldown_cmark_mdcat::{Environment, Settings};
use resources::CurlResourceHandler;
use tracing::{event, instrument, Level};

use args::ResourceAccess;
use output::Output;

/// Argument parsing for mdcat.
#[allow(missing_docs)]
pub mod args;
/// Output handling for mdcat.
pub mod output;
/// Resource handling for mdca.
pub mod resources;
/// Statistics and line number handling for mdcat.
pub mod stats;

/// Default read size limit for resources.
pub static DEFAULT_RESOURCE_READ_LIMIT: u64 = 104_857_600;

/// Read input for `filename`.
///
/// If `filename` is `-` read from standard input, otherwise try to open and
/// read the given file.
pub fn read_input<T: AsRef<str>>(filename: T) -> Result<(PathBuf, String)> {
    let cd = std::env::current_dir()?;
    let mut buffer = String::new();

    if filename.as_ref() == "-" {
        stdin().read_to_string(&mut buffer)?;
        Ok((cd, buffer))
    } else {
        let mut source = File::open(filename.as_ref())?;
        source.read_to_string(&mut buffer)?;
        let base_dir = cd
            .join(filename.as_ref())
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(cd);
        Ok((base_dir, buffer))
    }
}

/// Process a single file.
///
/// Read from `filename` and render the contents to `output`.
#[instrument(skip(output, settings, resource_handler), level = "debug")]
pub fn process_file(
    filename: &str,
    settings: &Settings,
    resource_handler: &dyn ResourceUrlHandler,
    output: &mut Output,
    show_line_numbers: bool,
    show_stats: bool,
) -> Result<()> {
    let (base_dir, input) = read_input(filename)?;
    event!(
        Level::TRACE,
        "Read input, using {} as base directory",
        base_dir.display()
    );
    
    // Calculate statistics if requested
    if show_stats {
        let stats = stats::DocumentStats::from_markdown(&input);
        writeln!(output.writer(), "{}", stats.format())?;
        if !show_line_numbers {
            // If only stats are requested, don't render the full document
            return Ok(());
        }
    }
    
    let parser = Parser::new_ext(
        &input,
        Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
    );
    let env = Environment::for_local_directory(&base_dir)?;

    let mut sink = BufWriter::new(output.writer());
    
    // If line numbers are enabled, we need to process the content differently
    if show_line_numbers {
        let total_lines = input.lines().count();
        let line_number_width = total_lines.to_string().len();
        
        // Add line numbers to each line
        let lines: Vec<String> = input.lines().enumerate().map(|(i, line)| {
            format!("{:>width$} │ {}", i + 1, line, width = line_number_width)
        }).collect();
        
        let numbered_input = lines.join("\n");
        let parser = Parser::new_ext(
            &numbered_input,
            Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
        );
        
        pulldown_cmark_mdcat::push_tty(settings, &env, resource_handler, &mut sink, parser)
            .and_then(|_| {
                event!(Level::TRACE, "Finished rendering, flushing output");
                sink.flush()
            })
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::BrokenPipe {
                    event!(Level::TRACE, "Ignoring broken pipe");
                    Ok(())
                } else {
                    event!(Level::ERROR, ?error, "Failed to process file: {:#}", error);
                    Err(error)
                }
            })?;
    } else {
        pulldown_cmark_mdcat::push_tty(settings, &env, resource_handler, &mut sink, parser)
            .and_then(|_| {
                event!(Level::TRACE, "Finished rendering, flushing output");
                sink.flush()
            })
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::BrokenPipe {
                    event!(Level::TRACE, "Ignoring broken pipe");
                    Ok(())
                } else {
                    event!(Level::ERROR, ?error, "Failed to process file: {:#}", error);
                    Err(error)
                }
            })?;
    }
    
    Ok(())
}

/// Create the resource handler for mdcat.
pub fn create_resource_handler(access: ResourceAccess) -> Result<DispatchingResourceHandler> {
    let mut resource_handlers: Vec<Box<dyn ResourceUrlHandler>> = vec![Box::new(
        FileResourceHandler::new(DEFAULT_RESOURCE_READ_LIMIT),
    )];
    if let ResourceAccess::Remote = access {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        event!(
            target: "mdcat::main",
            Level::DEBUG,
            "Remote resource access permitted, creating HTTP client with user agent {}",
            user_agent
        );
        let client = CurlResourceHandler::create(DEFAULT_RESOURCE_READ_LIMIT, user_agent)
            .with_context(|| "Failed to build HTTP client".to_string())?;
        resource_handlers.push(Box::new(client));
    }
    Ok(DispatchingResourceHandler::new(resource_handlers))
}

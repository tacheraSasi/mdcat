// Copyright 2024 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{Result, Write};
use pulldown_cmark::{Event, Parser, Options};

/// Statistics about a markdown document.
#[derive(Debug, Default)]
pub struct DocumentStats {
    /// Total number of characters (including whitespace).
    pub character_count: usize,
    /// Total number of words.
    pub word_count: usize,
    /// Number of lines.
    pub line_count: usize,
    /// Number of headings.
    pub heading_count: usize,
    /// Number of code blocks.
    pub code_block_count: usize,
    /// Number of links.
    pub link_count: usize,
    /// Number of images.
    pub image_count: usize,
    /// Number of lists.
    pub list_count: usize,
    /// Number of tables.
    pub table_count: usize,
}

impl DocumentStats {
    /// Calculate statistics from markdown content.
    pub fn from_markdown(content: &str) -> Self {
        let mut stats = DocumentStats::default();
        
        // Count characters and lines
        stats.character_count = content.len();
        stats.line_count = content.lines().count();
        
        // Count words (simple whitespace-based counting)
        stats.word_count = content
            .split_whitespace()
            .count();
        
        // Parse markdown to count structural elements
        let parser = Parser::new_ext(
            content,
            Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
        );
        
        for event in parser {
            match event {
                Event::Start(pulldown_cmark::Tag::Heading { .. }) => {
                    stats.heading_count += 1;
                }
                Event::Start(pulldown_cmark::Tag::CodeBlock(_)) => {
                    stats.code_block_count += 1;
                }
                Event::Start(pulldown_cmark::Tag::Link(_, _, _)) => {
                    stats.link_count += 1;
                }
                Event::Start(pulldown_cmark::Tag::Image(_, _, _)) => {
                    stats.image_count += 1;
                }
                Event::Start(pulldown_cmark::Tag::List(_)) => {
                    stats.list_count += 1;
                }
                Event::Start(pulldown_cmark::Tag::Table(_)) => {
                    stats.table_count += 1;
                }
                _ => {}
            }
        }
        
        stats
    }
    
    /// Calculate estimated reading time in minutes.
    /// Based on average reading speed of 200-250 words per minute.
    pub fn reading_time_minutes(&self) -> usize {
        const WORDS_PER_MINUTE: usize = 225;
        (self.word_count + WORDS_PER_MINUTE - 1) / WORDS_PER_MINUTE // Ceiling division
    }
    
    /// Format statistics for display.
    pub fn format(&self) -> String {
        let reading_time = self.reading_time_minutes();
        format!(
            "Document Statistics:\n\
             ───────────────────\n\
             Characters: {}\n\
             Words: {}\n\
             Lines: {}\n\
             Headings: {}\n\
             Code blocks: {}\n\
             Links: {}\n\
             Images: {}\n\
             Lists: {}\n\
             Tables: {}\n\
             Estimated reading time: {} minute{}\n",
            self.character_count,
            self.word_count,
            self.line_count,
            self.heading_count,
            self.code_block_count,
            self.link_count,
            self.image_count,
            self.list_count,
            self.table_count,
            reading_time,
            if reading_time == 1 { "" } else { "s" }
        )
    }
}

/// Line number formatter for markdown output.
pub struct LineNumberFormatter {
    current_line: usize,
    show_line_numbers: bool,
    line_number_width: usize,
}

impl LineNumberFormatter {
    /// Create a new line number formatter.
    pub fn new(show_line_numbers: bool, total_lines: usize) -> Self {
        let line_number_width = if show_line_numbers {
            total_lines.to_string().len()
        } else {
            0
        };
        
        Self {
            current_line: 0,
            show_line_numbers,
            line_number_width,
        }
    }
    
    /// Write a line number prefix if line numbers are enabled.
    pub fn write_line_number<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        if self.show_line_numbers {
            self.current_line += 1;
            write!(
                writer,
                "{:>width$} │ ",
                self.current_line,
                width = self.line_number_width
            )?;
        }
        Ok(())
    }
    
    /// Write a newline and prepare for the next line.
    pub fn write_newline<W: Write>(&mut self, writer: &mut W) -> Result<()> {
        writeln!(writer)?;
        Ok(())
    }
    
    /// Get the current line number.
    pub fn current_line(&self) -> usize {
        self.current_line
    }
} 
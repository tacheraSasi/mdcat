// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::ValueHint;
use clap_complete::Shell;

fn after_help() -> &'static str {
    "See 'man 1 mdcat' for more information.

mdcat can be installed as or linked to mdless,
for automatic pagination.

Report issues to <https://github.com/swsnr/mdcat>."
}

fn long_version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "
Copyright (C) Sebastian Wiesner and contributors

This program is subject to the terms of the Mozilla Public License,
v. 2.0. If a copy of the MPL was not distributed with this file,
You can obtain one at http://mozilla.org/MPL/2.0/."
    )
}

#[derive(Debug, clap::Parser)]
#[command(multicall = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    #[command(version, about, after_help = after_help(), long_version = long_version())]
    Mdcat {
        #[command(flatten)]
        args: CommonArgs,
        /// Paginate the output of mdcat with a pager like less (default for mdless).
        #[arg(short, long, overrides_with = "no_pager")]
        paginate: bool,
        /// Do not paginate output (default). Overrides an earlier --paginate.
        #[arg(short = 'P', long)]
        no_pager: bool,
    },
    #[command(version, about, after_help = after_help(), long_version = long_version())]
    Mdless {
        #[command(flatten)]
        args: CommonArgs,
        /// Do not paginate output (default for mdcat).
        #[arg(short = 'P', long, overrides_with = "paginate")]
        no_pager: bool,
        /// Paginate the output of mdcat with a pager like less (default). Overrides an earlier --no-pager.
        #[arg(short, long)]
        paginate: bool,
    },
}

impl Command {
    pub fn paginate(&self) -> bool {
        match *self {
            // In both cases look at the option indicating the non-default
            // behaviour; the overrides above are configured accordingly.
            Command::Mdcat { paginate, .. } => paginate,
            Command::Mdless { no_pager, .. } => !no_pager,
        }
    }
}

impl std::ops::Deref for Command {
    type Target = CommonArgs;

    fn deref(&self) -> &Self::Target {
        match self {
            Command::Mdcat { args, .. } => args,
            Command::Mdless { args, .. } => args,
        }
    }
}

#[derive(Debug, clap::Args)]
// #[command(author, version, about, after_help = after_help(), long_version = long_version())]
pub struct CommonArgs {
    /// Files to read.  If - read from standard input instead.
    #[arg(default_value="-", value_hint = ValueHint::FilePath)]
    pub filenames: Vec<String>,
    /// Disable all colours and other styles.
    #[arg(short = 'c', long, aliases=["nocolour", "no-color", "nocolor"])]
    pub no_colour: bool,
    /// Maximum number of columns to use for output.
    #[arg(long)]
    pub columns: Option<u16>,
    /// Do not load remote resources like images.
    #[arg(short, long = "local")]
    pub local_only: bool,
    /// Exit immediately if any error occurs processing an input file.
    #[arg(long = "fail")]
    pub fail_fast: bool,
    /// Print detected terminal name and exit.
    #[arg(long = "detect-terminal")]
    pub detect_and_exit: bool,
    /// Skip terminal detection and only use ANSI formatting.
    #[arg(long = "ansi", conflicts_with = "no_colour")]
    pub ansi_only: bool,
    /// Show line numbers in the output.
    #[arg(long = "line-numbers")]
    pub line_numbers: bool,
    /// Display statistics about the document (word count, character count, etc.).
    #[arg(long = "stats")]
    pub stats: bool,
    /// Generate completions for a shell to standard output and exit.
    #[arg(long)]
    pub completions: Option<Shell>,
}

/// What resources mdcat may access.
#[derive(Debug, Copy, Clone)]
pub enum ResourceAccess {
    /// Only allow local resources.
    LocalOnly,
    /// Allow remote resources
    Remote,
}

impl CommonArgs {
    /// Whether remote resource access is permitted.
    pub fn resource_access(&self) -> ResourceAccess {
        if self.local_only {
            ResourceAccess::LocalOnly
        } else {
            ResourceAccess::Remote
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::CommandFactory;

    #[test]
    fn verify_app() {
        Args::command().debug_assert();
    }
}

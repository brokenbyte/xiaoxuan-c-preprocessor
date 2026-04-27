// Copyright (c) 2026 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions.
// For more details, see the LICENSE, LICENSE.additional, and CONTRIBUTING files.

use std::fmt::Display;

use crate::{
    position::Position,
    range::Range,
    source_message::{attach_with_snippet_by_position, attach_with_snippet_by_range},
};

/// `Linter` is different from `Error`, it is useful messages report to the user,
/// such as suggestions and warnings. It is not an error that prevents preprocessing,
/// but it can be helpful for users to understand potential issues or improvements in their code.
#[derive(Debug, PartialEq)]
pub enum Linter {
    Message(
        LintLevel,
        /* file number */ usize,
        /* message */ String,
    ),
    MessageWithPosition(
        LintLevel,
        /* file number */ usize,
        /* message */ String,
        Position,
    ),
    MessageWithRange(
        LintLevel,
        /* file number */ usize,
        /* message */ String,
        Range,
    ),
}

#[derive(Debug, PartialEq)]
pub enum LintLevel {
    /// A suggestion is a recommendation for improving the code, but it does not indicate a potential issue.
    Info,

    /// A warning indicates a potential issue in the code that may lead to unexpected behavior or bugs.
    Warn,
}

impl Display for LintLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintLevel::Info => write!(f, "Info"),
            LintLevel::Warn => write!(f, "Warn"),
        }
    }
}

impl Linter {
    /// Generates a source message for the linter, including the message and the relevant snippet of source code.
    /// The full path of header file can be obtained from the file number using the `HeaderFileCache`.
    pub fn source_message(
        &self,
        source_file_canonical_full_path: &str,
        source_text_content: &str,
    ) -> String {
        match self {
            Linter::Message(lint_level, _, msg) => {
                let title = format!("{}: {}", lint_level, msg);
                let file = format!("File: {}", source_file_canonical_full_path);

                format!("{}\n{}", title, file)
            }
            Linter::MessageWithPosition(lint_level, _, msg, position) => {
                let title = format!("{}:", lint_level);
                let message =
                    attach_with_snippet_by_position(source_text_content, position.index, msg);
                let file = format!("File: {}", source_file_canonical_full_path);
                let location = format!(
                    "Position: line {}, column {}",
                    position.line + 1,
                    position.column + 1
                );

                format!("{}\n{}\n{}\n{}", title, message, location, file)
            }
            Linter::MessageWithRange(lint_level, _, msg, range) => {
                let title = format!("{}:", lint_level);

                let message = attach_with_snippet_by_range(
                    source_text_content,
                    range.start.index,
                    range.end_inclusive.index - range.start.index + 1,
                    msg,
                );
                let file = format!("File: {}", source_file_canonical_full_path);
                let location = format!(
                    "Position: line {}, column {}",
                    range.start.line + 1,
                    range.start.column + 1,
                );
                format!("{}\n{}\n{}\n{}", title, message, location, file)
            }
        }
    }
}

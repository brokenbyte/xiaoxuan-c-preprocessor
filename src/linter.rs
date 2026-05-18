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

/// `Lint` is different from `Error`, it is useful messages report to the user,
/// such as suggestions and warnings. It is not an error that prevents preprocessing,
/// but it can be helpful for users to understand potential issues or improvements in their code.
#[derive(Debug, PartialEq)]
pub enum Lint {
    Message(
        /* lint_level */ LintLevel,
        /* suppress_name */ String,
        /* file_number */ usize,
        /* message */ String,
    ),
    MessageWithPosition(
        /* lint_level */ LintLevel,
        /* suppress_name */ String,
        /* file_number */ usize,
        /* message */ String,
        Position,
    ),
    MessageWithRange(
        /* lint_level */ LintLevel,
        /* suppress_name */ String,
        /* file_number */ usize,
        /* message */ String,
        Range,
    ),
}

/// The severity level of a lint message, which can be either `Info` or `Warn`.
///
/// - `Info`: A suggestion is a recommendation for improving the code, but it does not indicate a potential issue.
/// - `Warn`: A warning indicates a potential issue in the code that may lead to unexpected behavior or bugs.
///
/// Actually, there are additional lint levels `Error` and `Disable`:
///
/// - `Error`: An error indicates a critical issue in the code that must be addressed,
///   as it may cause the program to fail or produce incorrect results, or
///   some features that are not supported by the preprocessor, such as `#line`,
///   can also be reported as errors to inform users that they cannot use these features in their code.
/// - `Disable`: This level is used to indicate that a specific compile-feature is disabled by default,
///   however, these features can be reenabled by configuration.
///
/// Since these levels messages mean that the preprocessor cannot continue processing the code,
/// they are reported as errors instead of lints, and they are not included in the `LintLevel` enum.
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

impl Lint {
    /// Generates a source message for the lint, including the message and the relevant snippet of source code.
    /// The full path of header file can be obtained from the file number using the `HeaderFileCache`.
    pub fn source_message(
        &self,
        source_file_canonical_full_path: &str,
        source_text_content: &str,
    ) -> String {
        match self {
            Lint::Message(suppress_name, lint_level, _, msg) => {
                let title = format!("{}: {}", lint_level, msg);
                let file = format!("File: {}", source_file_canonical_full_path);
                let name = format!("Lint: {}", suppress_name);

                format!("{}\n{}\n{}\n", title, file, name)
            }
            Lint::MessageWithPosition(suppress_name, lint_level, _, msg, position) => {
                let title = format!("{}:", lint_level);
                let message =
                    attach_with_snippet_by_position(source_text_content, position.index, msg);
                let location = format!(
                    "Position: line {}, column {}",
                    position.line + 1,
                    position.column + 1
                );
                let file = format!("File: {}", source_file_canonical_full_path);
                let name = format!("Lint: {}", suppress_name);

                format!("{}\n{}\n{}\n{}\n{}\n", title, message, location, file, name)
            }
            Lint::MessageWithRange(suppress_name, lint_level, _, msg, range) => {
                let title = format!("{}:", lint_level);

                let message = attach_with_snippet_by_range(
                    source_text_content,
                    range.start.index,
                    range.end_inclusive.index - range.start.index + 1,
                    msg,
                );

                let location = format!(
                    "Position: line {}, column {}",
                    range.start.line + 1,
                    range.start.column + 1,
                );
                let file = format!("File: {}", source_file_canonical_full_path);
                let name = format!("Lint: {}", suppress_name);
                format!("{}\n{}\n{}\n{}\n{}\n", title, message, location, file, name)
            }
        }
    }
}

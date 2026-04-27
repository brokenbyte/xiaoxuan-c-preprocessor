// Copyright (c) 2026 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions.
// For more details, see the LICENSE, LICENSE.additional, and CONTRIBUTING files.

use crate::{
    position::Position,
    range::Range,
    source_message::{
        attach_with_snippet_by_last_position, attach_with_snippet_by_position,
        attach_with_snippet_by_range,
    },
};

#[derive(Debug, PartialEq)]
pub enum PreprocessError {
    Message(String),
    UnexpectedEndOfDocument(String),
    MessageWithPosition(String, Position),
    MessageWithRange(String, Range),
}

// impl Display for PreprocessError {
//     fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         unimplemented!()
//     }
// }

// impl std::error::Error for PreprocessError {}

#[derive(Debug, PartialEq)]
pub struct PreprocessFileError {
    pub file_number: usize,
    pub error: PreprocessError,
}

impl PreprocessFileError {
    pub fn new(file_number: usize, error: PreprocessError) -> Self {
        Self { file_number, error }
    }
}

// impl Display for PreprocessFileError {
//     fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         unimplemented!()
//     }
// }

// impl std::error::Error for PreprocessFileError {}

impl PreprocessFileError {
    pub fn source_message(
        &self,
        source_file_canonical_full_path: &str,
        source_text_content: &str,
    ) -> String {
        match &self.error {
            PreprocessError::Message(msg) => {
                let title = format!("Error: {}", msg);
                let file = format!("File: {}", source_file_canonical_full_path);
                format!("{}\n{}", title, file)
            }
            PreprocessError::UnexpectedEndOfDocument(msg) => {
                let title = "Error:".to_owned();
                let message = attach_with_snippet_by_last_position(source_text_content, msg);
                let file = format!("File: {}", source_file_canonical_full_path);
                format!("{}\n{}\n{}", title, message, file)
            }
            PreprocessError::MessageWithPosition(msg, position) => {
                let title = "Error:".to_owned();
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
            PreprocessError::MessageWithRange(msg, range) => {
                let title = "Error:".to_owned();
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

// Copyright (c) 2026 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions.
// For more details, see the LICENSE, LICENSE.additional, and CONTRIBUTING files.

mod char_with_position;
mod expression;
mod initializer;
mod lexer;
mod parser;
mod processor;

pub mod ast;
pub mod ast_printer;
pub mod consts;
pub mod context;
pub mod error;
pub mod linter;
pub mod location;
pub mod memory_file_provider;
pub mod native_file_provider;
pub mod peekable_iter;
pub mod position;
pub mod range;
pub mod source_message;
pub mod token;

pub use processor::process_source_file;

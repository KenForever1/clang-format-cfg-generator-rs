use std::io;

use thiserror::Error;


// Define the result type using a custom error
pub(crate) type ParseResult<T> = std::result::Result<T, ParseError>;


// 定义自定义错误类型
#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("Undefined reference or target file.")]
    UndefinedFilePath,
    #[error("Reference file not found.")]
    FileNotFound,
    #[error("Reference file is empty.")]
    EmptyFile,
    #[error("Could not open reference file.")]
    FileOpenError(#[from] io::Error),
    // #[error("Could not create clang format file.")]
    // CreateFileError,
    #[error("Could not write to clang format file. {0}")]
    WriteFileError(String),
    #[error("Invalid command line arguments.")]
    InvalidArguments,
    #[error("Invalid argument for clang-format version.")]
    InvalidVersionArgument,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
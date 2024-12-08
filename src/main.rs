mod clang_format_lib;
mod error;
mod generator;
mod parser;
mod write_cfg;

use clang_format_lib::{ClangFormatSettings, Parser};
use clap::{Arg, Command};
use error::{ParseError, ParseResult};
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, Write};
use std::path::Path;
use std::rc::Rc;

type TextFileContent = Vec<String>;

// Function to return the correct line ending based on the OS
fn line_ending() -> &'static [u8] {
    #[cfg(windows)]
    {
        b"\r\n"
    }

    #[cfg(not(windows))]
    {
        b"\n"
    }
}

fn write_to_file(dst: &Path, lines: &TextFileContent) -> std::result::Result<(), String> {
    // let mut file = match File::create(dst) {
    //     Ok(file) => file,
    //     Err(_) => return Err(String::from("Could not open file to write.")),
    // };

    let mut file = match OpenOptions::new().write(true).create(true).open(dst) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Could not open file to write.")),
    };

    for line in lines {
        // if let Err(_) = writeln!(file, "{}", line) {
        //     return Err(String::from("Failure writing to the file."));
        // }
        if let Err(_) = file.write_all(line.as_bytes()) {
            return Err(String::from("Failure writing to the file."));
        }

        // Conditionally compile to use the correct line ending
        if let Err(_) = file.write_all(line_ending()) {
            return Err(String::from("Failure writing to the file."));
        }
        
    }

    if let Err(_) = file.flush() {
        return Err(String::from("Failure closing the file."));
    }

    Ok(())
}

fn create_clang_format_file(
    settings: Rc<RefCell<clang_format_lib::ClangFormatSettings>>,
    dst: &Path,
    version: u32,
) -> Result<(), ParseError> {
    let mut file_content: TextFileContent = vec![];

    write_cfg::write_clang_format_file(&settings.borrow(), version, &mut file_content);

    if let Err(err) = write_to_file(dst, &file_content) {
        return Err(ParseError::WriteFileError(err));
    }

    Ok(())
}

fn parse_clang_format_settings(src: &Path, dst: &Path, version: u32) -> Result<(), ParseError> {
    if src.as_os_str().is_empty() || dst.as_os_str().is_empty() {
        return Err(ParseError::UndefinedFilePath);
    }
    if !src.exists() {
        return Err(ParseError::FileNotFound);
    }

    if fs::metadata(src)?.len() == 0 {
        return Err(ParseError::EmptyFile);
    }

    let file = fs::File::open(src)?;
    let reader = std::io::BufReader::new(file);

    let settings = Rc::new(RefCell::new(ClangFormatSettings::new()));

    let mut parser = parser::Impl::new(settings.clone());

    for line in reader.lines() {
        let line = line?;
        parser.parse_line(&line);
    }

    parser.finish();

    create_clang_format_file(settings, &dst, version)
}


fn make_reference_file(dst: &Path) -> Result<(), ParseError> {
    let mut file_content: TextFileContent = vec![];
    generator::generate_reference_file(&mut file_content);
    if let Err(err) = write_to_file(dst, &file_content) {
        return Err(ParseError::WriteFileError(err));
    } else {
        Ok(())
    }
}


fn run() -> ParseResult<()> {
    let matches = Command::new("ClangFormatParser")
        .version("1.0")
        .about("Parses and processes clang-format settings")
        .arg(Arg::new("src")
            .required_unless_present("reference")
            .help("Source file path"))
        .arg(Arg::new("dst")
            .required_unless_present("reference")
            .help("Destination file path"))
        .arg(Arg::new("clang-version")
            .required_unless_present("reference")
            .help("Clang-format version"))
        .arg(Arg::new("reference")
            .long("reference")
            .help("Make reference file"))
        .get_matches();

    if let Some(reference_file) = matches.get_one::<String>("reference") {
        return make_reference_file(Path::new(reference_file));
    }

    let src = matches.get_one::<String>("src").ok_or(ParseError::InvalidArguments)?;
    let dst = matches.get_one::<String>("dst").ok_or(ParseError::InvalidArguments)?;
    let version_str = matches.get_one::<String>("clang-version").ok_or(ParseError::InvalidArguments)?;

    let version = version_str.parse::<u32>()
        .map_err(|_| ParseError::InvalidVersionArgument)?;

    parse_clang_format_settings(Path::new(src), Path::new(dst), version)
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

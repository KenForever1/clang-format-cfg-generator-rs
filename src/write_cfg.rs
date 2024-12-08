use std::{collections::HashMap, fmt};
use std::fmt::Write;
use crate::clang_format_lib::{VERSION, ALIGNMENT, Setting, ClangFormatSettings};

// Function to convert VERSION to unsigned int
fn version_to_uint(v: &VERSION) -> u32 {
    let map_v_uint: HashMap<VERSION, u32> = vec![
        (VERSION::V3_5, 35),
        (VERSION::V3_7, 37),
        (VERSION::V3_8, 38),
        (VERSION::V5_0, 50),
        (VERSION::V10_0, 100),
        (VERSION::V13_0, 130),
        (VERSION::V14_0, 140),
        (VERSION::V16_0, 160),
        (VERSION::V17_0, 170),
        (VERSION::V18_0, 180),
    ]
    .into_iter()
    .collect();

    // Use the get method to find the version and return 999 if not found
    *map_v_uint.get(&v).unwrap_or(&999)
}

fn in_version(version : u32, introduces: &VERSION) -> bool {
    version >= version_to_uint(introduces)
}

fn in_version_range(version : u32, introduced: &VERSION, outdated: &VERSION) -> bool {
    in_version(version, introduced) && !in_version(version, outdated)
}

fn format_version(version: u32) -> String {
    let major = version / 10;
    let minor = version  - (major * 10);
    format!("{}.{}", major, minor)
}


// Define a trait for converting values to strings
trait ValueToString {
    fn value_to_string(&self) -> String;
}

// Implement the trait for the ALIGNMENT enum
impl ValueToString for ALIGNMENT {
    fn value_to_string(&self) -> String {
        match self {
            ALIGNMENT::LEFT => "Left".to_string(),
            ALIGNMENT::MIDDLE => "Middle".to_string(),
            ALIGNMENT::RIGHT => "Right".to_string(),
        }
    }
}

// Implement the Display trait for ALIGNMENT to use with formatting
impl fmt::Display for ALIGNMENT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value_to_string())
    }
}

// Generic function to convert values to strings
// fn value_to_string<T: fmt::Display>(arg: T) -> String {
//     arg.to_string()
// }

struct Writer<'a> {
    lines: &'a mut Vec<String>,
    version: u32,
}

impl<'a> Writer<'a> {
    fn new(lines: &'a mut Vec<String>, version: u32) -> Self {
        Writer { lines, version }
    }

    fn head(&mut self) {
        self.lines.reserve(24);

        self.lines.push("# created with clang-format-cfg-generator-rs".to_string());

        let line = format!("# created for clang-format version {}", format_version(self.version));
        self.lines.push(line);

        self.new_line();
    }

    fn write_text(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }

    fn new_line(&mut self) {
        self.lines.push(String::new());
    }


    fn write<VALUE>(&mut self, s: &Setting<VALUE>, indentation: bool)
    where
        VALUE: std::fmt::Display + ToString, // Assuming VALUE can be converted to a string
    {
        if !in_version(self.version, &s.version) {
            return;
        }

        let mut oss = String::new();

        if s.is_set() {

            if indentation {
                oss.push_str("  ");
            }

            write!(&mut oss, "{}: ", s.command).unwrap();
            write!(&mut oss, "{}", s.get_value().unwrap().to_string()).unwrap();
        } else {
            write!(&mut oss, "# {}: ?", s.command).unwrap();
        }

        self.lines.push(oss);
    }

    fn in_version<VALUE>(&self, s: &Setting<VALUE>) -> bool where
    VALUE: std::fmt::Display + ToString {
        return in_version(self.version, &s.version)
    }


}

pub fn write_clang_format_file(settings: &ClangFormatSettings, version: u32, lines: &mut Vec<String>) {
    let mut writer = Writer::new(lines, version);

    writer.head();
    writer.write(&settings.language, false);
    writer.new_line();

    writer.write(&settings.use_tab, false);
    writer.write(&settings.indent_width, false);
    writer.write(&settings.column_limit, false);
    writer.write(&settings.max_empty_lines_to_keep, false);
    writer.write(&settings.fix_namespace_comments, false);
    writer.new_line();

    if in_version(version, &VERSION::V3_8) {
        writer.write_text("BreakBeforeBraces: Custom");
        writer.write_text("BraceWrapping:");

        writer.write(&settings.break_before_braces.after_class, true);
        writer.write(&settings.break_before_braces.after_function, true);
        writer.write(&settings.break_before_braces.after_namespace, true);
        writer.write(&settings.break_before_braces.after_struct, true);
        writer.write(&settings.break_before_braces.after_control_statement, true);
        writer.write(&settings.break_before_braces.after_enum, true);
        writer.write(&settings.break_before_braces.before_else, true);
    }

    writer.new_line();

    // Adjust version checking logic as necessary
    if in_version_range(version, &VERSION::V3_7, &VERSION::V10_0) {
        writer.write(&settings.spaces_in_parens.spaces_in_parentheses, false);
    } else if in_version_range(version, &VERSION::V10_0, &VERSION::V17_0) {
        writer.write(&settings.spaces_in_parens.spaces_in_conditional_statement, false);
    } else if in_version(version, &VERSION::V17_0) {
        writer.write_text("SpacesInParens: Custom");
        writer.write_text("SpacesInParensOptions:");

        writer.write(&settings.spaces_in_parens.in_conditional_statements, true);
        writer.write(&settings.spaces_in_parens.other, true);
    }

    writer.new_line();

    writer.write(&settings.space_before.space_before_assignment_operators, false);
    writer.write(&settings.spaces_in_parens.spaces_in_parentheses, false);
    writer.write(&settings.space_before.space_before_square_brackets, false);

    // Alignment
    writer.write(&settings.alignment.pointer_alignment, false);

    if writer.in_version(&settings.alignment.reference_alignment){
        if settings.alignment.pointer_alignment.is_set() && settings.alignment.reference_alignment.is_set() {
            if settings.alignment.pointer_alignment.get_value() == settings.alignment.reference_alignment.get_value() {
                writer.write_text("ReferenceAlignment: Pointer");

            }else{
                writer.write(&settings.alignment.reference_alignment, false);

            }
        }
    }

    writer.new_line();

    // SpaceBeforeParens
    if in_version_range(version, &VERSION::V3_5, &VERSION::V14_0) {
        writer.write(&settings.space_before.space_before_assignment_operators, false);
    } else if in_version(version, &VERSION::V14_0) {
        writer.write_text("SpaceBeforeParens: Custom");
        writer.write_text("SpaceBeforeParensOptions:");

        writer.write(&settings.space_before.space_before_square_brackets, true);
        writer.write(&settings.space_before.space_before_assignment_operators, true);
    }
}
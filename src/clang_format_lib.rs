use std::fmt::Display;

// enum class
#[derive(PartialEq, Eq, Hash)]
pub enum VERSION {
    V3_5,
    V3_7,
    V3_8,
    V5_0,
    V10_0,
    V13_0,
    V14_0,
    V16_0,
    V17_0,
    V18_0,
}

// convert setting struct to rust
pub struct Setting<Arg> where  Arg: Display {
    pub command: &'static str,
    pub data: Option<Arg>,
    pub version: VERSION,
    _marker: std::marker::PhantomData<Arg>,
}

impl<Arg> Setting<Arg> where  Arg: Display {
    pub const fn new(command: &'static str, version: VERSION) -> Self {
        Self {
            command,
            data: None,
            version,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set(&mut self, data: Arg) {
        self.data = Some(data);
    }

    // cast method equivalent
    // pub fn cast<In>(&mut self, input: In)
    // where
    //     In: Into<Arg>,
    // {
    //     self.data = Some(input.into());
    // }

    pub fn is_set(&self) -> bool {
        self.data.is_some()
    }

    pub fn get_value(&self) -> Option<&Arg> {
        self.data.as_ref()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum ALIGNMENT {
    LEFT,
    MIDDLE,
    RIGHT,
}

type SettingText = Setting<&'static str>;
type SettingNumber = Setting<u32>;
type SettingSwitch = Setting<bool>;
type SettingAlignment = Setting<ALIGNMENT>;

pub struct ClangFormatSettings {
    pub language: SettingText,
    pub use_tab: SettingText,
    pub column_limit: SettingNumber,
    pub indent_width: SettingNumber,
    pub max_empty_lines_to_keep: SettingNumber,
    pub alignment: AlignmentSettings,
    pub fix_namespace_comments: SettingSwitch,
    pub break_before_braces: BreakBeforeBracesSettings,
    pub spaces_in_square_brackets: SettingSwitch,
    pub space_before: SpaceBeforeSettings,
    pub spaces_in_parens: SpacesInParensSettings,
    pub space_before_parens: SpaceBeforeParensSettings,
}

pub struct AlignmentSettings {
    pub pointer_alignment: SettingAlignment,
    pub reference_alignment: SettingAlignment,
}

pub struct BreakBeforeBracesSettings {
    pub after_class: SettingSwitch,
    pub after_namespace: SettingSwitch,
    pub after_struct: SettingSwitch,
    pub after_function: SettingSwitch,
    pub after_control_statement: SettingSwitch,
    pub after_enum: SettingSwitch,
    pub before_else: SettingSwitch,
}

pub struct SpaceBeforeSettings {
    pub space_before_assignment_operators: SettingSwitch,
    pub space_before_square_brackets: SettingSwitch,
}

pub struct SpacesInParensSettings {
    pub in_conditional_statements: SettingSwitch,
    pub other: SettingSwitch,
    pub spaces_in_conditional_statement: SettingSwitch,
    pub spaces_in_parentheses: SettingSwitch,
}

pub struct SpaceBeforeParensSettings {
    pub space_before_parens: SettingText,
    pub after_control_statements: SettingSwitch,
    pub after_function_definition_name: SettingSwitch,
}

impl ClangFormatSettings {
    pub fn new() -> Self {
        let mut settings = ClangFormatSettings {
            language: SettingText::new("Language", VERSION::V3_5),
            use_tab: SettingText::new("UseTab", VERSION::V3_7),
            column_limit: SettingNumber::new("ColumnLimit", VERSION::V3_7),
            indent_width: SettingNumber::new("IndentWidth", VERSION::V3_7),
            max_empty_lines_to_keep: SettingNumber::new("MaxEmptyLinesToKeep", VERSION::V3_7),
            alignment: AlignmentSettings {
                pointer_alignment: SettingAlignment::new("PointerAlignment", VERSION::V3_7),
                reference_alignment: SettingAlignment::new("ReferenceAlignment", VERSION::V13_0),
            },
            fix_namespace_comments: SettingSwitch::new("FixNamespaceComments", VERSION::V5_0),
            break_before_braces: BreakBeforeBracesSettings {
                after_class: SettingSwitch::new("AfterClass", VERSION::V3_8),
                after_namespace: SettingSwitch::new("AfterNamespace", VERSION::V3_8),
                after_struct: SettingSwitch::new("AfterStruct", VERSION::V3_8),
                after_function: SettingSwitch::new("AfterFunction", VERSION::V3_8),
                after_control_statement: SettingSwitch::new("AfterControlStatement", VERSION::V3_8),
                after_enum: SettingSwitch::new("AfterEnum", VERSION::V3_8),
                before_else: SettingSwitch::new("BeforeElse", VERSION::V3_8),
            },
            spaces_in_square_brackets: SettingSwitch::new("SpacesInSquareBrackets", VERSION::V3_7),
            space_before: SpaceBeforeSettings {
                space_before_assignment_operators: SettingSwitch::new(
                    "SpaceBeforeAssignmentOperators",
                    VERSION::V3_7,
                ),
                space_before_square_brackets: SettingSwitch::new(
                    "SpaceBeforeSquareBrackets",
                    VERSION::V10_0,
                ),
            },
            spaces_in_parens: SpacesInParensSettings {
                in_conditional_statements: SettingSwitch::new(
                    "InConditionalStatements",
                    VERSION::V17_0,
                ),
                other: SettingSwitch::new("Other", VERSION::V17_0),
                spaces_in_conditional_statement: SettingSwitch::new(
                    "SpacesInConditionalStatement",
                    VERSION::V10_0,
                ),
                spaces_in_parentheses: SettingSwitch::new("SpacesInParentheses", VERSION::V3_7),
            },
            space_before_parens: SpaceBeforeParensSettings {
                space_before_parens: SettingText::new("SpaceBeforeParens", VERSION::V3_5),
                after_control_statements: SettingSwitch::new(
                    "AfterControlStatements",
                    VERSION::V14_0,
                ),
                after_function_definition_name: SettingSwitch::new(
                    "AfterFunctionDefinitionName",
                    VERSION::V14_0,
                ),
            },
        };

        // Initialize settings with default values
        settings.language.set("Cpp");
        settings.use_tab.set("Never");

        settings
    }
}


pub trait Parser {
    fn parse_line(&mut self, line: &str);

    fn finish(&mut self);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clangformat_settings() {
        let settings = ClangFormatSettings::new();
        assert!(true);

        println!("Language Setting: {:?}", settings.language.get_value());
        println!("UseTab Setting: {:?}", settings.use_tab.get_value());
    }
}

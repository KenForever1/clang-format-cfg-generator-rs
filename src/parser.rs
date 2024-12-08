// https://alvalea.gitbooks.io/rust-for-cpp/content/pimpl.html
use crate::clang_format_lib::{ClangFormatSettings, Parser, ALIGNMENT};
use std::rc::Rc;
use std::cell::RefCell;

struct LineInfo {
    line: String,
    settings: Rc<RefCell<ClangFormatSettings>>,
    finished: bool,
}

impl LineInfo {
    fn find(&self, search: &str) -> bool {
        self.line.contains(search)
    }

    fn without(&self, search: &str) -> bool {
        !self.find(search)
    }
}

struct TopicInfo {
    done: bool,
    func: Box<dyn FnMut(&mut LineInfo) -> bool>,
}

fn add_topic<F>(topics: &mut Vec<TopicInfo>, f: F)
where
    F: FnMut(&mut LineInfo) -> bool + 'static,
{
    topics.push(TopicInfo {
        func: Box::new(f),
        done: false,
    });
}

pub(crate) struct Impl {
    settings: Rc<RefCell<ClangFormatSettings>>,
    topics: Vec<TopicInfo>,
}

impl Impl {
    pub fn new(settings: Rc<RefCell<ClangFormatSettings>>) -> Self {
        let mut topics = Vec::with_capacity(15);
        Self::set_topics(&mut topics);
        Self { settings, topics }
    }
    fn run_checks(&mut self, info: &mut LineInfo) {
        for topic in &mut self.topics {
            if !topic.done {
                topic.done = (topic.func)(info);
            }
        }
    }

    fn set_topics(topics: &mut Vec<TopicInfo>) {
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("namespace") && info.without("// namespace");
            if result {
                info.settings.borrow_mut().break_before_braces.after_namespace.set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("class");
            if result {
                (info.settings.borrow_mut().break_before_braces.after_class).set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("struct");
            if result {
                (info.settings.borrow_mut().break_before_braces.after_struct).set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("enum");
            if result {
                (info.settings.borrow_mut().break_before_braces.after_enum).set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("ReferenceClass") && info.without("class");
            if result {
                (info.settings.borrow_mut().break_before_braces.after_function).set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let pos = info.line.find("TYPE_A");
            let result = pos.is_some();
            if let Some(pos) = pos {
                info.settings.borrow_mut().indent_width.set(pos as u32);
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("MAX WIDTH");
            if result {
                info.settings.borrow_mut().column_limit.set(info.line.len() as u32);
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("[");
            if result {
                info.settings.borrow_mut().spaces_in_square_brackets.set(info.find("[ 5 ]"));
                (info.settings.borrow_mut().space_before.space_before_square_brackets).set(info.find(" ["));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("=");
            if result {
                (info.settings.borrow_mut().space_before.space_before_assignment_operators).set(info.find(" ="));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            if info.finished {
                if !info.settings.borrow().fix_namespace_comments.is_set() {
                    info.settings.borrow_mut().fix_namespace_comments.set(false);
                }
                return true;
            }
            let result = info.find("// namespace");
            if result {
                info.settings.borrow_mut().fix_namespace_comments.set(true);
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            if info.find("int* ") {
                info.settings.borrow_mut().alignment.pointer_alignment.set(ALIGNMENT::LEFT);
            } else if info.find("int * ") {
                info.settings.borrow_mut().alignment.pointer_alignment.set(ALIGNMENT::MIDDLE);
            } else if info.find("int *") {
                info.settings.borrow_mut().alignment.pointer_alignment.set(ALIGNMENT::RIGHT);
            }
            info.settings.borrow().alignment.pointer_alignment.is_set()
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            if info.find("float& ") {
                info.settings.borrow_mut().alignment.reference_alignment.set(ALIGNMENT::LEFT);
            } else if info.find("float & ") {
                info.settings.borrow_mut().alignment.reference_alignment.set(ALIGNMENT::MIDDLE);
            } else if info.find("float &") {
                info.settings.borrow_mut().alignment.reference_alignment.set(ALIGNMENT::RIGHT);
            }
            info.settings.borrow().alignment.reference_alignment.is_set()
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            if info.find("( value )") {
                info.settings.borrow_mut().spaces_in_parens.in_conditional_statements.set(true);
                info.settings.borrow_mut().spaces_in_parens.spaces_in_conditional_statement.set(true);
            } else if info.find("(value)") {
                info.settings.borrow_mut().spaces_in_parens.in_conditional_statements.set(false);
                info.settings.borrow_mut().spaces_in_parens.spaces_in_conditional_statement.set(false);
            }
            info.settings.borrow().spaces_in_parens.in_conditional_statements.is_set()
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            if info.find("(int") {
                info.settings.borrow_mut().spaces_in_parens.spaces_in_parentheses.set(false);
                info.settings.borrow_mut().spaces_in_parens.other.set(false);
            } else if info.find("( int") {
                info.settings.borrow_mut().spaces_in_parens.spaces_in_parentheses.set(true);
                info.settings.borrow_mut().spaces_in_parens.other.set(true);
            }
            info.settings.borrow_mut().spaces_in_parens.spaces_in_parentheses.is_set()
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("if");
            if result {
                (info.settings.borrow_mut().break_before_braces.after_control_statement).set(info.without("{"));
            }
            result
        });
    
        add_topic(topics, |info: &mut LineInfo| {
            let result = info.find("else");
            if result {
                info.settings.borrow_mut().break_before_braces.before_else.set(info.without("}"));
            }
            result
        });
    
        let mut empty_lines = EmptyLines {
            max_consecutive_empty_lines: 0,
            consecutive_empty_lines: 0,
        };
    
        add_topic(topics, move |info: &mut LineInfo| {
            if info.finished {
                info.settings.borrow_mut().max_empty_lines_to_keep.set(empty_lines.max_consecutive_empty_lines);
                return true;
            }
            if info.line.is_empty() {
                empty_lines.increment();
            } else {
                empty_lines.reset();
            }
            false
        });
    
        let mut space_after = SpaceAfter {
            space_after_if: false,
            space_after_function: false,
        };
    
        add_topic(topics, move |info: &mut LineInfo| {
            if info.finished || (space_after.space_after_if && space_after.space_after_function) {
                if space_after.space_after_function && space_after.space_after_if {
                    info.settings.borrow_mut().space_before_parens.space_before_parens.set("Always");
                } else if !space_after.space_after_function && !space_after.space_after_if {
                    info.settings.borrow_mut().space_before_parens.space_before_parens.set("Never");
                } else if !space_after.space_after_function && space_after.space_after_if {
                    info.settings.borrow_mut().space_before_parens.space_before_parens.set("ControlStatements");
                }
    
                info.settings.borrow_mut().space_before_parens.after_control_statements.set(space_after.space_after_if);
                info.settings.borrow_mut().space_before_parens.after_function_definition_name.set(space_after.space_after_function);
    
                return true;
            }
    
            if info.find("if (") {
                space_after.space_after_if = true;
            } else if info.find("ReferenceClass (") {
                space_after.space_after_function = true;
            }
    
            false
        });
    }
    
}

impl Parser for Impl {
    fn parse_line(&mut self, line: &str) {
        let mut info = LineInfo {
            line: line.to_string(),
            settings: self.settings.clone(),
            finished: false,
        };
        self.run_checks(&mut info);
    }

    fn finish(&mut self) {
        let mut info = LineInfo {
            line: "".to_string(),
            settings: self.settings.clone(),
            finished: true,
        };
        self.run_checks(&mut info);
    }
}


struct EmptyLines {
    max_consecutive_empty_lines: u32,
    consecutive_empty_lines: u32,
}

impl EmptyLines {
    fn increment(&mut self) {
        self.consecutive_empty_lines += 1;
        if self.consecutive_empty_lines > self.max_consecutive_empty_lines {
            self.max_consecutive_empty_lines = self.consecutive_empty_lines;
        }
    }

    fn reset(&mut self) {
        self.consecutive_empty_lines = 0;
    }
}

struct SpaceAfter {
    space_after_if: bool,
    space_after_function: bool,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lineinfo() {
        let settings = Rc::new(RefCell::new(ClangFormatSettings::new()));
        let  info = LineInfo {
            line: "Hello World".to_string(),
            settings: settings.clone(),
            finished: false,
        };
        assert!(info.find("World"));
        assert!(!info.without("World"));
    }


    fn test_add_topic<F>(f: F) {
        let mut topics: Vec<TopicInfo> = Vec::new();

        add_topic(&mut topics, |info: &mut LineInfo| {
            // Example logic that might use info
            println!("Processing line info");
            true // Return a boolean value
        });

        // Example of using the stored function
        if let Some(topic) = topics.first_mut() {
            let mut line_info = LineInfo {
                line: "Hello World".to_string(),
                settings: Rc::new(RefCell::new(ClangFormatSettings::new())),
                finished: false,
            };
            let result = (topic.func)(&mut line_info); // Call the closure
            println!("Result: {}", result);
        }
    }
}

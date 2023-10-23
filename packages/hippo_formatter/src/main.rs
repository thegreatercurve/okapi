use core::panic;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Doc {
    Text(String),
    Break,
    Group(VecDeque<Self>),
    Nest(i32, Box<Self>),
    Nil,
}

#[derive(PartialEq)]
enum Mode {
    Flat,
    Break,
}

type Node = (i32, Mode, Doc);

struct PrettyPrinter {
    node_stack: VecDeque<Node>,
    max_column_width: i32,
}

const WHITE_SPACE: &str = " ";
const WHITE_SPACE_LEN: i32 = WHITE_SPACE.len() as i32;

const BREAK_TEXT: &str = "\n";

const NEST_TEXT: &str = "   ";
const NEST_TEXT_LEN: i32 = NEST_TEXT.len() as i32;

impl PrettyPrinter {
    fn new(max_column_width: i32) -> Self {
        Self {
            node_stack: VecDeque::new(),
            max_column_width,
        }
    }

    fn format(self, doc: Doc) -> String {
        self.format_imp(doc)
    }

    fn format_imp(mut self, root_doc: Doc) -> String {
        let mut output: String = String::new();

        let mut current_column_count: i32 = 0;

        if let Doc::Group(group) = root_doc {
            self.node_stack = VecDeque::from(vec![(0, Mode::Flat, Doc::Group(group))]);
        }

        while !self.node_stack.is_empty() {
            let (indent, mode, doc) = self.node_stack.pop_front().unwrap();

            match doc {
                Doc::Text(text) => {
                    current_column_count += text.len() as i32;

                    output.push_str(&text);
                }
                Doc::Break => match mode {
                    Mode::Break => {
                        current_column_count += NEST_TEXT_LEN * indent;

                        output.push_str(BREAK_TEXT);

                        for _ in 0..indent {
                            output.push_str(NEST_TEXT);
                        }
                    }
                    Mode::Flat => {
                        current_column_count += WHITE_SPACE_LEN;

                        output.push_str(WHITE_SPACE);
                    }
                },
                Doc::Nest(nest_indent_total, child) => {
                    self.node_stack
                        .push_front((indent + nest_indent_total, mode, *child));
                }
                Doc::Group(doc) => {
                    if self.fits(current_column_count, indent, doc) {
                    } else {
                    };
                }
                Doc::Nil => output.push_str(""),
            }
        }

        output
    }

    fn fits(&mut self, current_column_count: i32, indent: i32, root_doc: VecDeque<Doc>) -> bool {
        // There are problems here.
        let node_stack: VecDeque<Doc> = VecDeque::new();

        for value in root_doc.into_iter().rev() {
            self.node_stack.push_front((indent, Mode::Flat, value));
        }

        let mut remaining_columns_count = self.max_column_width - current_column_count;

        while !node_stack.is_empty() && remaining_columns_count > -1 {
            let (indent, mode, doc) = self.node_stack.pop_front().unwrap();

            match doc {
                Doc::Break => {
                    match mode {
                        Mode::Break => {
                            panic!("Unexpected break");
                        }
                        Mode::Flat => {
                            remaining_columns_count -= WHITE_SPACE_LEN;
                        }
                    };
                }
                Doc::Text(text) => {
                    remaining_columns_count -= text.len() as i32;
                }
                Doc::Nest(nest_indent_total, child) => {
                    self.node_stack
                        .push_front((indent + nest_indent_total, mode, *child));
                }
                Doc::Group(stack) => {
                    for value in stack.into_iter().rev() {
                        self.node_stack.push_front((indent, Mode::Flat, value));
                    }
                }
                Doc::Nil => {}
            }
        }

        remaining_columns_count > -1
    }
}

fn main() {
    let doc = Doc::Group(VecDeque::from(vec![
        Doc::Text("[begin".to_string()),
        Doc::Break,
        Doc::Group(VecDeque::from(vec![
            Doc::Text("[stmt;".to_string()),
            Doc::Break,
            Doc::Text("stmt;".to_string()),
            Doc::Break,
            Doc::Text("stmt;]".to_string()),
        ])),
        Doc::Break,
        Doc::Text("end]".to_string()),
    ]));

    let printer = PrettyPrinter::new(10);

    let formatted_string = printer.format(doc);

    print!("{}", formatted_string);
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{Doc, PrettyPrinter};

    #[test]
    fn it_should_correctly_break_based_on_max_width_1() {
        let doc = Doc::Group(VecDeque::from(vec![
            Doc::Text("[begin".to_string()),
            Doc::Break,
            Doc::Group(VecDeque::from(vec![
                Doc::Text("[stmt;".to_string()),
                Doc::Break,
                Doc::Text("stmt;".to_string()),
                Doc::Break,
                Doc::Text("stmt;]".to_string()),
            ])),
            Doc::Break,
            Doc::Text("end]".to_string()),
        ]));

        assert_eq!(
            PrettyPrinter::new(80).format(doc.clone()),
            "[begin [stmt; stmt; stmt;] end]"
        );
        assert_eq!(
            PrettyPrinter::new(30).format(doc.clone()),
            "[begin
[stmt; stmt; stmt;]
end]"
        );
        assert_eq!(
            PrettyPrinter::new(10).format(doc.clone()),
            "[begin
[stmt;
stmt;
stmt;]
end]"
        );
    }

    #[test]
    fn it_should_correctly_break_based_on_max_width_2() {
        let doc = Doc::Group(VecDeque::from(vec![
            Doc::Text("[begin".to_string()),
            Doc::Break,
            Doc::Group(VecDeque::from(vec![
                Doc::Text("[stmt;".to_string()),
                Doc::Break,
                Doc::Text("stmt;".to_string()),
                Doc::Break,
                Doc::Group(VecDeque::from(vec![
                    Doc::Text("[stmt;".to_string()),
                    Doc::Break,
                    Doc::Text("stmt;".to_string()),
                    Doc::Break,
                    Doc::Text("stmt;]".to_string()),
                ])),
                Doc::Break,
                Doc::Text("stmt;]".to_string()),
            ])),
            Doc::Break,
            Doc::Text("end]".to_string()),
        ]));

        assert_eq!(
            PrettyPrinter::new(80).format(doc.clone()),
            "[begin [stmt; stmt; [stmt; stmt; stmt;] stmt;] end]"
        );
        assert_eq!(
            PrettyPrinter::new(50).format(doc.clone()),
            "[begin\n[stmt; stmt; [stmt; stmt; stmt;] stmt;]\nend]"
        );
        assert_eq!(
            PrettyPrinter::new(40).format(doc.clone()),
            "[begin\n[stmt;\nstmt;\n[stmt; stmt; stmt;]\nstmt;]\nend]"
        );
        assert_eq!(
            PrettyPrinter::new(10).format(doc.clone()),
            "[begin\n[stmt;\nstmt;\n[stmt;\nstmt;\nstmt;]\nstmt;]\nend]"
        );
    }

    #[test]
    fn it_should_correctly_break_based_with_nested_groups() {
        let doc = Doc::Group(VecDeque::from(vec![
            Doc::Text("[begin".to_string()),
            Doc::Break,
            Doc::Nest(
                1,
                Box::from(Doc::Group(VecDeque::from(vec![
                    Doc::Text("[stmt;".to_string()),
                    Doc::Break,
                    Doc::Text("stmt;".to_string()),
                    Doc::Break,
                    Doc::Text("stmt;]".to_string()),
                ]))),
            ),
            Doc::Break,
            Doc::Text("end]".to_string()),
        ]));

        assert_eq!(
            PrettyPrinter::new(80).format(doc.clone()),
            "[begin [stmt; stmt; stmt;] end]"
        );

        assert_eq!(
            PrettyPrinter::new(10).format(doc.clone()),
            "[begin [stmt; stmt; stmt;] end]"
        );
    }
}

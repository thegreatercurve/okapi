use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Doc {
    Line(bool),
    Text(String),
    Group(VecDeque<Doc>),
    Nest(i32, Box<Doc>),
}

struct PrettyPrinter {
    node_stack: VecDeque<Doc>,
    max_column_width: i32,
}

const WHITE_SPACE: &str = " ";
const WHITE_SPACE_LEN: i32 = WHITE_SPACE.len() as i32;

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

    fn format_imp(mut self, doc: Doc) -> String {
        let mut current_column_count: i32 = 0;

        let mut output: String = String::new();

        if let Doc::Group(group) = doc {
            self.node_stack = VecDeque::from(vec![Doc::Group(group)]);
        }

        while !self.node_stack.is_empty() {
            let current = self.node_stack.pop_front().unwrap();

            match current {
                Doc::Line(is_line_break) => {
                    let break_char = if is_line_break { "\n" } else { WHITE_SPACE };

                    output.push_str(break_char);

                    current_column_count += WHITE_SPACE_LEN;
                }
                Doc::Text(text) => {
                    output.push_str(&text);

                    current_column_count += text.len() as i32;
                }
                Doc::Nest(indents_total, child) => {
                    for _ in 0..indents_total {
                        current_column_count += NEST_TEXT_LEN;
                    }

                    self.node_stack.push_front(*child);
                }
                Doc::Group(child) => {
                    let broken_group = if !self.fits(current_column_count, &child) {
                        self.break_all_lines(&child)
                    } else {
                        child
                    };

                    for value in broken_group.into_iter().rev() {
                        self.node_stack.push_front(value);
                    }
                }
            }
        }

        output
    }

    fn fits(&mut self, current_column_count: i32, group: &VecDeque<Doc>) -> bool {
        let mut child_stack = group.clone();

        let mut remaining_columns_count = self.max_column_width - current_column_count;

        while !child_stack.is_empty() && remaining_columns_count > -1 {
            let current = child_stack.pop_front().unwrap();

            match current {
                Doc::Line(_is_line_break) => {
                    remaining_columns_count -= WHITE_SPACE_LEN;
                }
                Doc::Text(text) => {
                    remaining_columns_count -= text.len() as i32;
                }
                Doc::Nest(_, child) => {
                    remaining_columns_count -= NEST_TEXT_LEN;
                }
                Doc::Group(child) => {
                    for value in child.into_iter().rev() {
                        child_stack.push_front(value);
                    }
                }
            }
        }

        remaining_columns_count > -1
    }

    fn break_all_lines(&mut self, group: &VecDeque<Doc>) -> VecDeque<Doc> {
        group
            .iter()
            .map(|value| match value {
                Doc::Line(_is_line_break) => Doc::Line(true),
                _ => value.clone(),
            })
            .collect()
    }
}

fn main() {
    let doc = Doc::Group(VecDeque::from(vec![
        Doc::Text("[begin".to_string()),
        Doc::Line(false),
        Doc::Group(VecDeque::from(vec![
            Doc::Text("[stmt;".to_string()),
            Doc::Line(false),
            Doc::Text("stmt;".to_string()),
            Doc::Line(false),
            Doc::Text("stmt;]".to_string()),
        ])),
        Doc::Line(false),
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
            Doc::Line(false),
            Doc::Group(VecDeque::from(vec![
                Doc::Text("[stmt;".to_string()),
                Doc::Line(false),
                Doc::Text("stmt;".to_string()),
                Doc::Line(false),
                Doc::Text("stmt;]".to_string()),
            ])),
            Doc::Line(false),
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
            Doc::Line(false),
            Doc::Group(VecDeque::from(vec![
                Doc::Text("[stmt;".to_string()),
                Doc::Line(false),
                Doc::Text("stmt;".to_string()),
                Doc::Line(false),
                Doc::Group(VecDeque::from(vec![
                    Doc::Text("[stmt;".to_string()),
                    Doc::Line(false),
                    Doc::Text("stmt;".to_string()),
                    Doc::Line(false),
                    Doc::Text("stmt;]".to_string()),
                ])),
                Doc::Line(false),
                Doc::Text("stmt;]".to_string()),
            ])),
            Doc::Line(false),
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
            Doc::Line(false),
            Doc::Nest(
                1,
                Box::from(Doc::Group(VecDeque::from(vec![
                    Doc::Text("[stmt;".to_string()),
                    Doc::Line(false),
                    Doc::Text("stmt;".to_string()),
                    Doc::Line(false),
                    Doc::Text("stmt;]".to_string()),
                ]))),
            ),
            Doc::Line(false),
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

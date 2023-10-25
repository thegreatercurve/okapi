use core::panic;
use std::collections::VecDeque;

#[derive(Clone)]
enum Doc {
    Text(String),
    Break(String),
    Deque(VecDeque<Self>),
    Group(Box<Self>),
    Nest(i32, Box<Self>),
    Nil,
}

#[derive(Clone, Copy)]
enum Mode {
    Flat,
    Break,
}

type Node = (i32, Mode, Doc);

fn text(text: &str) -> Doc {
    Doc::Text(text.to_string())
}

fn nest(indent: i32, doc: Doc) -> Doc {
    Doc::Nest(indent, Box::from(Doc::Deque(VecDeque::from(vec![doc]))))
}

fn break_with(text: &str) -> Doc {
    Doc::Break(text.to_string())
}

fn break_with_space() -> Doc {
    Doc::Break(" ".to_string())
}

fn group(doc: Doc) -> Doc {
    Doc::Group(Box::from(Doc::Deque(VecDeque::from(vec![doc]))))
}

fn doc_vec_to_doc(doc: Vec<Doc>) -> Doc {
    Doc::Deque(VecDeque::from(doc))
}

struct PrettyPrinter {
    max_column_width: i32,
}

impl PrettyPrinter {
    fn new(max_column_width: i32) -> Self {
        Self { max_column_width }
    }

    fn format(self, doc: Doc) -> String {
        self.format_imp(doc)
    }

    fn format_imp(mut self, root_doc: Doc) -> String {
        let mut output: String = String::new();

        let mut current_column_count: i32 = 0;

        let mut stack: VecDeque<(i32, Mode, Doc)> = VecDeque::new();

        if let Doc::Group(group) = root_doc {
            stack = VecDeque::from(vec![(0, Mode::Flat, Doc::Group(group))]);
        }

        while !stack.is_empty() {
            let (indent, mode, doc) = stack.pop_front().unwrap();

            match doc {
                Doc::Text(text) => {
                    current_column_count += text.len() as i32;

                    output.push_str(&text);
                }
                Doc::Break(text) => match mode {
                    Mode::Break => {
                        let mut break_text = String::from("\n");

                        for _ in 0..indent {
                            break_text.push_str(" ");
                        }

                        output.push_str(&break_text);

                        current_column_count += break_text.len() as i32 * indent;
                    }
                    Mode::Flat => {
                        current_column_count += text.len() as i32;

                        output.push_str(&text);
                    }
                },
                Doc::Nest(nest_indent, child) => {
                    stack.push_front((indent + nest_indent, mode, *child))
                }
                Doc::Deque(doc) => {
                    for value in doc.into_iter().rev() {
                        stack.push_front((indent, mode, value))
                    }
                }
                Doc::Group(doc) => {
                    if self.fits(
                        current_column_count,
                        (indent, Mode::Flat, *doc.clone()),
                        stack.clone(),
                    ) {
                        stack.push_front((indent, Mode::Flat, *doc.clone()));
                    } else {
                        stack.push_front((indent, Mode::Break, *doc.clone()));
                    }
                }
                Doc::Nil => output.push_str(""),
            }
        }

        output
    }

    fn fits(
        &mut self,
        current_column_count: i32,
        first_node: Node,
        node_stack: VecDeque<Node>,
    ) -> bool {
        let mut remaining_column_count: i32 = self.max_column_width - current_column_count;

        let mut stack = VecDeque::from(vec![first_node]);

        stack.extend(node_stack);

        while !stack.is_empty() && remaining_column_count > -1 {
            let (indent, mode, doc) = stack.pop_front().unwrap();

            match doc {
                Doc::Break(text) => {
                    match mode {
                        Mode::Break => remaining_column_count = -1,
                        Mode::Flat => remaining_column_count -= text.len() as i32,
                    };
                }
                Doc::Text(text) => remaining_column_count -= text.len() as i32,
                Doc::Nest(current_indent, doc) => {
                    stack.push_front((indent + current_indent, mode, *doc))
                }
                Doc::Deque(doc) => {
                    for value in doc.into_iter().rev() {
                        stack.push_front((indent, mode, value))
                    }
                }
                Doc::Group(doc) => stack.push_front((indent, Mode::Flat, *doc)),
                Doc::Nil => {}
            }
        }

        remaining_column_count > -1
    }
}

fn main() {
    let doc = group(doc_vec_to_doc(vec![
        text("[begin"),
        nest(
            3,
            group(doc_vec_to_doc(vec![
                break_with_space(),
                text("[stmt;"),
                break_with_space(),
                text("stmt;"),
                break_with_space(),
                text("stmt;]"),
            ])),
        ),
        break_with_space(),
        text("end]"),
    ]));

    let mut printer = PrettyPrinter::new(50);
    print!("{}", printer.format(doc.clone()));
    printer = PrettyPrinter::new(32);
    print!("{}", printer.format(doc.clone()));
    printer = PrettyPrinter::new(10);
    print!("{}", printer.format(doc.clone()));
}

#[cfg(test)]
mod tests {
    use crate::{
        break_with, break_with_space, doc_vec_to_doc, group, nest, text, Doc, PrettyPrinter,
    };

    #[test]
    fn it_should_correctly_break_based_on_max_width_1() {
        let doc = group(doc_vec_to_doc(vec![
            text("[begin"),
            nest(
                3,
                group(doc_vec_to_doc(vec![
                    break_with_space(),
                    text("[stmt;"),
                    break_with_space(),
                    text("stmt;"),
                    break_with_space(),
                    text("stmt;]"),
                ])),
            ),
            break_with_space(),
            text("end]"),
        ]));

        assert_eq!(
            PrettyPrinter::new(50).format(doc.clone()),
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
    fn it_should_correctly_break_based_with_nested_groups() {
        fn binop(left: &str, op: &str, right: &str) -> Doc {
            group(doc_vec_to_doc(vec![nest(
                2,
                doc_vec_to_doc(vec![
                    group(doc_vec_to_doc(vec![
                        text(left),
                        break_with_space(),
                        break_with(op),
                    ])),
                    break_with_space(),
                    text(right),
                ]),
            )]))
        }

        let cond = binop("a", "==", "b");
        let expr_1 = binop("a", "<<", "b");
        let expr_2 = binop("a", "+", "b");

        fn if_then(cond: Doc, expr_1: Doc, expr_2: Doc) -> Doc {
            group(doc_vec_to_doc(vec![group(doc_vec_to_doc(vec![
                nest(
                    2,
                    doc_vec_to_doc(vec![text("if"), break_with_space(), cond]),
                ),
                break_with_space(),
                group(doc_vec_to_doc(vec![nest(
                    2,
                    doc_vec_to_doc(vec![text("then"), break_with_space(), expr_1]),
                )])),
                break_with_space(),
                group(doc_vec_to_doc(vec![nest(
                    2,
                    doc_vec_to_doc(vec![text("else"), break_with_space(), expr_2]),
                )])),
            ]))]))
        }

        let doc = if_then(cond, expr_1, expr_2);

        assert_eq!(PrettyPrinter::new(32).format(doc.clone()), "");

        assert_eq!(PrettyPrinter::new(15).format(doc.clone()), "");

        assert_eq!(PrettyPrinter::new(10).format(doc.clone()), "");
    }
}

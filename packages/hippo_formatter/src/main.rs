pub(crate) enum Doc {
    Line,
    Text(String),
    Group(Box<Vec<Doc>>),
    Nest(i32, Box<Doc>),
    Empty,
}

enum Mode {
    Flat,
    Break,
}

struct Node {
    indent_len: i32,
    child: Doc,
}

struct Formatter {
    max_width: i32,
}

impl Formatter {
    fn new(max_width: i32) -> Self {
        Formatter { max_width }
    }

    fn fits(width: i32, doc: Doc) -> bool {
        unimplemented!()
    }

    fn format(&self, node: Node) {
        let mut stack: Vec<Node> = Vec::new();

        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            if let Doc::Group(child) = current.child {
                continue;
            }
        }
    }
}

fn main() {
    let formatter = Formatter::new(2);

    formatter.format(Node {
        indent_len: 0,
        child: Doc::Group(Box::new(vec![
            Doc::Text("hello".to_string()),
            Doc::Text("world".to_string()),
        ])),
    })
}

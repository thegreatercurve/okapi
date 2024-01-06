use crate::parser::common::assert_parser_eq;

#[test]
fn debugger() {
    assert_parser_eq!(
        "debugger;",
        Program {
            body: vec![ProgramBody::Statement(Statement::Debugger(
                DebuggerStatement {
                    node: Node::new(0, 9)
                }
            ))],
            node: Node::new(0, 9),
            source_type: ProgramSourceTypes::Module
        }
    );
}

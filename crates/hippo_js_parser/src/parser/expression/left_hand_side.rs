use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.3 Left-Hand-Side Expressions
    // https://tc39.es/ecma262/#sec-left-hand-side-expressions

    // https://tc39.es/ecma262/#prod-MemberExpression
    fn parse_member_expression(
        &mut self,
        member_expression: Option<Expression>,
        start_index: usize,
    ) -> Result<Expression, ParserError> {
        let next_member_expression = match member_expression {
            _ if self.token_kind() == TokenKind::Keyword(KeywordKind::New) => {
                // Handle `new MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]`.
                self.advance_any(); // Eat 'new' token.

                let callee_start_index = self.start_node();

                let member_expression = self.parse_member_expression(None, callee_start_index)?;

                let arguments = self.parse_new_expression_arguments()?;

                Expression::New(NewExpression {
                    node: self.end_node(start_index)?,
                    callee: Box::new(member_expression),
                    arguments,
                })
            }
            Some(left_hand_side_expression) => left_hand_side_expression,
            None => self.parse_primary_expression()?,
        };

        match self.parse_member_expression_tail(&next_member_expression, start_index)? {
            Some(member_expression) => {
                // Handle `MemberExpression . IdentifierName`.
                // Handle `MemberExpression . PrivateIdentifier`.
                // Handle `MemberExpression [ Expression ]`.
                // Handle `MemberExpression TemplateLiteral`.
                self.parse_member_expression(Some(member_expression), start_index)
            }
            None => Ok(next_member_expression),
        }
    }

    fn parse_member_expression_tail(
        &mut self,
        member_expression: &Expression,
        start_index: usize,
    ) -> Result<Option<Expression>, ParserError> {
        let mut is_computed = false;

        if self.token_kind().is_template_start() {
            // Handle `MemberExpression TemplateLiteral`.
            return Ok(Some(Expression::TaggedTemplate(
                self.parse_tagged_template(member_expression, start_index)?,
            )));
        }

        let member_expression_property = match self.token_kind() {
            TokenKind::OptionalChaining => {
                // Handle `OptionalExpression > MemberExpression OptionalChain`.
                return Ok(Some(
                    self.parse_optional_chain(&member_expression, start_index)?,
                ));
            }
            TokenKind::Dot => {
                // Handle `MemberExpression . IdentifierName`.
                // Handle `MemberExpression . PrivateIdentifier`.
                Ok(Some(self.parse_static_member_expression(false)?))
            }
            TokenKind::LeftSquareBracket => {
                // Handle `MemberExpression [ Expression ]`.
                is_computed = true;

                Ok(Some(self.parse_computed_member_expression()?))
            }
            _ => Ok(None),
        }?;

        match member_expression_property {
            Some(member_expression_property) => Ok(Some(Expression::Member(MemberExpression {
                node: self.end_node(start_index)?,
                object: Box::new(member_expression.clone()),
                property: member_expression_property,
                computed: is_computed,
                optional: false,
            }))),
            None => Ok(None),
        }
    }

    // https://tc39.es/ecma262/#prod-SuperProperty
    fn parse_super_property(&mut self) -> Result<Expression, ParserError> {
        // TODO Throw error if not in a class.
        let mut is_computed = false;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Super))?;

        let super_identifier = SuperExpression {
            node: self.end_node(start_index)?,
        };

        let property = match self.token_kind() {
            TokenKind::Dot => self.parse_static_member_expression(false)?,
            TokenKind::LeftSquareBracket => {
                is_computed = true;

                self.parse_computed_member_expression()?
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(Expression::Member(MemberExpression {
            node: self.end_node(start_index)?,
            object: Box::new(Expression::Super(super_identifier)),
            property,
            computed: is_computed,
            optional: false,
        }))
    }

    // https://tc39.es/ecma262/#prod-NewTarget
    fn parse_new_target(&mut self) -> Result<Expression, ParserError> {
        // TODO Throw error if not in a function body or class.
        let meta_property_start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::New))?;

        let meta_identifier = Identifier {
            node: self.end_node(meta_property_start_index)?,
            name: String::from("new"),
        };

        self.expect_and_advance(TokenKind::Dot)?;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Target))?;

        let property_identifier = Identifier {
            node: self.end_node(start_index)?,
            name: String::from("target"),
        };

        Ok(Expression::MetaProperty(MetaProperty {
            node: self.end_node(meta_property_start_index)?,
            meta: meta_identifier,
            property: property_identifier,
        }))
    }

    // https://tc39.es/ecma262/#prod-ImportMeta
    fn parse_import_meta(&mut self) -> Result<Expression, ParserError> {
        // TODO Throw error if not in a module.
        let meta_property_start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Import))?;

        let meta_identifier = Identifier {
            node: self.end_node(meta_property_start_index)?,
            name: String::from("import"),
        };

        self.expect_and_advance(TokenKind::Dot)?;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Identifier)?;

        let property_identifier = Identifier {
            node: self.end_node(start_index)?,
            name: String::from("meta"),
        };

        Ok(Expression::MetaProperty(MetaProperty {
            node: self.end_node(meta_property_start_index)?,
            meta: meta_identifier,
            property: property_identifier,
        }))
    }

    // https://tc39.es/ecma262/#prod-LeftHandSideExpression
    pub(crate) fn parse_left_hand_side_expression(&mut self) -> Result<Expression, ParserError> {
        let previous_in_optional_chain = self.context.in_optional_chain;
        self.context.in_optional_chain = false;

        let start_index = self.start_node();

        let mut left_hand_side_expression = match (self.token_kind(), self.peek_token_kind()) {
            (
                TokenKind::Keyword(KeywordKind::Super),
                TokenKind::LeftSquareBracket | TokenKind::Dot,
            ) => {
                // Handle `super . IdentifierName`.
                // Handle `super [ Expression ]`.
                self.parse_super_property()
            }
            (TokenKind::Keyword(KeywordKind::New), TokenKind::Dot) => {
                // Handle `new.target`.
                self.parse_new_target()
            }
            (TokenKind::Keyword(KeywordKind::Import), TokenKind::Dot) => {
                // Handle `import.meta`.
                self.parse_import_meta()
            }
            (TokenKind::Keyword(KeywordKind::Super), TokenKind::LeftParenthesis) => {
                // Handle `super Arguments`.
                self.parse_super_call()
            }
            (TokenKind::Keyword(KeywordKind::Import), TokenKind::LeftParenthesis) => {
                // Handle `import ( AssignmentExpression )`.
                self.parse_import_call()
            }
            (TokenKind::Keyword(KeywordKind::New), _) => self.parse_new_expression(),
            _ => self.parse_call_expression(None),
        }?;

        // Handle `CallExpression Arguments`, which are initially parsed as a `NewExpression` (e.g. `new Foo()()` is a `CallExpression` but `new Foo()` is a `NewExpression`). This is a special case where a `NewExpression` is refined to a `CallExpression` when followed by `Arguments`.
        if self.token_kind() == TokenKind::LeftParenthesis {
            let optional_arguments = self.parse_arguments()?;

            let new_lhs = Expression::Call(CallExpression {
                node: self.end_node(start_index)?,
                callee: CallExpressionCallee::Expression(Box::new(left_hand_side_expression)),
                arguments: optional_arguments,
                optional: self.context.in_optional_chain,
            });

            left_hand_side_expression =
                self.parse_call_expression_tail(Some(new_lhs), start_index)?;
        }

        if self.context.in_optional_chain {
            left_hand_side_expression = Expression::Chain(ChainExpression {
                node: self.end_node(start_index)?,
                expression: ChainElement::try_from(left_hand_side_expression)?,
            });
        }

        self.context.in_optional_chain = previous_in_optional_chain;

        Ok(left_hand_side_expression)
    }

    // https://tc39.es/ecma262/#prod-NewExpression
    fn parse_new_expression(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        self.parse_member_expression(None, start_index)
    }

    fn parse_new_expression_arguments(
        &mut self,
    ) -> Result<Vec<NewExpressionArguments>, ParserError> {
        let mut arguments = vec![];

        if self.token_kind() == TokenKind::LeftParenthesis {
            arguments.extend(self.parse_arguments()?)
        };

        arguments
            .into_iter()
            .map(NewExpressionArguments::try_from)
            .collect::<Result<Vec<NewExpressionArguments>, ParserError>>()
    }

    // https://tc39.es/ecma262/#prod-CallExpression
    fn parse_call_expression(
        &mut self,
        call_expression: Option<Expression>,
    ) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        self.parse_call_expression_tail(call_expression, start_index)
    }

    fn parse_call_expression_tail(
        &mut self,
        call_expression: Option<Expression>,
        start_index: usize,
    ) -> Result<Expression, ParserError> {
        let next_call_expression = match call_expression {
            Some(call_expression) => call_expression,
            None => self.parse_member_expression(None, start_index)?,
        };

        let next_call_expression = if self.token_kind() == TokenKind::LeftParenthesis {
            // When processing an instance of the production `CallExpression : CoverCallExpressionAndAsyncArrowHead` the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:
            // `CallMemberExpression[Yield, Await] : MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]`
            let optional_arguments = self.parse_arguments()?;

            Expression::Call(CallExpression {
                node: self.end_node(start_index)?,
                callee: CallExpressionCallee::Expression(Box::new(next_call_expression)),
                arguments: optional_arguments,
                optional: self.context.in_optional_chain,
            })
        } else {
            next_call_expression
        };

        match self.token_kind() {
            TokenKind::OptionalChaining => {
                // Handle `OptionalExpression > CallExpression OptionalChain`.
                return self.parse_optional_chain(&next_call_expression, start_index);
            }
            TokenKind::LeftParenthesis => {
                // Handle `CallExpression Arguments`.
                self.parse_call_expression_tail(Some(next_call_expression), start_index)
            }
            _ => match self.parse_member_expression_tail(&next_call_expression, start_index)? {
                Some(member_expression) => {
                    // Handle `CallExpression . IdentifierName`.
                    // Handle `CallExpression . PrivateIdentifier`.
                    // Handle `CallExpression [ Expression ]`.
                    // Handle `CallExpression TemplateLiteral`.
                    self.parse_call_expression_tail(Some(member_expression), start_index)
                }
                None => Ok(next_call_expression),
            },
        }
    }

    // https://tc39.es/ecma262/#prod-SuperCall
    fn parse_super_call(&mut self) -> Result<Expression, ParserError> {
        // TODO Throw error if not in a constructor.
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Super))?;

        let super_identifier = SuperExpression {
            node: self.end_node(start_index)?,
        };

        let arguments = self.parse_arguments()?;

        Ok(Expression::Call(CallExpression {
            node: self.end_node(start_index)?,
            callee: CallExpressionCallee::Expression(Box::new(Expression::Super(super_identifier))),
            arguments,
            optional: false,
        }))
    }

    // https://tc39.es/ecma262/#prod-ImportCall
    fn parse_import_call(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Import))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let assignment_expression = self.parse_assignment_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(Expression::Import(ImportExpression {
            node: self.end_node(start_index)?,
            source: Box::new(assignment_expression),
        }))
    }

    // https://tc39.es/ecma262/#prod-Arguments
    // https://tc39.es/ecma262/#prod-ArgumentList
    fn parse_arguments(&mut self) -> Result<Vec<CallExpressionArgument>, ParserError> {
        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let mut arguments_list = vec![];

        while self.token_kind() != TokenKind::RightParenthesis {
            let start_index = self.start_node();

            let is_spread = if self.token_kind() == TokenKind::Ellipsis {
                self.advance_any(); // Eat '...' token.

                true
            } else {
                false
            };

            let argument = self.parse_assignment_expression()?;

            if is_spread {
                arguments_list.push(CallExpressionArgument::SpreadElement(SpreadElement {
                    node: self.end_node(start_index)?,
                    argument,
                }));
            } else {
                arguments_list.push(CallExpressionArgument::Expression(argument));
            }

            if self.token_kind() != TokenKind::Comma {
                break;
            }

            if self.token_kind() == TokenKind::RightParenthesis {
                break;
            }

            self.advance_any();
        }

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(arguments_list)
    }

    // https://tc39.es/ecma262/#prod-OptionalExpression
    // https://tc39.es/ecma262/#prod-OptionalChain
    fn parse_optional_chain(
        &mut self,
        optional_expression: &Expression,
        start_index: usize,
    ) -> Result<Expression, ParserError> {
        // TODO Add error got for template literals.
        let mut is_computed = false;

        let is_current_token_optional = if self.token_kind() == TokenKind::OptionalChaining {
            self.advance_any(); // Eat '?' token.

            self.context.in_optional_chain = true;

            true
        } else {
            false
        };

        if is_current_token_optional {
            let next_optional_expression = if self.token_kind() == TokenKind::LeftParenthesis {
                // Handle `OptionalExpression  Arguments`.
                let arguments = self.parse_arguments()?;

                Expression::Call(CallExpression {
                    node: self.end_node(start_index)?,
                    callee: CallExpressionCallee::Expression(Box::new(optional_expression.clone())),
                    arguments,
                    optional: is_current_token_optional,
                })
            } else {
                let member_expression_property = match self.token_kind() {
                    token_kind
                        if token_kind.is_identifier_name()
                            || token_kind == TokenKind::PrivateIdentifier =>
                    {
                        // Handle `OptionalExpression . IdentifierName`.
                        // Handle `OptionalExpression . PrivateIdentifier`.
                        Ok(Some(self.parse_static_member_expression(true)?))
                    }
                    TokenKind::LeftSquareBracket => {
                        // Handle `OptionalExpression [ Expression ]`.
                        is_computed = true;

                        Ok(Some(self.parse_computed_member_expression()?))
                    }
                    _ => Ok(None),
                }?;

                match member_expression_property {
                    Some(member_expression_property) => Expression::Member(MemberExpression {
                        node: self.end_node(start_index)?,
                        object: Box::new(optional_expression.clone()),
                        property: member_expression_property,
                        computed: is_computed,
                        optional: is_current_token_optional,
                    }),
                    None => optional_expression.clone(),
                }
            };

            self.parse_optional_chain(&next_optional_expression, start_index)
        } else {
            return Ok(optional_expression.clone());
        }
    }

    // Helper function to parse object property access expressions (e.g. `a.b``) .
    fn parse_static_member_expression(
        &mut self,
        is_optional: bool,
    ) -> Result<MemberExpressionProperty, ParserError> {
        if !is_optional {
            self.expect_and_advance(TokenKind::Dot)?;
        }

        match self.token_kind() {
            token_kind if token_kind.is_identifier_name() => {
                Ok(MemberExpressionProperty::Expression(Box::new(
                    Expression::Identifier(self.parse_identifier_name()?),
                )))
            }
            TokenKind::PrivateIdentifier => Ok(MemberExpressionProperty::PrivateIdentifier(
                self.parse_private_identifier()?,
            )),
            _ => return Err(self.unexpected_current_token_kind()),
        }
    }

    // Helper function to parse object property access expressions (e.g. `a.b``) .
    fn parse_computed_member_expression(
        &mut self,
    ) -> Result<MemberExpressionProperty, ParserError> {
        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        // Ensure that parenthezied expression in a `for` statement parses the `in` keyword as a relational expression.
        let previous_allow_in = self.context.allow_in;
        self.context.allow_in = true;

        let computed_expression = self.parse_expression()?;

        self.context.allow_in = previous_allow_in;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(MemberExpressionProperty::Expression(Box::new(
            computed_expression,
        )))
    }

    // Helper function to parse tagged template expressions (e.g. foo`bar`), which is distinct from a regular template literal (e.g. `foo`).
    fn parse_tagged_template(
        &mut self,
        member_expression: &Expression,
        start_index: usize,
    ) -> Result<TaggedTemplateExpression, ParserError> {
        let template_literal = self.parse_template_literal()?;

        Ok(TaggedTemplateExpression {
            node: self.end_node(start_index)?,
            tag: Box::new(member_expression.clone()),
            quasi: template_literal,
        })
    }

    // https://tc39.es/ecma262/#prod-IdentifierName
    fn parse_identifier_name(&mut self) -> Result<Identifier, ParserError> {
        let start_index = self.start_node();

        let identifier_name = self.token_value();

        self.advance_any(); // Eat identifier or reserved keyword token.

        Ok(Identifier {
            node: self.end_node(start_index)?,
            name: String::from(identifier_name),
        })
    }

    // Helper function to parse private identifiers (e.g. `#foo`).
    // https://tc39.es/ecma262/#prod-PrivateIdentifier
    pub(crate) fn parse_private_identifier(&mut self) -> Result<PrivateIdentifier, ParserError> {
        let start_index = self.start_node();
        let token_value = self.token_value();

        self.advance_any(); // Eat private identifier token.

        Ok(PrivateIdentifier {
            node: self.end_node(start_index)?,
            name: String::from(token_value),
        })
    }
}

use crate::renderer::js::token::{JsLexer, Token};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    ExpressionStatement(Option<Rc<Node>>),
    AdditiveExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    AssignmentExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    MemberExpression {
        object: Option<Rc<Node>>,
        property: Option<Rc<Node>>,
    },
    NumericLiteral(u64),
}

impl Node {
    pub fn new_expression_statement(expression: Option<Rc<Node>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ExpressionStatement(expression)))
    }

    pub fn new_addirive_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AdditiveExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_assignment_expression(
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::AssignmentExpression {
            operator,
            left,
            right,
        }))
    }

    pub fn new_member_expression(
        object: Option<Rc<Self>>,
        property: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        Some(Rc::new(Node::MemberExpression { object, property }))
    }

    pub fn new_numeric_literal(value: u64) -> Option<Rc<Self>> {
        Some(Rc::new(Node::NumericLiteral(value)))
    }
}

pub struct JsParser {
    t: Peekable<JsLexer>,
}

impl JsParser {
    pub fn new(t: JsLexer) -> Self {
        Self { t: t.peekable() }
    }

    pub fn parse_ast(&mut self) -> Program {
        let mut program = Program::new();

        let mut body = Vec::new();

        loop {
            let node = self.source_element();

            match node {
                Some(n) => body.push(n),
                None => {
                    program.set_body(body);
                    return program;
                }
            }
        }
    }

    fn source_element(&mut self) -> Option<Rc<Node>> {
        match self.t.peek() {
            Some(t) => t,
            None => return None,
        };

        self.statement()
    }

    fn statement(&mut self) -> Option<Rc<Node>> {
        let node = Node::new_expression_statement(self.assignment_expression());

        if let Some(Token::Punctuator(c)) = self.t.peek() {
            if c == &';' {
                assert!(self.t.next().is_some());
            }
        }

        node
    }

    fn assignment_expression(&mut self) -> Option<Rc<Node>> {
        self.additive_expression()
    }

    fn additive_expression(&mut self) -> Option<Rc<Node>> {
        let left = self.left_hand_side_expression();

        let t = match self.t.peek() {
            Some(token) => token.clone(),
            None => return left,
        };

        match t {
            Token::Punctuator(c) => match c {
                '+' | '-' => {
                    assert!(self.t.next().is_some());
                    Node::new_addirive_expression(c, left, self.assignment_expression())
                }
                _ => left,
            },
            _ => left,
        }
    }

    fn left_hand_side_expression(&mut self) -> Option<Rc<Node>> {
        self.member_expression()
    }

    fn member_expression(&mut self) -> Option<Rc<Node>> {
        self.primary_expression()
    }

    fn primary_expression(&mut self) -> Option<Rc<Node>> {
        let t = match self.t.next() {
            Some(token) => token,
            None => return None,
        };

        match t {
            Token::Number(value) => Node::new_numeric_literal(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    body: Vec<Rc<Node>>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn set_body(&mut self, body: Vec<Rc<Node>>) {
        self.body = body;
    }

    pub fn body(&self) -> &Vec<Rc<Node>> {
        &self.body
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_empty() {
        let input = "".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let expected = Program::new();
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(
            Some(Rc::new(Node::NumericLiteral(42))),
        )));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }

    #[test]
    fn test_add_nums() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let mut expected = Program::new();
        let mut body = Vec::new();
        body.push(Rc::new(Node::ExpressionStatement(
            Some(Rc::new(Node::AdditiveExpression {
                operator: '+',
                left: Some(Rc::new(Node::NumericLiteral(1))),
                right: Some(Rc::new(Node::NumericLiteral(2))),
            }))
        )));
        expected.set_body(body);
        assert_eq!(expected, parser.parse_ast());
    }
}

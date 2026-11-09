// Recursive descent parser for Mini-C statements

use crate::ast::*;
use crate::lexer_manual::Token;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    FailedToFindToken(Token),
    ExpectedTypeToken,
    ExpectedIdentifier,
    UnexpectedToken(Token),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
    ExpectedStatement,
    ExpectedCompoundStatement,
    ExpectedExpressionStatement,
    ExpectedSelectionStatement,
    ExpectedIterationStatement,
    ExpectedJumpStatement,
    ExpectedLabeledStatement,
    ExpectedSemicolon,
    ExpectedLeftBrace,
    ExpectedRightBrace,
    ExpectedLeftParen,
    ExpectedRightParen,
    ExpectedColon,
    ExpectedQuestion,
    ExpectedElse,
    ExpectedWhile,
    ExpectedFor,
    ExpectedReturn,
    ExpectedBreak,
    ExpectedContinue,
    ExpectedGoto,
    ExpectedCase,
    ExpectedDefault,
    ExpectedSwitch,
    ExpectedIf,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
            current: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_token(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<Token, ParseError> {
        match self.advance() {
            Some(token) if token == expected => Ok(token),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(name),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn is_current_token(&self, token: &Token) -> bool {
        self.current_token().map_or(false, |t| t == token)
    }

    fn consume_if(&mut self, token: &Token) -> bool {
        if self.is_current_token(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    // Main parsing entry point
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token() {
            Some(Token::Identifier(_)) => {
                // Could be labeled statement or expression statement
                if let Some(Token::Colon) = self.peek_token(1) {
                    self.parse_labeled_statement()
                } else {
                    self.parse_expression_statement()
                }
            }
            Some(Token::Case) => self.parse_case_statement(),
            Some(Token::Default) => self.parse_default_statement(),
            Some(Token::BraceL) => self.parse_compound_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::Switch) => self.parse_switch_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::Do) => self.parse_do_while_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Goto) => self.parse_goto_statement(),
            Some(Token::Continue) => self.parse_continue_statement(),
            Some(Token::Break) => self.parse_break_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::Semicolon) => self.parse_expression_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // Labeled statements
    fn parse_labeled_statement(&mut self) -> Result<Statement, ParseError> {
        let label = self.expect_identifier()?;
        self.expect_token(Token::Colon)?;
        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::LabeledStatement { label, statement })
    }

    fn parse_case_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Case)?;
        let expression = Box::new(self.parse_expression()?);
        self.expect_token(Token::Colon)?;
        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::CaseStatement { expression, statement })
    }

    fn parse_default_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Default)?;
        self.expect_token(Token::Colon)?;
        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::DefaultStatement { statement })
    }

    // Expression statements
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        if self.consume_if(&Token::Semicolon) {
            return Ok(Statement::ExpressionStatement { expression: None });
        }

        let expression = Some(Box::new(self.parse_expression()?));
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::ExpressionStatement { expression })
    }

    // Compound statements
    fn parse_compound_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::BraceL)?;
        let mut items = Vec::new();

        while !self.consume_if(&Token::BraceR) {
            if self.current_token().is_none() {
                return Err(ParseError::UnexpectedEOF);
            }

            // Try to parse as declaration first, then as statement
            if self.is_declaration_start() {
                items.push(CompoundItem::Declaration(self.parse_declaration()?));
            } else {
                items.push(CompoundItem::Statement(self.parse_statement()?));
            }
        }

        Ok(Statement::CompoundStatement { items })
    }

    fn is_declaration_start(&self) -> bool {
        matches!(
            self.current_token(),
            Some(Token::Int | Token::Float | Token::Char | Token::Void | Token::Struct | Token::Enum | Token::Typedef)
        )
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        // Simplified declaration parsing for now
        let specifiers = self.parse_type_specifiers()?;
        let declarators = self.parse_declarators()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Declaration::VariableDeclaration { specifiers, declarators })
    }

    fn parse_type_specifiers(&mut self) -> Result<Vec<TypeSpecifier>, ParseError> {
        let mut specifiers = Vec::new();
        
        while let Some(token) = self.current_token() {
            match token {
                Token::Int => {
                    self.advance();
                    specifiers.push(TypeSpecifier::Int);
                }
                Token::Float => {
                    self.advance();
                    specifiers.push(TypeSpecifier::Float);
                }
                Token::Char => {
                    self.advance();
                    specifiers.push(TypeSpecifier::Char);
                }
                Token::Void => {
                    self.advance();
                    specifiers.push(TypeSpecifier::Void);
                }
                _ => break,
            }
        }

        if specifiers.is_empty() {
            Err(ParseError::ExpectedTypeToken)
        } else {
            Ok(specifiers)
        }
    }

    fn parse_declarators(&mut self) -> Result<Vec<Declarator>, ParseError> {
        let mut declarators = Vec::new();
        
        loop {
            let declarator = self.parse_declarator()?;
            declarators.push(declarator);
            
            if !self.consume_if(&Token::Comma) {
                break;
            }
        }

        Ok(declarators)
    }

    fn parse_declarator(&mut self) -> Result<Declarator, ParseError> {
        let name = self.expect_identifier()?;
        Ok(Declarator::Identifier(name))
    }

    // Selection statements
    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::If)?;
        self.expect_token(Token::ParenL)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect_token(Token::ParenR)?;
        let then_statement = Box::new(self.parse_statement()?);
        
        let else_statement = if self.consume_if(&Token::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::IfStatement {
            condition,
            then_statement,
            else_statement,
        })
    }

    fn parse_switch_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Switch)?;
        self.expect_token(Token::ParenL)?;
        let expression = Box::new(self.parse_expression()?);
        self.expect_token(Token::ParenR)?;
        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::SwitchStatement { expression, statement })
    }

    // Iteration statements
    fn parse_while_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::While)?;
        self.expect_token(Token::ParenL)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect_token(Token::ParenR)?;
        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::WhileStatement { condition, statement })
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Do)?;
        let statement = Box::new(self.parse_statement()?);
        self.expect_token(Token::While)?;
        self.expect_token(Token::ParenL)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect_token(Token::ParenR)?;
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::DoWhileStatement { statement, condition })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::For)?;
        self.expect_token(Token::ParenL)?;
        
        let init = if self.is_declaration_start() {
            Some(Box::new(Statement::ExpressionStatement { expression: None }))
        } else if self.consume_if(&Token::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_expression_statement()?))
        };

        let condition = if self.consume_if(&Token::Semicolon) {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect_token(Token::Semicolon)?;
            Some(Box::new(expr))
        };

        let increment = if self.consume_if(&Token::ParenR) {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect_token(Token::ParenR)?;
            Some(Box::new(expr))
        };

        let statement = Box::new(self.parse_statement()?);
        Ok(Statement::ForStatement {
            init,
            condition,
            increment,
            statement,
        })
    }

    // Jump statements
    fn parse_goto_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Goto)?;
        let label = self.expect_identifier()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::GotoStatement { label })
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Continue)?;
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::ContinueStatement)
    }

    fn parse_break_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Break)?;
        self.expect_token(Token::Semicolon)?;
        Ok(Statement::BreakStatement)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Return)?;
        
        let expression = if self.consume_if(&Token::Semicolon) {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect_token(Token::Semicolon)?;
            Some(Box::new(expr))
        };

        Ok(Statement::ReturnStatement { expression })
    }

    // Expression parsing (simplified for now)
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_conditional_expression()?;
        
        if let Some(op) = self.parse_assignment_operator() {
            let right = self.parse_assignment_expression()?;
            Ok(Expression::Assignment {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_assignment_operator(&mut self) -> Option<AssignmentOperator> {
        match self.current_token() {
            Some(Token::AssignOp) => {
                self.advance();
                Some(AssignmentOperator::Assign)
            }
            Some(Token::MultiplyAssign) => {
                self.advance();
                Some(AssignmentOperator::MultiplyAssign)
            }
            Some(Token::DivideAssign) => {
                self.advance();
                Some(AssignmentOperator::DivideAssign)
            }
            Some(Token::ModuloAssign) => {
                self.advance();
                Some(AssignmentOperator::ModuloAssign)
            }
            Some(Token::AddAssign) => {
                self.advance();
                Some(AssignmentOperator::AddAssign)
            }
            Some(Token::SubtractAssign) => {
                self.advance();
                Some(AssignmentOperator::SubtractAssign)
            }
            _ => None,
        }
    }

    fn parse_conditional_expression(&mut self) -> Result<Expression, ParseError> {
        let condition = self.parse_logical_or_expression()?;
        
        if self.consume_if(&Token::Question) {
            let then_expr = self.parse_expression()?;
            self.expect_token(Token::Colon)?;
            let else_expr = self.parse_conditional_expression()?;
            Ok(Expression::Conditional {
                condition: Box::new(condition),
                true_expr: Box::new(then_expr),
                false_expr: Box::new(else_expr),
            })
        } else {
            Ok(condition)
        }
    }

    fn parse_logical_or_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_logical_and_expression()?;
        
        while self.consume_if(&Token::OrOp) {
            let right = self.parse_logical_and_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::LogicalOr,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_equality_expression()?;
        
        while self.consume_if(&Token::AndOp) {
            let right = self.parse_equality_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: BinaryOperator::LogicalAnd,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_relational_expression()?;
        
        while let Some(op) = self.parse_equality_operator() {
            let right = self.parse_relational_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_equality_operator(&mut self) -> Option<BinaryOperator> {
        match self.current_token() {
            Some(Token::EqualsOp) => {
                self.advance();
                Some(BinaryOperator::Equal)
            }
            Some(Token::NotEqualsOp) => {
                self.advance();
                Some(BinaryOperator::NotEqual)
            }
            _ => None,
        }
    }

    fn parse_relational_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_additive_expression()?;
        
        while let Some(op) = self.parse_relational_operator() {
            let right = self.parse_additive_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_relational_operator(&mut self) -> Option<BinaryOperator> {
        match self.current_token() {
            Some(Token::LessOp) => {
                self.advance();
                Some(BinaryOperator::Less)
            }
            Some(Token::GreaterOp) => {
                self.advance();
                Some(BinaryOperator::Greater)
            }
            Some(Token::LessEqOp) => {
                self.advance();
                Some(BinaryOperator::LessEqual)
            }
            Some(Token::GreaterEqOp) => {
                self.advance();
                Some(BinaryOperator::GreaterEqual)
            }
            _ => None,
        }
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative_expression()?;
        
        while let Some(op) = self.parse_additive_operator() {
            let right = self.parse_multiplicative_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_additive_operator(&mut self) -> Option<BinaryOperator> {
        match self.current_token() {
            Some(Token::Plus) => {
                self.advance();
                Some(BinaryOperator::Add)
            }
            Some(Token::Minus) => {
                self.advance();
                Some(BinaryOperator::Subtract)
            }
            _ => None,
        }
    }

    fn parse_multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_unary_expression()?;
        
        while let Some(op) = self.parse_multiplicative_operator() {
            let right = self.parse_unary_expression()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_multiplicative_operator(&mut self) -> Option<BinaryOperator> {
        match self.current_token() {
            Some(Token::Star) => {
                self.advance();
                Some(BinaryOperator::Multiply)
            }
            Some(Token::Slash) => {
                self.advance();
                Some(BinaryOperator::Divide)
            }
            Some(Token::Percent) => {
                self.advance();
                Some(BinaryOperator::Modulo)
            }
            _ => None,
        }
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Plus) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::Plus,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Minus) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Not) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::LogicalNot,
                    operand: Box::new(operand),
                })
            }
            Some(Token::BitwiseNot) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::BitwiseNot,
                    operand: Box::new(operand),
                })
            }
            Some(Token::AddressOf) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::AddressOf,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Dereference) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::Unary {
                    operator: UnaryOperator::Dereference,
                    operand: Box::new(operand),
                })
            }
            Some(Token::PreIncrement) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::PreIncrement(Box::new(operand)))
            }
            Some(Token::PreDecrement) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expression::PreDecrement(Box::new(operand)))
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary_expression()?;
        
        loop {
            match self.current_token() {
                Some(Token::ParenL) => {
                    self.advance();
                    let mut arguments = Vec::new();
                    
                    if !self.consume_if(&Token::ParenR) {
                        arguments.push(self.parse_expression()?);
                        while self.consume_if(&Token::Comma) {
                            arguments.push(self.parse_expression()?);
                        }
                        self.expect_token(Token::ParenR)?;
                    }
                    
                    expr = Expression::FunctionCall {
                        function: Box::new(expr),
                        arguments,
                    };
                }
                Some(Token::BracketL) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect_token(Token::BracketR)?;
                    expr = Expression::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Some(Token::Dot) => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    expr = Expression::StructMember {
                        object: Box::new(expr),
                        member,
                    };
                }
                Some(Token::Arrow) => {
                    self.advance();
                    let member = self.expect_identifier()?;
                    expr = Expression::PointerMember {
                        pointer: Box::new(expr),
                        member,
                    };
                }
                Some(Token::PostIncrement) => {
                    self.advance();
                    expr = Expression::PostIncrement(Box::new(expr));
                }
                Some(Token::PostDecrement) => {
                    self.advance();
                    expr = Expression::PostDecrement(Box::new(expr));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Some(Token::Identifier(name)) => {
                Ok(Expression::Identifier(name))
            }
            Some(Token::IntLit(value)) => {
                Ok(Expression::Constant(Constant::Integer(value)))
            }
            Some(Token::FloatLit(value)) => {
                Ok(Expression::Constant(Constant::Float(value)))
            }
            Some(Token::StringLit(value)) => {
                Ok(Expression::StringLiteral(value))
            }
            Some(Token::BoolLit(value)) => {
                Ok(Expression::Constant(Constant::Boolean(value)))
            }
            Some(Token::ParenL) => {
                let expr = self.parse_expression()?;
                self.expect_token(Token::ParenR)?;
                Ok(Expression::Parenthesized(Box::new(expr)))
            }
            _ => Err(ParseError::ExpectedExpr),
        }
    }
}
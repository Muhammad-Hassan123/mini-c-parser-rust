// Abstract Syntax Tree definitions for Mini-C

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // Labeled statements
    LabeledStatement {
        label: String,
        statement: Box<Statement>,
    },
    CaseStatement {
        expression: Box<Expression>,
        statement: Box<Statement>,
    },
    DefaultStatement {
        statement: Box<Statement>,
    },
    
    // Expression statement
    ExpressionStatement {
        expression: Option<Box<Expression>>,
    },
    
    // Compound statement
    CompoundStatement {
        items: Vec<CompoundItem>,
    },
    
    // Selection statements
    IfStatement {
        condition: Box<Expression>,
        then_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    SwitchStatement {
        expression: Box<Expression>,
        statement: Box<Statement>,
    },
    
    // Iteration statements
    WhileStatement {
        condition: Box<Expression>,
        statement: Box<Statement>,
    },
    DoWhileStatement {
        statement: Box<Statement>,
        condition: Box<Expression>,
    },
    ForStatement {
        init: Option<Box<Statement>>,
        condition: Option<Box<Expression>>,
        increment: Option<Box<Expression>>,
        statement: Box<Statement>,
    },
    
    // Jump statements
    GotoStatement {
        label: String,
    },
    ContinueStatement,
    BreakStatement,
    ReturnStatement {
        expression: Option<Box<Expression>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundItem {
    Declaration(Declaration),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration {
        specifiers: Vec<TypeSpecifier>,
        declarators: Vec<Declarator>,
    },
    FunctionDeclaration {
        specifiers: Vec<TypeSpecifier>,
        declarator: Declarator,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    Int,
    Float,
    Char,
    Void,
    Short,
    Long,
    Signed,
    Unsigned,
    Struct,
    Enum,
    Typedef,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declarator {
    Identifier(String),
    Pointer(Box<Declarator>),
    Array(Box<Declarator>, Option<Box<Expression>>),
    Function(Box<Declarator>, Vec<Parameter>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub specifiers: Vec<TypeSpecifier>,
    pub declarator: Option<Declarator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Primary expressions
    Identifier(String),
    Constant(Constant),
    StringLiteral(String),
    Parenthesized(Box<Expression>),
    
    // Postfix expressions
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StructMember {
        object: Box<Expression>,
        member: String,
    },
    PointerMember {
        pointer: Box<Expression>,
        member: String,
    },
    PostIncrement(Box<Expression>),
    PostDecrement(Box<Expression>),
    
    // Unary expressions
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    PreIncrement(Box<Expression>),
    PreDecrement(Box<Expression>),
    
    // Binary expressions
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    
    // Assignment expressions
    Assignment {
        left: Box<Expression>,
        operator: AssignmentOperator,
        right: Box<Expression>,
    },
    
    // Conditional expression
    Conditional {
        condition: Box<Expression>,
        true_expr: Box<Expression>,
        false_expr: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    AddressOf,
    Dereference,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Relational
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    
    // Logical
    LogicalAnd,
    LogicalOr,
    
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    LeftShiftAssign,
    RightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
}
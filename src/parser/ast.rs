pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: Identifier,
        initial: Expression,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>,
    },
    Else {
        then: Option<Block>,
    },
    Fn {
        name: Identifier,
        params: Vec<Parameter>,
        body: Block,
    },
    Loop {
        iterable: Option<Expression>,
        value: Option<Identifier>,
        then: Block,
    },
    Expr {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(Identifier),
    Assign(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Prefix(Op, Box<Expression>),
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Bang,
    Equals,
    NotEquals,
    Assign,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    And,
    Or,
}
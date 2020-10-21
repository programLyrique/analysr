

pub enum Prog {
    Expr,
    Assign(Expr, Expr)
}
// Expr list

// Assign

pub enum Expr {
    Float(f64),
    Int(i32),
    Str(&str),
    Symbol(&str),
    Null,
    BinOp(Box<Expr>, Opcode, Box<Expr>),
    UnOp(Opcode, Box<Expr>),
    ExprList(Vec<Expr>),
    Program(Box<Prog>),
    SubList(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    For(Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>,),
    Repeat(Box<Expr>),
    SubScript(Box<Expr>, SubList)
}

pub enum Opcode {
    // Arithmetic
    Mul,
    Div,
    Add,
    Sub,
    Seq, // :
    Power, // ^ 
    Mod, // %% (SPECIAL)
    // Help operator
    Help, // ?
    // Logical operator
    GT,// >
    GE,// >=
    LT,// <
    LE,// <=
    EQ, // ==
    NE, // !=
    AND, // &
    OR, // |
    AND2, // &&
    OR2, // ||
    NOT, // !

    // Slot access
    SLOT // $, @
}

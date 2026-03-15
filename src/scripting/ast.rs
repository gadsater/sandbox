/// Top-level program: a list of statements.
#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// `let name = expr` or `let name :: type = expr`
    Let {
        name: String,
        ty: Option<TypeAnnotation>,
        value: Expr,
    },
    /// `def Name :: tup (fields...)`
    DefTuple {
        name: String,
        fields: Vec<(String, TypeAnnotation)>,
    },
    /// `let methodName on TypeName :: fun (...) = body`
    MethodDef {
        method_name: String,
        on_type: String,
        params: Vec<(String, TypeAnnotation)>,
        ret_ty: Option<TypeAnnotation>,
        body: Vec<Stmt>,
    },
    /// `ret expr`
    Return(Expr),
    /// Bare expression statement (e.g. function call).
    ExprStmt(Expr),
    /// `name op= expr`  (e.g. `self.x += x.x`)
    AssignOp {
        target: Expr,
        op: BinOp,
        value: Expr,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal.
    IntLit(i64),
    /// Float literal.
    FloatLit(f64),
    /// String literal.
    StringLit(String),
    /// Boolean literal.
    BoolLit(bool),
    /// Variable / identifier reference.
    Ident(String),
    /// Binary operation.
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    /// Unary negation.
    Neg(Box<Expr>),
    /// Function call: `callee(args...)`
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    /// Method call: `expr.method(args...)`
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    /// Field access: `expr.field`
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    /// Anonymous function literal: `fun (params) retTy = body`
    FunLit {
        params: Vec<(String, TypeAnnotation)>,
        ret_ty: Option<TypeAnnotation>,
        body: Vec<Stmt>,
    },
    /// Tuple/struct construction: `TypeName { fields... }`
    TupleConstruct {
        type_name: String,
        fields: Vec<FieldInit>,
    },
    /// `self` keyword.
    SelfRef,
}

#[derive(Debug, Clone)]
pub enum FieldInit {
    /// Named: `field: expr`
    Named(String, Expr),
    /// Positional: just an expression.
    Positional(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

/// Minimal type annotation for the scripting language.
#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Simple(String),              // int, float, string, bool, or user-defined
    Fun(Vec<TypeAnnotation>, Option<Box<TypeAnnotation>>), // fun params -> ret
}

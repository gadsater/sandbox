use std::collections::HashMap;
use std::fmt;

use super::ast::*;

/// Runtime value.
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    /// A tuple/struct instance: type name + ordered fields.
    Tuple(String, Vec<(String, Value)>),
    /// A function closure.
    Fun(FunValue),
    Nil,
}

#[derive(Debug, Clone)]
pub struct FunValue {
    pub params: Vec<(String, TypeAnnotation)>,
    pub body: Vec<Stmt>,
    /// Captured environment at definition time.
    pub closure: HashMap<String, Value>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::Tuple(name, fields) => {
                write!(f, "{name} {{ ")?;
                for (i, (fname, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{fname}: {val}")?;
                }
                write!(f, " }}")
            }
            Value::Fun(_) => write!(f, "<function>"),
        }
    }
}

/// The interpreter state.
pub struct Interpreter {
    /// Variable scopes (stack of hashmaps).
    scopes: Vec<HashMap<String, Value>>,
    /// Registered tuple/struct type definitions: name → field names.
    types: HashMap<String, Vec<String>>,
    /// Method table: (type_name, method_name) → FunValue.
    methods: HashMap<(String, String), FunValue>,
    /// Output buffer (for `println` etc.)
    pub output: String,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Self {
            scopes: vec![HashMap::new()],
            types: HashMap::new(),
            methods: HashMap::new(),
            output: String::new(),
        };
        // Pre-register builtins in the global scope.
        interp.scopes[0].insert("println".into(), Value::Nil); // handled specially
        interp
    }

    pub fn exec_program(&mut self, prog: &Program) -> Result<Value, String> {
        let mut last = Value::Nil;
        for stmt in &prog.stmts {
            match self.exec_stmt(stmt) {
                Ok(v) => last = v,
                Err(e) => return Err(e),
            }
        }
        // Trim trailing newline from output for cleaner test comparisons.
        if !self.output.is_empty() {
            Ok(Value::Str(self.output.trim_end().to_string()))
        } else {
            Ok(last)
        }
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.set_var(name.clone(), val.clone());
                Ok(val)
            }
            Stmt::DefTuple { name, fields } => {
                let field_names: Vec<String> = fields.iter().map(|(n, _)| n.clone()).collect();
                self.types.insert(name.clone(), field_names);
                Ok(Value::Nil)
            }
            Stmt::MethodDef {
                method_name,
                on_type,
                params,
                body,
                ..
            } => {
                let fun = FunValue {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.current_env(),
                };
                self.methods
                    .insert((on_type.clone(), method_name.clone()), fun);
                Ok(Value::Nil)
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Err(format!("__return__:{}", self.value_to_transport(&val)))
            }
            Stmt::ExprStmt(expr) => self.eval_expr(expr),
            Stmt::AssignOp { target, op, value } => {
                let rhs = self.eval_expr(value)?;
                self.assign_op(target, *op, rhs)
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::IntLit(n) => Ok(Value::Int(*n)),
            Expr::FloatLit(f) => Ok(Value::Float(*f)),
            Expr::StringLit(s) => Ok(Value::Str(s.clone())),
            Expr::BoolLit(b) => Ok(Value::Bool(*b)),
            Expr::Ident(name) => self
                .get_var(name)
                .ok_or_else(|| format!("undefined variable: {name}")),
            Expr::SelfRef => self
                .get_var("self")
                .ok_or_else(|| "self used outside method".into()),
            Expr::Neg(inner) => {
                let val = self.eval_expr(inner)?;
                match val {
                    Value::Int(n) => Ok(Value::Int(-n)),
                    Value::Float(f) => Ok(Value::Float(-f)),
                    _ => Err("cannot negate non-numeric value".into()),
                }
            }
            Expr::BinOp { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binop(&l, *op, &r)
            }
            Expr::Call { callee, args } => {
                // Check for builtin println
                if let Expr::Ident(name) = callee.as_ref() {
                    if name == "println" {
                        let mut parts = Vec::new();
                        for a in args {
                            parts.push(format!("{}", self.eval_expr(a)?));
                        }
                        let line = parts.join(" ");
                        self.output.push_str(&line);
                        self.output.push('\n');
                        return Ok(Value::Nil);
                    }
                }
                let callee_val = self.eval_expr(callee)?;
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(a)?);
                }
                self.call_fun(&callee_val, arg_vals)
            }
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let obj = self.eval_expr(object)?;
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(a)?);
                }
                self.call_method(obj, method, arg_vals)
            }
            Expr::FieldAccess { object, field } => {
                let obj = self.eval_expr(object)?;
                match &obj {
                    Value::Tuple(_, fields) => {
                        for (name, val) in fields {
                            if name == field {
                                return Ok(val.clone());
                            }
                        }
                        Err(format!("no field '{field}' on tuple"))
                    }
                    _ => Err(format!("cannot access field on {obj}")),
                }
            }
            Expr::FunLit { params, body, .. } => Ok(Value::Fun(FunValue {
                params: params.clone(),
                body: body.clone(),
                closure: self.current_env(),
            })),
            Expr::TupleConstruct { type_name, fields } => {
                let field_names = self
                    .types
                    .get(type_name)
                    .cloned()
                    .ok_or_else(|| format!("undefined type: {type_name}"))?;
                let mut result_fields = Vec::new();
                for (i, fi) in fields.iter().enumerate() {
                    match fi {
                        FieldInit::Named(name, expr) => {
                            let val = self.eval_expr(expr)?;
                            result_fields.push((name.clone(), val));
                        }
                        FieldInit::Positional(expr) => {
                            let val = self.eval_expr(expr)?;
                            let name = field_names.get(i).cloned().unwrap_or_else(|| {
                                format!("_{i}")
                            });
                            result_fields.push((name, val));
                        }
                    }
                }
                Ok(Value::Tuple(type_name.clone(), result_fields))
            }
        }
    }

    fn eval_binop(&self, l: &Value, op: BinOp, r: &Value) -> Result<Value, String> {
        match (l, r) {
            (Value::Int(a), Value::Int(b)) => match op {
                BinOp::Add => Ok(Value::Int(a + b)),
                BinOp::Sub => Ok(Value::Int(a - b)),
                BinOp::Mul => Ok(Value::Int(a * b)),
                BinOp::Div => {
                    if *b == 0 {
                        return Err("division by zero".into());
                    }
                    Ok(Value::Int(a / b))
                }
                BinOp::Mod => Ok(Value::Int(a % b)),
                BinOp::Eq => Ok(Value::Bool(a == b)),
                BinOp::Neq => Ok(Value::Bool(a != b)),
                BinOp::Lt => Ok(Value::Bool(a < b)),
                BinOp::Gt => Ok(Value::Bool(a > b)),
                BinOp::Le => Ok(Value::Bool(a <= b)),
                BinOp::Ge => Ok(Value::Bool(a >= b)),
                _ => Err("invalid op for int".into()),
            },
            (Value::Float(a), Value::Float(b)) => match op {
                BinOp::Add => Ok(Value::Float(a + b)),
                BinOp::Sub => Ok(Value::Float(a - b)),
                BinOp::Mul => Ok(Value::Float(a * b)),
                BinOp::Div => Ok(Value::Float(a / b)),
                _ => Err("invalid op for float".into()),
            },
            (Value::Str(a), Value::Str(b)) if op == BinOp::Add => {
                Ok(Value::Str(format!("{a}{b}")))
            }
            (Value::Bool(a), Value::Bool(b)) => match op {
                BinOp::And => Ok(Value::Bool(*a && *b)),
                BinOp::Or => Ok(Value::Bool(*a || *b)),
                BinOp::Eq => Ok(Value::Bool(a == b)),
                _ => Err("invalid op for bool".into()),
            },
            _ => Err(format!("type mismatch in binary op: {l} {op:?} {r}")),
        }
    }

    fn call_fun(&mut self, callee: &Value, args: Vec<Value>) -> Result<Value, String> {
        let fun = match callee {
            Value::Fun(f) => f.clone(),
            _ => return Err(format!("not callable: {callee}")),
        };
        self.push_scope();
        // Restore closure env.
        for (k, v) in &fun.closure {
            self.set_var(k.clone(), v.clone());
        }
        // Bind params.
        for (i, (pname, _)) in fun.params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Nil);
            self.set_var(pname.clone(), val);
        }
        let result = self.exec_body(&fun.body);
        self.pop_scope();
        result
    }

    fn call_method(
        &mut self,
        mut obj: Value,
        method: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let type_name = match &obj {
            Value::Tuple(name, _) => name.clone(),
            _ => return Err(format!("cannot call method on {obj}")),
        };
        let fun = self
            .methods
            .get(&(type_name, method.to_string()))
            .cloned()
            .ok_or_else(|| format!("undefined method: {method}"))?;

        self.push_scope();
        for (k, v) in &fun.closure {
            self.set_var(k.clone(), v.clone());
        }
        self.set_var("self".into(), obj.clone());
        for (i, (pname, _)) in fun.params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Nil);
            self.set_var(pname.clone(), val);
        }
        let result = self.exec_body(&fun.body);
        // Retrieve potentially mutated self.
        if let Some(new_self) = self.get_var("self") {
            obj = new_self;
        }
        self.pop_scope();

        // Propagate mutation back to the caller's variable if possible.
        // (Simplified: we update the variable in the parent scope.)
        // For now, store the mutated value back.
        // This is a simplification; a real implementation would use references.
        self.update_last_tuple(&obj);

        result
    }

    fn exec_body(&mut self, stmts: &[Stmt]) -> Result<Value, String> {
        let mut last = Value::Nil;
        for stmt in stmts {
            match self.exec_stmt(stmt) {
                Ok(v) => last = v,
                Err(e) => {
                    // Check for return sentinel.
                    if let Some(val_str) = e.strip_prefix("__return__:") {
                        return Ok(self.transport_to_value(val_str));
                    }
                    return Err(e);
                }
            }
        }
        Ok(last)
    }

    fn assign_op(&mut self, target: &Expr, op: BinOp, rhs: Value) -> Result<Value, String> {
        match target {
            Expr::FieldAccess { object, field } => {
                if let Expr::SelfRef = object.as_ref() {
                    let self_val = self
                        .get_var("self")
                        .ok_or("self not in scope")?;
                    if let Value::Tuple(name, mut fields) = self_val {
                        for (fname, fval) in &mut fields {
                            if fname == field {
                                *fval = self.eval_binop(fval, op, &rhs)?;
                                let updated = Value::Tuple(name, fields);
                                self.set_var("self".into(), updated.clone());
                                return Ok(updated);
                            }
                        }
                        Err(format!("no field '{field}' on self"))
                    } else {
                        Err("self is not a tuple".into())
                    }
                } else {
                    Err("compound assignment only supported on self fields for now".into())
                }
            }
            Expr::Ident(name) => {
                let current = self.get_var(name).ok_or_else(|| {
                    format!("undefined variable: {name}")
                })?;
                let new_val = self.eval_binop(&current, op, &rhs)?;
                self.set_var(name.clone(), new_val.clone());
                Ok(new_val)
            }
            _ => Err("invalid assignment target".into()),
        }
    }

    // ----- scope management -----

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn set_var(&mut self, name: String, val: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, val);
        }
    }

    fn get_var(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    fn current_env(&self) -> HashMap<String, Value> {
        let mut env = HashMap::new();
        for scope in &self.scopes {
            for (k, v) in scope {
                env.insert(k.clone(), v.clone());
            }
        }
        env
    }

    /// After a method call mutates `self`, try to update the variable in the parent scope.
    fn update_last_tuple(&mut self, val: &Value) {
        if let Value::Tuple(type_name, _) = val {
            // Walk scopes to find a variable of the same tuple type.
            for scope in self.scopes.iter_mut().rev() {
                for (_name, v) in scope.iter_mut() {
                    if let Value::Tuple(tn, _) = v {
                        if tn == type_name {
                            *v = val.clone();
                            return;
                        }
                    }
                }
            }
        }
    }

    // ----- serialisation helpers for return sentinel -----

    fn value_to_transport(&self, val: &Value) -> String {
        match val {
            Value::Int(n) => format!("int:{n}"),
            Value::Float(f) => format!("float:{f}"),
            Value::Str(s) => format!("str:{s}"),
            Value::Bool(b) => format!("bool:{b}"),
            Value::Nil => "nil".into(),
            Value::Tuple(name, fields) => {
                let fs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{k}={}", self.value_to_transport(v)))
                    .collect();
                format!("tuple:{name}:{}", fs.join(","))
            }
            Value::Fun(_) => "fun".into(),
        }
    }

    fn transport_to_value(&self, s: &str) -> Value {
        if let Some(rest) = s.strip_prefix("int:") {
            Value::Int(rest.parse().unwrap_or(0))
        } else if let Some(rest) = s.strip_prefix("float:") {
            Value::Float(rest.parse().unwrap_or(0.0))
        } else if let Some(rest) = s.strip_prefix("str:") {
            Value::Str(rest.to_string())
        } else if let Some(rest) = s.strip_prefix("bool:") {
            Value::Bool(rest == "true")
        } else if s.starts_with("tuple:") {
            // Simplified: reconstruct from transport format.
            let parts: Vec<&str> = s.splitn(3, ':').collect();
            let name = parts.get(1).unwrap_or(&"").to_string();
            let mut fields = Vec::new();
            if let Some(field_str) = parts.get(2) {
                for pair in field_str.split(',') {
                    if let Some((k, v)) = pair.split_once('=') {
                        fields.push((k.to_string(), self.transport_to_value(v)));
                    }
                }
            }
            Value::Tuple(name, fields)
        } else {
            Value::Nil
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::eval;

    #[test]
    fn eval_simple_let() {
        let result = eval("let x = 3").unwrap();
        assert_eq!(format!("{result}"), "3");
    }

    #[test]
    fn eval_arithmetic() {
        let result = eval("let x = 2 + 3 * 4").unwrap();
        assert_eq!(format!("{result}"), "14");
    }

    #[test]
    fn eval_println() {
        let result = eval("println(42)").unwrap();
        assert_eq!(format!("{result}"), "42");
    }

    #[test]
    fn eval_tuple_construct() {
        let src = "def Point :: tup (x int, y int, z int)\nlet p = Point { 1, 2, 3 }\nprintln(p)";
        let result = eval(src).unwrap();
        assert!(format!("{result}").contains("Point"));
    }
}

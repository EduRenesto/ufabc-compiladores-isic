use std::{collections::HashMap, io::{Write, BufRead}, fmt::Display};

use isic_front::{ast::{IsiProgram, Ident, BinaryOp, Expr}, visitor::IsiVisitor};
use isic_middle::IsiType;

#[derive(Clone)]
pub enum IsiValue {
    Int(u64),
    Float(f32),
    String(String),
    Bool(bool),
    Unit,
}

impl Display for IsiValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsiValue::Int(i) => write!(f, "{}", i),
            IsiValue::Float(x) => write!(f, "{}", x),
            IsiValue::String(s) => write!(f, "{}", s),
            IsiValue::Bool(b) => write!(f, "{}", b),
            IsiValue::Unit => write!(f, ""),
        }
    }
}

pub struct IsiInterpreter<'a, R: BufRead, W: Write> {
    program: &'a IsiProgram,
    sym_table: HashMap<Ident, IsiValue>,
    sym_types: HashMap<Ident, IsiType>, // apenas pra scan...
    stdin: &'a mut R,
    stdout: &'a mut W,
}

impl<'a, R: BufRead, W: Write> IsiInterpreter<'a, R, W> {
    pub fn new(program: &'a IsiProgram, stdin: &'a mut R, stdout: &'a mut W) -> Self {
        IsiInterpreter {
            program,
            sym_table: HashMap::new(),
            sym_types: HashMap::new(),
            stdin,
            stdout,
        }
    }

    pub fn exec(&mut self) {
        self.visit_program(&self.program);
    }
}

impl<'a, R: BufRead, W: Write> IsiVisitor for IsiInterpreter<'a, R, W> {
    type Ret = IsiValue;

    fn visit_int_literal(&mut self, lit: &isic_front::ast::IntLiteral) -> Self::Ret {
        IsiValue::Int(lit.0)
    }

    fn visit_float_literal(&mut self, lit: &isic_front::ast::FloatLiteral) -> Self::Ret {
        IsiValue::Float(lit.0)
    }

    fn visit_string_literal(&mut self, lit: &isic_front::ast::StringLiteral) -> Self::Ret {
        IsiValue::String(lit.0.clone())
    }

    fn visit_ident(&mut self, id: &Ident) -> Self::Ret {
        self.sym_table.get(id).unwrap().clone()
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Self::Ret {
        // Replicando mecanismo do type checker, nao gotsei...
        // Mas aqui tambem precisamos dessa info pra poder
        // parsear as entradas do usuario.
        let ty = match decl.var_type.name.as_str() {
            "int" => IsiType::Int,
            "float" => IsiType::Float,
            "string" => IsiType::String,
            "bool" => IsiType::Bool,
            _ => unreachable!()
        };

        self.sym_types.insert(decl.var_name.clone(), ty);

        IsiValue::Unit
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Self::Ret {
        let lhs = self.visit_expr(&bexpr.1);
        let rhs = self.visit_expr(&bexpr.2);

        match bexpr.0 {
            BinaryOp::Add => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Int(l + r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Float(l + r),
                    (IsiValue::String(l), IsiValue::String(r)) => IsiValue::String(format!("{}{}", l, r)),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Sub => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Int(l - r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Float(l - r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Mul => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Int(l * r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Float(l * r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Div => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Int(l / r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Float(l / r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Gt => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l > r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l > r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Lt => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l < r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l < r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Geq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l >= r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l >= r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Leq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l <= r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l <= r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Eq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l == r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l == r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
            BinaryOp::Neq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => IsiValue::Bool(l != r),
                    (IsiValue::Float(l), IsiValue::Float(r)) => IsiValue::Bool(l != r),
                    _ => unreachable!() // porque a AST foi type-checked antes de chegar aqui
                }
            },
        }
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) -> Self::Ret {
        match call.fname.name.as_str() {
            "escreva" => {
                let val = self.visit_expr(&call.args[0]);

                writeln!(self.stdout, "{}", val).unwrap();

                self.stdout.flush().unwrap();
            },
            "leia" => {
                let mut input = String::new();
                self.stdin.read_line(&mut input).unwrap();

                if let Expr::Ident(ref id) = call.args[0] {
                    let ty = self.sym_types.get(id).unwrap();

                    let val = match ty {
                        IsiType::Int => IsiValue::Int(input.trim().parse::<u64>().unwrap()),
                        IsiType::Float => IsiValue::Float(input.trim().parse::<f32>().unwrap()),
                        IsiType::String => IsiValue::String(input.trim().to_string()),
                        IsiType::Bool => IsiValue::Bool(input.trim().parse::<bool>().unwrap()),
                        IsiType::Unit => IsiValue::Unit,
                    };

                    self.sym_table.insert(id.clone(), val);
                }
            },
            _ => unreachable!()
        }

        IsiValue::Unit
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Self::Ret {
        let val = self.visit_expr(&assignment.val);

        self.sym_table.insert(assignment.ident.clone(), val);

        IsiValue::Unit
    }

    fn visit_conditional(&mut self, conditional: &isic_front::ast::Conditional) -> Self::Ret {
        let should_take = self.visit_expr(&conditional.cond);

        match should_take {
            IsiValue::Bool(true) => {
                for stmt in &conditional.taken {
                    self.visit_statement(stmt);
                }
            }
            _ => {
                for stmt in &conditional.not_taken {
                    self.visit_statement(stmt);
                }
            }
        }

        IsiValue::Unit
    }

    fn visit_while_loop(&mut self, while_loop: &isic_front::ast::WhileLoop) -> Self::Ret {
        while let IsiValue::Bool(true) = self.visit_expr(&while_loop.cond) {
            for stmt in &while_loop.body {
                self.visit_statement(stmt);
            }
        }

        IsiValue::Unit
    }

    fn visit_do_while_loop(&mut self, do_while_loop: &isic_front::ast::DoWhileLoop) -> Self::Ret {
        loop {
            for stmt in &do_while_loop.body {
                self.visit_statement(stmt);
            }

            if !matches!(self.visit_expr(&do_while_loop.cond), IsiValue::Bool(true)) {
                break;
            }
        }

        IsiValue::Unit
    }
}

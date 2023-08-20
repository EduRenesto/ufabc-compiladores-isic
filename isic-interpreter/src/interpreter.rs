use std::{collections::HashMap, io::{Write, BufRead}, fmt::Display};

use isic_front::{ast::{IsiProgram, Ident, BinaryOp, Expr}, visitor::IsiVisitor};
use isic_middle::IsiType;

#[derive(Clone, Debug)]
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

/// O interpretador da IsiLanguage.
///
/// Ele é implementado como um IsiVisitor, e cada função visitadora
/// tenta avaliar qual o valor do nó atual, retornando Ok(valor) se
/// o valor foi computado com sucesso, ou Err(e) se houve algum problema.
///
/// **Importante.** O interpretador **assume que a AST já foi validada,
/// e não há erros de tipos**. Se uma AST inválida for passada para o
/// interpretador, o comportamento é não definido. Provavelmente ocorrerá
/// um panic. Portanto, sempre valide a AST antes de usar o interpretador.
pub struct IsiInterpreter<'a, R: BufRead, W: Write> {
    /// Referencia ao programa a ser interpretado.
    program: &'a IsiProgram,
    /// Tabela de valores das variáveis do programa.
    sym_table: HashMap<Ident, IsiValue>,
    /// Tabela de tipos das variáveis do programa.
    sym_types: HashMap<Ident, IsiType>, // apenas pra scan...
    /// Referência ao stdin.
    stdin: &'a mut R,
    /// Referência ao stdout.
    stdout: &'a mut W,
}

impl<'a, R: BufRead, W: Write> IsiInterpreter<'a, R, W> {
    /// Cria um novo interpretador.
    pub fn new(program: &'a IsiProgram, stdin: &'a mut R, stdout: &'a mut W) -> Self {
        IsiInterpreter {
            program,
            sym_table: HashMap::new(),
            sym_types: HashMap::new(),
            stdin,
            stdout,
        }
    }

    /// Executa o programa associado.
    pub fn exec(&mut self) {
        for i in self.visit_program(&self.program) {
            i.unwrap();
        }
    }
}

impl<'a, R: BufRead, W: Write> IsiVisitor for IsiInterpreter<'a, R, W> {
    type Ret = Result<IsiValue, String>;

    fn visit_int_literal(&mut self, lit: &isic_front::ast::IntLiteral) -> Self::Ret {
        Ok(IsiValue::Int(lit.0))
    }

    fn visit_float_literal(&mut self, lit: &isic_front::ast::FloatLiteral) -> Self::Ret {
        Ok(IsiValue::Float(lit.0))
    }

    fn visit_string_literal(&mut self, lit: &isic_front::ast::StringLiteral) -> Self::Ret {
        Ok(IsiValue::String(lit.0.clone()))
    }

    fn visit_ident(&mut self, id: &Ident) -> Self::Ret {
        self
            .sym_table
            .get(id)
            .cloned()
            .ok_or_else(|| format!("No value for variable {}", id.name))
    }

    fn visit_decl(&mut self, decl: &isic_front::ast::VarDecl) -> Self::Ret {
        // Replicando mecanismo do type checker, nao gotsei...
        // Mas aqui tambem precisamos dessa info pra poder
        // parsear as entradas do usuario.
        let ty = match decl.var_type.name.as_str() {
            "int" => Ok(IsiType::Int),
            "float" => Ok(IsiType::Float),
            "string" => Ok( IsiType::String ),
            "bool" => Ok( IsiType::Bool ),
            t => Err(format!("Unknown type {} for variable {}", t, decl.var_name.name))
        }?;

        self.sym_types.insert(decl.var_name.clone(), ty);

        Ok(IsiValue::Unit)
    }

    fn visit_bin_expr(&mut self, bexpr: &isic_front::ast::BinExpr) -> Self::Ret {
        let lhs = self.visit_expr(&bexpr.1)?;
        let rhs = self.visit_expr(&bexpr.2)?;

        match bexpr.0 {
            BinaryOp::Add => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Int(l + r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Float(l + r)),
                    (IsiValue::String(l), IsiValue::String(r)) => Ok(IsiValue::String(format!("{}{}", l, r))),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Sub => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Int(l - r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Float(l - r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Mul => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Int(l * r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Float(l * r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Div => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Int(l / r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Float(l / r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Mod => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Int(l % r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Gt => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l > r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l > r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Lt => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l < r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l < r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Geq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l >= r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l >= r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Leq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l <= r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l <= r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Eq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l == r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l == r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Neq => {
                match (lhs, rhs) {
                    (IsiValue::Int(l), IsiValue::Int(r)) => Ok(IsiValue::Bool(l != r)),
                    (IsiValue::Float(l), IsiValue::Float(r)) => Ok(IsiValue::Bool(l != r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::And => {
                match (lhs, rhs) {
                    (IsiValue::Bool(l), IsiValue::Bool(r)) => Ok(IsiValue::Bool(l && r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
            BinaryOp::Or => {
                match (lhs, rhs) {
                    (IsiValue::Bool(l), IsiValue::Bool(r)) => Ok(IsiValue::Bool(l || r)),
                    (l, r) => Err(format!("Unexpected: wrong values for operation {:?}: {:?} and {:?}", bexpr.0, l, r))
                }
            },
        }
    }

    fn visit_fn_call(&mut self, call: &isic_front::ast::FnCall) -> Self::Ret {
        match call.fname.name.as_str() {
            "escreva" => {
                let val = self.visit_expr(&call.args[0])?;

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
            x => return Err(format!("Unknown function name {}", x)),
        }

        Ok(IsiValue::Unit)
    }

    fn visit_assignment(&mut self, assignment: &isic_front::ast::Assignment) -> Self::Ret {
        let val = self.visit_expr(&assignment.val)?;

        self.sym_table.insert(assignment.ident.clone(), val);

        Ok(IsiValue::Unit)
    }

    fn visit_conditional(&mut self, conditional: &isic_front::ast::Conditional) -> Self::Ret {
        let should_take = self.visit_expr(&conditional.cond)?;

        match should_take {
            IsiValue::Bool(true) => {
                for stmt in &conditional.taken {
                    self.visit_statement(stmt)?;
                }
            }
            _ => {
                for stmt in &conditional.not_taken {
                    self.visit_statement(stmt)?;
                }
            }
        }

        Ok(IsiValue::Unit)
    }

    fn visit_while_loop(&mut self, while_loop: &isic_front::ast::WhileLoop) -> Self::Ret {
        while let IsiValue::Bool(true) = self.visit_expr(&while_loop.cond)? {
            for stmt in &while_loop.body {
                self.visit_statement(stmt)?;
            }
        }

        Ok(IsiValue::Unit)
    }

    fn visit_do_while_loop(&mut self, do_while_loop: &isic_front::ast::DoWhileLoop) -> Self::Ret {
        loop {
            for stmt in &do_while_loop.body {
                self.visit_statement(stmt)?;
            }

            if !matches!(self.visit_expr(&do_while_loop.cond)?, IsiValue::Bool(true)) {
                break;
            }
        }

        Ok(IsiValue::Unit)
    }

    fn visit_negation(&mut self, neg: &isic_front::ast::Negation) -> Self::Ret {
        match self.visit_expr(&neg.expr)? {
            IsiValue::Bool(b) => Ok(IsiValue::Bool(!b)),
            v => Err(format!("Unexpected: tried to negate a non-Bool value {:?}", v)),
        }
    }

    fn visit_multi_decl(&mut self, decls: &isic_front::ast::MultiVarDecl) -> Self::Ret {
        for decl in &decls.0 {
            self.visit_decl(decl)?;
        }

        Ok(IsiValue::Unit)
    }
}

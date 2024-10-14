use crate::Expr;
use crate::Kind;
use crate::Opcode;
use crate::Value;

pub struct Compiler {
    pub instructions: Vec<Opcode>,
    pub consts: Vec<Value>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            consts: Vec::new(),
        }
    }

    fn emit(&mut self, opcode: Opcode) -> usize {
        self.instructions.push(opcode);
        self.instructions.len() - 1
    }

    fn save(&mut self, value: Value) -> usize {
        self.consts.push(value);
        self.consts.len() - 1
    }

    pub fn compile(&mut self, source: &Vec<Expr>) -> Result<(), String> {
        for expr in source.iter() {
            self.assemble(expr)?;
            self.emit(Opcode::Pop);
        }
        Ok(())
    }

    fn assemble(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Ident(_, _) => todo!(),
            Expr::Integer(_, integer) => {
                let integer = Value::Integer(*integer);
                let index = self.save(integer);
                self.emit(Opcode::Const(index));
            }
            Expr::Float(_, _) => todo!(),
            Expr::Boolean(_, boolean) => {
                if *boolean {
                    self.emit(Opcode::True);
                } else {
                    self.emit(Opcode::False);
                }
            }
            Expr::String(_, _) => todo!(),
            Expr::Let(_, _, _) => todo!(),
            Expr::Return(_, _) => todo!(),
            Expr::Unary(token, right) => {
                self.assemble(right)?;
                match token.kind {
                    Kind::Minus => self.emit(Opcode::Minus),
                    Kind::Bang => self.emit(Opcode::Bang),
                    _ => Err(format!("Unknown operator: {}", token))?,
                };
            }
            Expr::Binary(token, left, right) => {
                self.assemble(left)?;
                self.assemble(right)?;
                match token.kind {
                    Kind::Plus => self.emit(Opcode::Add),
                    Kind::Minus => self.emit(Opcode::Sub),
                    Kind::Star => self.emit(Opcode::Mul),
                    Kind::Slash => self.emit(Opcode::Div),
                    Kind::Lt => self.emit(Opcode::Lt),
                    Kind::Gt => self.emit(Opcode::Gt),
                    Kind::Eq => self.emit(Opcode::Eq),
                    Kind::Ne => self.emit(Opcode::Ne),
                    _ => Err(format!("Unknown operator: {}", token))?,
                };
            }
            Expr::Paren(_, value) => self.assemble(value)?,
            Expr::If(_, condition, consequence, alternative) => {
                self.assemble(condition)?;
                let jump_not_truthy_position = self.instructions.len();
                if consequence.is_empty() {
                    self.emit(Opcode::None);
                } else {
                    for expr in consequence.iter() {
                        self.assemble(expr)?;
                    }
                }
                self.instructions.insert(
                    jump_not_truthy_position,
                    Opcode::Judge(self.instructions.len() + 2),
                );
                let jump_position = self.instructions.len();
                if alternative.is_empty() {
                    self.emit(Opcode::None);
                } else {
                    for expr in alternative.iter() {
                        self.assemble(expr)?;
                    }
                }
                self.instructions
                    .insert(jump_position, Opcode::Jump(self.instructions.len() + 1));
            }
            Expr::Function(_, _, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
            Expr::Array(_, _) => todo!(),
            Expr::Map(_, _) => todo!(),
            Expr::Index(_, _, _) => todo!(),
            Expr::Field(_, _, _) => todo!(),
            Expr::Request(_, _, _, _) => todo!(),
            Expr::Test(_, _, _) => todo!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Compiler;
    use crate::Opcode;
    use crate::Value;

    fn run_compiler_tests(tests: Vec<(&str, Vec<Value>, Vec<Opcode>)>) {
        for (text, consts, instructions) in tests {
            let source = crate::parser::Parser::new(text).parse().unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile(&source);
            assert!(result.is_ok(), "compile error: {}", result.unwrap_err());
            assert_eq!(compiler.instructions, instructions);
            assert_eq!(compiler.consts, consts);
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            (
                "1 + 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Add, Opcode::Pop],
            ),
            (
                "1; 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Pop, Opcode::Const(1), Opcode::Pop],
            ),
            (
                "1 - 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Sub, Opcode::Pop],
            ),
            (
                "1 * 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Mul, Opcode::Pop],
            ),
            (
                "2 / 1",
                vec![Value::Integer(2), Value::Integer(1)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Div, Opcode::Pop],
            ),
            (
                "-1",
                vec![Value::Integer(1)],
                vec![Opcode::Const(0), Opcode::Minus, Opcode::Pop],
            ),
        ];
        run_compiler_tests(tests);
    }

    #[test]
    fn test_boolean_arithmetic() {
        let tests = vec![
            ("true", vec![], vec![Opcode::True, Opcode::Pop]),
            ("false", vec![], vec![Opcode::False, Opcode::Pop]),
            (
                "1 < 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Lt, Opcode::Pop],
            ),
            (
                "1 > 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Gt, Opcode::Pop],
            ),
            (
                "1 == 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Eq, Opcode::Pop],
            ),
            (
                "1 != 2",
                vec![Value::Integer(1), Value::Integer(2)],
                vec![Opcode::Const(0), Opcode::Const(1), Opcode::Ne, Opcode::Pop],
            ),
            (
                "true == false",
                vec![],
                vec![Opcode::True, Opcode::False, Opcode::Eq, Opcode::Pop],
            ),
            (
                "true != false",
                vec![],
                vec![Opcode::True, Opcode::False, Opcode::Ne, Opcode::Pop],
            ),
            ("!true", vec![], vec![Opcode::True, Opcode::Bang, Opcode::Pop]),
        ];
        run_compiler_tests(tests);
    }

    #[test]
    fn test_conditionals() {
        let tests = vec![
            (
                "if (true) { 10 }; 3333;",
                vec![Value::Integer(10), Value::Integer(3333)],
                vec![
                    Opcode::True,
                    Opcode::Judge(4),
                    Opcode::Const(0),
                    Opcode::Jump(5),
                    Opcode::None,
                    Opcode::Pop,
                    Opcode::Const(1),
                    Opcode::Pop,
                ],
            ),
            (
                "if (true) { 10 } else { 20 }; 3333;",
                vec![Value::Integer(10), Value::Integer(20), Value::Integer(3333)],
                vec![
                    Opcode::True,
                    Opcode::Judge(4),
                    Opcode::Const(0),
                    Opcode::Jump(5),
                    Opcode::Const(1),
                    Opcode::Pop,
                    Opcode::Const(2),
                    Opcode::Pop,
                ],
            ),
            (
                "if (true) {} else { 10 }; 3333;",
                vec![Value::Integer(10), Value::Integer(3333)],
                vec![
                    Opcode::True,
                    Opcode::Judge(4),
                    Opcode::None,
                    Opcode::Jump(5),
                    Opcode::Const(0),
                    Opcode::Pop,
                    Opcode::Const(1),
                    Opcode::Pop,
                ],
            ),
        ];
        run_compiler_tests(tests);
    }
}

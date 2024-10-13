use crate::Opcode;
use crate::Value;

pub struct Vm {
    consts: Vec<Value>,
    instructions: Vec<Opcode>,
    stack: Vec<Value>,
    sp: usize,
}

impl Vm {
    pub fn new(consts: Vec<Value>, instructions: Vec<Opcode>) -> Self {
        Self {
            consts,
            instructions,
            stack: Vec::new(),
            sp: usize::MIN,
        }
    }

    pub fn run(&mut self) {
        for i in 0..self.instructions.len() {
            let opcode = self.instructions[i];
            match opcode {
                Opcode::Const(index) => {
                    let value = self.consts[index].clone();
                    self.push(value);
                }
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                    let right = self.pop();
                    let left = self.pop();
                    match (left, right, opcode) {
                        (Value::Integer(left), Value::Integer(right), Opcode::Add) => {
                            self.push(Value::Integer(left + right))
                        }
                        (Value::Integer(left), Value::Integer(right), Opcode::Sub) => {
                            self.push(Value::Integer(left - right))
                        }
                        (Value::Integer(left), Value::Integer(right), Opcode::Mul) => {
                            self.push(Value::Integer(left * right))
                        }
                        (Value::Integer(left), Value::Integer(right), Opcode::Div) => {
                            self.push(Value::Integer(left / right))
                        }
                        (left, right, opcode) => panic!(
                            "unsupported types for binary operation: {} {:?} {}",
                            left, opcode, right
                        ),
                    }
                }
            }
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.insert(self.sp, value);
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.sp -= 1;
        self.stack[self.sp].clone()
    }

    pub fn past(&self) -> &Value {
        &self.stack[self.sp]
    }
}

#[cfg(test)]
mod tests {
    use crate::Compiler;
    use crate::Value;
    use crate::Vm;

    fn run_vm_tests(tests: Vec<(&str, Value)>) {
        for (text, value) in tests {
            let source = crate::parser::Parser::new(text).parse().unwrap();
            let mut compiler = Compiler::new();
            let result = compiler.compile(&source);
            assert!(result.is_ok(), "compile error: {}", result.unwrap_err());
            let mut vm = Vm::new(compiler.consts, compiler.instructions);
            vm.run();
            println!("{} = {}", vm.past(), value);
            assert_eq!(vm.past(), &value);
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            ("1", Value::Integer(1)),
            ("2", Value::Integer(2)),
            ("1 + 2", Value::Integer(3)),
            ("1 - 2", Value::Integer(-1)),
            ("1 * 2", Value::Integer(2)),
            ("4 / 2", Value::Integer(2)),
            ("50 / 2 * 2 + 10 - 5", Value::Integer(55)),
            ("5 * (2 + 10)", Value::Integer(60)),
            ("5 + 5 + 5 + 5 - 10", Value::Integer(10)),
            ("2 * 2 * 2 * 2 * 2", Value::Integer(32)),
            ("5 * 2 + 10", Value::Integer(20)),
            ("5 + 2 * 10", Value::Integer(25)),
            ("5 * (2 + 10)", Value::Integer(60)),
        ];
        run_vm_tests(tests);
    }
}

// use crate::context::Context;
// use crate::http::Client;
// use crate::record::Assert;
// use crate::Expr;
// use crate::token::{Kind, Token};
// use crate::Value;
// use std::collections::HashMap;

// fn eval_expr(expr: &Expr, context: &mut Context) -> Value {
//     match expr {
//         Expr::Ident(_, value) => eval_ident_expr(value, context),
//         Expr::Integer(_, value) => eval_integer_literal(value),
//         Expr::Float(_, value) => eval_float_literal(value),
//         Expr::Boolean(_, value) => eval_boolean_literal(value),
//         Expr::String(_, value) => eval_string_literal(value),
//         Expr::Let(token, name, value) => eval_let_expr(token, name, value, context),
//         Expr::Return(_, value) => eval_return_expr(value, context),
//         Expr::Unary(token, right) => eval_unary_expr(token, right, context),
//         Expr::Binary(token, left, right) => eval_binary_expr(token, left, right, context),
//         Expr::Paren(_, value) => eval_paren_expr(value, context),
//         Expr::If(token, condition, consequence, alternative) => {
//             eval_if_expr(token, condition, consequence, alternative, context)
//         }
//         Expr::Function(_, _, parameters, body) => eval_function_literal(parameters, body),
//         Expr::Call(_, function, arguments) => eval_call_expr(function, arguments, context),
//         Expr::Array(_, elements) => eval_array_literal(elements, context),
//         Expr::Map(_, pairs) => eval_map_literal(pairs, context),
//         Expr::Index(_, left, index) => eval_index_expr(left, index, context),
//         Expr::Field(_, object, field) => eval_field_expr(object, field, context),
//         Expr::Request(_, name, pieces, asserts) => eval_request_literal(name, pieces, asserts),
//         Expr::Test(_, _, _) => todo!(),
//     }
// }

// fn eval_let_expr(
//     token: &Token,
//     name: &Option<String>,
//     value: &Option<Box<Expr>>,
//     context: &mut Context,
// ) -> Value {
//     let variable_name;
//     if let Some(name) = name {
//         variable_name = name;
//     } else {
//         return Value::Error(format!("variable:{} name is none", token));
//     }
//     if let Some(value) = value {
//         let value = eval_expr(value, context);
//         if value.is_error() {
//             return value;
//         }
//         context.set(variable_name.clone(), value.clone());
//         value
//     } else {
//         Value::Error(format!("variable:{} value is none", token))
//     }
// }

// fn eval_return_expr(value: &Option<Box<Expr>>, context: &mut Context) -> Value {
//     if let Some(value) = value {
//         let value = eval_expr(value, context);
//         if value.is_error() {
//             return value;
//         }
//         Value::Return(Box::new(value))
//     } else {
//         Value::Error("return value expr is none".to_string())
//     }
// }

// fn eval_ident_expr(value: &String, context: &mut Context) -> Value {
//     if let Some(value) = context.get(value) {
//         value
//     } else {
//         Value::Error(format!("ident:{} not found", value))
//     }
// }

// fn eval_integer_literal(value: &Option<i64>) -> Value {
//     if let Some(integer) = value {
//         Value::Integer(*integer)
//     } else {
//         Value::Integer(i64::default())
//     }
// }

// fn eval_float_literal(value: &Option<f64>) -> Value {
//     if let Some(float) = value {
//         Value::Float(*float)
//     } else {
//         Value::Float(f64::default())
//     }
// }

// fn eval_boolean_literal(value: &Option<bool>) -> Value {
//     if let Some(boolean) = value {
//         Value::Boolean(*boolean)
//     } else {
//         Value::Boolean(bool::default())
//     }
// }

// fn eval_string_literal(string: &str) -> Value {
//     Value::String(string.to_owned())
// }

// fn eval_unary_expr(token: &Token, right: &Option<Box<Expr>>, context: &mut Context) -> Value {
//     if let Some(right) = right {
//         let right = eval_expr(right, context);
//         if right.is_error() {
//             return right;
//         }
//         match token.kind {
//             Kind::Bang => eval_bang_operator(right),
//             Kind::Minus => eval_minus_operator(right),
//             _ => Value::Error(format!("unknown operator: {}{}", token, right.kind())),
//         }
//     } else {
//         Value::Error(format!("{} unary expr right is none", token))
//     }
// }

// fn eval_bang_operator(right: Value) -> Value {
//     match right {
//         Value::Boolean(boolean) => Value::Boolean(!boolean),
//         _ => Value::Boolean(bool::default()),
//     }
// }

// fn eval_minus_operator(right: Value) -> Value {
//     match right {
//         Value::Integer(integer) => Value::Integer(-integer),
//         Value::Float(float) => Value::Float(-float),
//         _ => Value::Error(format!("unknown operator: -{}", right.kind())),
//     }
// }

// fn eval_binary_expr(
//     token: &Token,
//     left: &Option<Box<Expr>>,
//     right: &Option<Box<Expr>>,
//     context: &mut Context,
// ) -> Value {
//     let left = if let Some(left) = left {
//         eval_expr(left, context)
//     } else {
//         Value::Error(format!("{} binary expr left is none", token))
//     };
//     if left.is_error() {
//         return left;
//     }
//     let right = if let Some(right) = right {
//         eval_expr(right, context)
//     } else {
//         Value::Error(format!("{} binary expr right is none", token))
//     };
//     if right.is_error() {
//         return right;
//     }
//     eval_binary_operator(token, &left, &right)
// }

// fn eval_binary_operator(token: &Token, left: &Value, right: &Value) -> Value {
//     match (&left, &right) {
//         (Value::Integer(left), Value::Integer(right)) => eval_binary_integer(token, *left, *right),
//         (Value::Float(left), Value::Float(right)) => eval_binary_float(token, *left, *right),
//         (Value::Integer(left), Value::Float(right)) => eval_binary_float(token, *left as f64, *right),
//         (Value::Float(left), Value::Integer(right)) => eval_binary_float(token, *left, *right as f64),
//         (Value::Boolean(left), Value::Boolean(right)) => eval_binary_boolean(token, *left, *right),
//         (Value::String(left), Value::String(right)) => eval_binary_string(token, left, right),
//         _ => Value::Error(format!("type mismatch: {}{}{}", left.kind(), token, right.kind())),
//     }
// }

// fn eval_binary_integer(token: &Token, left: i64, right: i64) -> Value {
//     match token.kind {
//         Kind::Plus => Value::Integer(left + right),
//         Kind::Minus => Value::Integer(left - right),
//         Kind::Star => Value::Integer(left * right),
//         Kind::Slash => Value::Integer(left / right),
//         Kind::Lt => Value::Boolean(left < right),
//         Kind::Gt => Value::Boolean(left > right),
//         Kind::Eq => Value::Boolean(left == right),
//         Kind::Ne => Value::Boolean(left != right),
//         _ => Value::Error(format!("not support operator: {}{}{}", left, token, right)),
//     }
// }

// fn eval_binary_float(token: &Token, left: f64, right: f64) -> Value {
//     match token.kind {
//         Kind::Plus => Value::Float(left + right),
//         Kind::Minus => Value::Float(left - right),
//         Kind::Star => Value::Float(left * right),
//         Kind::Slash => Value::Float(left / right),
//         Kind::Lt => Value::Boolean(left < right),
//         Kind::Gt => Value::Boolean(left > right),
//         Kind::Eq => Value::Boolean(left == right),
//         Kind::Ne => Value::Boolean(left != right),
//         _ => Value::Error(format!("not support operator: {}{}{}", left, token, right)),
//     }
// }

// fn eval_binary_boolean(token: &Token, left: bool, right: bool) -> Value {
//     match token.kind {
//         Kind::Lt => Value::Boolean(left & !right),
//         Kind::Gt => Value::Boolean(!left & right),
//         Kind::Eq => Value::Boolean(left == right),
//         Kind::Ne => Value::Boolean(left != right),
//         _ => Value::Error(format!("not support operator: {}{}{}", left, token, right)),
//     }
// }

// fn eval_binary_string(token: &Token, left: &String, right: &String) -> Value {
//     match token.kind {
//         Kind::Plus => Value::String(format!("{}{}", left, right)),
//         Kind::Lt => Value::Boolean(left < right),
//         Kind::Gt => Value::Boolean(left > right),
//         Kind::Eq => Value::Boolean(left == right),
//         Kind::Ne => Value::Boolean(left != right),
//         _ => Value::Error(format!("not support operator: {}{}{}", left, token, right)),
//     }
// }

// fn eval_paren_expr(value: &Option<Box<Expr>>, context: &mut Context) -> Value {
//     if let Some(value) = value {
//         let value = eval_expr(value, context);
//         if value.is_error() {
//             return value;
//         }
//         value
//     } else {
//         Value::Error("paren value expr is none".to_string())
//     }
// }

// fn eval_if_expr(
//     token: &Token,
//     condition: &Option<Box<Expr>>,
//     consequence: &[Expr],
//     alternative: &[Expr],
//     context: &mut Context,
// ) -> Value {
//     let condition = if let Some(condition) = condition {
//         eval_expr(condition, context)
//     } else {
//         Value::Error(format!("{} if expr condition is none", token))
//     };
//     if condition.is_error() {
//         return condition;
//     }
//     match condition {
//         Value::Boolean(true) => eval_block_expr(consequence, context),
//         _ => eval_block_expr(alternative, context),
//     }
// }

// fn eval_function_literal(parameters: &[String], body: &[Expr]) -> Value {
//     Value::Function(parameters.to_owned(), body.to_owned())
// }

// fn eval_function_expr(
//     parameters: Vec<String>,
//     arguments: Vec<Value>,
//     body: Vec<Expr>,
//     context: &mut Context,
// ) -> Value {
//     if arguments.len() != parameters.len() {
//         Value::Error(format!(
//             "expect {} parameters but {}",
//             parameters.len(),
//             arguments.len()
//         ))
//     } else {
//         for (parameter, argument) in parameters.into_iter().zip(arguments.into_iter()) {
//             context.set(parameter, argument);
//         }
//         eval_block_expr(&body, context)
//     }
// }

// fn eval_call_expr(invoke: &Option<Box<Expr>>, arguments: &[Expr], context: &mut Context) -> Value {
//     let invoke = if let Some(invoke) = invoke {
//         eval_expr(invoke, context)
//     } else {
//         Value::Error("call expr function is none".to_string())
//     };
//     if invoke.is_error() {
//         return invoke;
//     }
//     let arguments = eval_exprs(arguments, context);
//     if let Some(last) = arguments.last() {
//         if last.is_error() {
//             return last.clone();
//         }
//     }
//     eval_call_value(invoke, arguments, context)
// }

// fn eval_call_value(invoke: Value, arguments: Vec<Value>, context: &mut Context) -> Value {
//     if let Value::Function(parameters, body) = invoke {
//         let mut context = Context::clone(context);
//         eval_function_expr(parameters, arguments, body, &mut context)
//     } else if let Value::Native(function) = invoke {
//         function(arguments)
//     } else if let Value::Request(name, pieces, asserts) = invoke {
//         let mut context = Context::clone(context);
//         eval_request_expr(name, pieces, asserts, &mut context)
//     } else {
//         Value::Error(String::from("not a function or request"))
//     }
// }

// fn eval_array_literal(elements: &[Expr], context: &mut Context) -> Value {
//     let elements = eval_exprs(elements, context);
//     if let Some(last) = elements.last() {
//         if last.is_error() {
//             return last.clone();
//         }
//     }
//     Value::Array(elements)
// }

// fn eval_exprs(elements: &[Expr], context: &mut Context) -> Vec<Value> {
//     let mut objects = Vec::new();
//     for element in elements {
//         let value = eval_expr(element, context);
//         objects.push(value.clone());
//         if value.is_error() {
//             return objects;
//         }
//     }
//     objects
// }

// fn eval_map_literal(map: &Vec<(Expr, Expr)>, context: &mut Context) -> Value {
//     let mut pairs = HashMap::new();

//     for (key, value) in map {
//         let key = eval_expr(key, context);
//         if key.is_error() {
//             return key;
//         }
//         let value = eval_expr(value, context);
//         if value.is_error() {
//             return value;
//         }
//         pairs.insert(key.to_string(), value);
//     }
//     Value::Map(pairs)
// }

// fn eval_index_expr(left: &Option<Box<Expr>>, index: &Option<Box<Expr>>, context: &mut Context) -> Value {
//     let left = if let Some(left) = left {
//         eval_expr(left, context)
//     } else {
//         Value::Error("index expr left is none".to_string())
//     };
//     if left.is_error() {
//         return left;
//     }
//     let index = if let Some(index) = index {
//         eval_expr(index, context)
//     } else {
//         Value::Error("index expr index is none".to_string())
//     };
//     if index.is_error() {
//         return index;
//     }
//     match (&left, &index) {
//         (Value::Array(elements), Value::Integer(index)) => {
//             let element = elements.get(*index as usize);
//             match element {
//                 Some(element) => element.clone(),
//                 None => Value::None,
//             }
//         }
//         (Value::Map(elements), key) => {
//             let element = elements.get(&key.to_string());
//             match element {
//                 Some(element) => element.clone(),
//                 None => Value::None,
//             }
//         }
//         (_, _) => Value::Error(format!("index operator not support: {}", left.kind())),
//     }
// }

// fn eval_field_expr(object: &Option<Box<Expr>>, field: &Option<String>, context: &mut Context) -> Value {
//     let object = if let Some(object) = object {
//         eval_expr(object, context)
//     } else {
//         Value::Error("index expr left is none".to_string())
//     };
//     if object.is_error() {
//         return object;
//     }
//     match (&object, field) {
//         (Value::Map(object), Some(field)) => {
//             let element = object.get(field);
//             match element {
//                 Some(element) => element.clone(),
//                 None => Value::None,
//             }
//         }
//         (_, None) => Value::Error("field name is none".to_string()),
//         (_, _) => Value::Error(format!("field operator not support: {}", object.kind())),
//     }
// }

// pub fn eval_block_expr(exprs: &[Expr], context: &mut Context) -> Value {
//     let mut result = Value::None;
//     for expr in exprs.iter() {
//         result = eval_expr(expr, context);
//         match result {
//             Value::Error(_) => return result,
//             Value::Return(value) => return *value,
//             _ => {}
//         }
//     }
//     result
// }

// fn eval_request_literal(name: &str, pieces: &[Expr], asserts: &[Expr]) -> Value {
//     Value::Request(name.to_owned(), pieces.to_owned(), asserts.to_owned())
// }

// fn eval_request_expr(name: String, pieces: Vec<Expr>, exprs: Vec<Expr>, context: &mut Context) -> Value {
//     let message = pieces
//         .iter()
//         .map(|p| eval_expr(p, context).to_string())
//         .collect::<String>();
//     let (request, response, time, error) = Client::default().send(&message);
//     let value = response.to_value();
//     if let Value::Map(map) = &value {
//         map.iter()
//             .for_each(|(key, value)| context.set(key.clone(), value.clone()))
//     }
//     let mut asserts = Vec::new();
//     for expr in exprs {
//         if let Expr::Binary(token, left, right) = &expr {
//             let left = if let Some(left) = left {
//                 eval_expr(left, context)
//             } else {
//                 Value::None
//             };
//             let right = if let Some(right) = right {
//                 eval_expr(right, context)
//             } else {
//                 Value::None
//             };
//             let result = if let Value::Boolean(boolean) = eval_binary_operator(token, &left, &right) {
//                 boolean
//             } else {
//                 false
//             };
//             let compare = token.clone();
//             asserts.push(Assert {
//                 expr,
//                 left,
//                 compare,
//                 right,
//                 result,
//             })
//         }
//     }
//     value
// }

// // TODO test error handling
// #[test]
// fn test_error_handling() {}

// #[test]
// fn test_native_function() {
//     let tests = vec![
//         (r#"length("")"#, Value::Integer(0)),
//         (r#"length("four")"#, Value::Integer(4)),
//         (r#"length("hello world")"#, Value::Integer(11)),
//         (
//             r#"length(1)"#,
//             Value::Error("function length not supported type Integer".to_string()),
//         ),
//         (
//             r#"length("one", "two")"#,
//             Value::Error("wrong number of arguments. got=2, want=1".to_string()),
//         ),
//         (r#"length([1, 2, 3])"#, Value::Integer(3)),
//         (r#"length([])"#, Value::Integer(0)),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("{} == {}", evaluated, expected);
//         assert!(evaluated == expected);
//     }
// }

// #[test]
// fn test_eval_integer_expr() {
//     let tests = vec![
//         ("5", 5),
//         ("10", 10),
//         ("-5", -5),
//         ("-10", -10),
//         ("5 + 5 + 5 + 5 - 10", 10),
//         ("2 * 2 * 2 * 2 * 2", 32),
//         ("-50 + 100 + -50", 0),
//         ("5 * 2 + 10", 20),
//         ("5 + 2 * 10", 25),
//         ("20 + 2 * -10", 0),
//         ("50 / 2 * 2 + 10", 60),
//         ("2 * (5 + 10)", 30),
//         ("3 * 3 * 3 + 10", 37),
//         ("3 * (3 * 3) + 10", 37),
//         ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("evaluated = {}", evaluated);
//         if let Value::Integer(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_float_expr() {
//     let tests = vec![
//         ("0.5", 0.5),
//         ("0.10", 0.10),
//         ("-0.5", -0.5),
//         ("-0.10", -0.10),
//         ("1 + 0.10", 1.1),
//         ("0.10 + 1", 1.1),
//         ("1 - 0.10", 0.9),
//         ("0.1 - 1", -0.9),
//         ("2 * 2 * 2 * 2 * 0.1", 1.6),
//         ("0.1 * 2 * 2 * 2 * 2 ", 1.6),
//         ("5 / 0.2", 25.0),
//         ("0.5 / 2", 0.25),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("evaluated = {}", evaluated);
//         if let Value::Float(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_boolean_expr() {
//     let tests = vec![
//         ("true", true),
//         ("false", false),
//         ("1 < 2", true),
//         ("1 > 2", false),
//         ("1 < 1", false),
//         ("1 > 1", false),
//         ("1 == 1", true),
//         ("1 != 1", false),
//         ("1 == 2", false),
//         ("1 != 2", true),
//         ("true == true", true),
//         ("false == false", true),
//         ("true == false", false),
//         ("true != false", true),
//         ("false != true", true),
//         ("(1 < 2) == true", true),
//         ("(1 < 2) == false", false),
//         ("(1 > 2) == true", false),
//         ("(1 > 2) == false", true),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         if let Value::Boolean(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_bang_operator() {
//     let tests = vec![
//         ("!true", false),
//         ("!false", true),
//         ("!5", false),
//         ("!!true", true),
//         ("!!false", false),
//         ("!!5", true),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         if let Value::Boolean(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_if_expr() {
//     let tests = vec![
//         ("if (true) { 10 }", Value::Integer(10)),
//         ("if (false) { 10 }", Value::None),
//         ("if (1) { 10 }", Value::None),
//         ("if (1 < 2) { 10 }", Value::Integer(10)),
//         ("if (1 > 2) { 10 }", Value::None),
//         ("if (1 > 2) { 10 } else { 20 }", Value::Integer(20)),
//         ("if (1 < 2) { 10 } else { 20 }", Value::Integer(10)),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("evaluated:{}", evaluated);
//         assert!(evaluated == expected);
//     }
// }

// #[test]
// fn test_eval_let_expr() {
//     let tests = vec![
//         ("let a = 5; a;", 5),
//         ("let a = 5 * 5; a;", 25),
//         ("let a = 5; let b = a; b;", 5),
//         ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         if let Value::Integer(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_return_expr() {
//     let tests = vec![
//         ("return 10;", 10),
//         ("return 10; 9;", 10),
//         ("return 2 * 5; 9;", 10),
//         ("9; return 2 * 5; 9;", 10),
//         ("if (10 > 1) { return 10; }", 10),
//         (
//             r#"
//             if (10 > 1) {
//             if (10 > 1) {
//                 return 10;
//             }

//             return 1;
//             }
//             "#,
//             10,
//         ),
//         (
//             r#"
//             let f = fn(x) {
//             return x;
//             x + 10;
//             };
//             f(10);
//             "#,
//             10,
//         ),
//         (
//             r#"
//             let f = fn(x) {
//             let result = x + 10;
//             return result;
//             return 10;
//             };
//             f(10);
//             "#,
//             20,
//         ),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         if let Value::Return(evaluated) = evaluated {
//             if let Value::Integer(evaluated) = *evaluated {
//                 println!("{} == {}", evaluated, expected);
//                 assert!(evaluated == expected);
//             }
//         }
//     }
// }

// #[test]
// fn test_eval_function_object() {
//     let text = "let a = fn(x) { x + 2; };a";
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::Function(parameters, body) = evaluated {
//         assert!(parameters.len() == 1);
//         assert!(parameters[0] == "x");
//         println!("fn({}) {{ {} ;}}", parameters[0], body[0]);
//         assert!(body[0].to_string() == "(x + 2)");
//     }
// }

// #[test]
// fn test_eval_function_call() {
//     let tests = vec![
//         ("let identity = fn(x) { x; }; identity(5);", 5),
//         ("let identity = fn(x) { return x; }; identity(5);", 5),
//         ("let double = fn(x) { x * 2; }; double(5);", 10),
//         ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
//         ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
//         ("fn(x) { x; }(5)", 5),
//         ("fn x(x) { x; }; x(5)", 5),
//         ("fn len(x) { x; }; len(10)", 10),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("evaluated:{}:{}", evaluated, evaluated.kind());
//         if let Value::Integer(evaluated) = evaluated {
//             println!("{} == {}", evaluated, expected);
//             assert!(evaluated == expected);
//         }
//     }
// }

// #[test]
// fn test_eval_enclosing_context() {
//     let text = r#"
//     let first = 10;
//     let second = 10;
//     let third = 10;
    
//     let ourFunction = fn(first) {
//     let second = 20;
    
//     first + second + third;
//     };
    
//     ourFunction(20) + first + second;
//     "#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::Integer(evaluated) = evaluated {
//         assert!(evaluated == 70);
//     }
// }

// #[test]
// fn test_eval_closure() {
//     let text = r#"
//     let newAdder = fn(x) {
//         fn(y) { x + y };
//     };
      
//     let addTwo = newAdder(2);
//     addTwo(2);
//     "#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::Integer(evaluated) = evaluated {
//         assert!(evaluated == 4);
//     }
// }

// #[test]
// fn test_eval_fibonacci() {
//     let text = r#"
//     let fibonacci = fn (x) {
//         if (x == 0) {
//           0
//         } else {
//           if (x == 1) {
//             1
//           } else {
//             fibonacci(x - 1) + fibonacci(x -2)
//           }
//         }
//       };  
//     fibonacci(22);
//     "#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     println!("evaluated:{}", evaluated);
//     if let Value::Integer(evaluated) = evaluated {
//         assert!(evaluated == 17711);
//     }
// }

// #[test]
// fn test_fibonacci() {
//     fn fibonacci(x: i64) -> i64 {
//         match x {
//             0 => 0,
//             1 => 1,
//             _ => fibonacci(x - 1) + fibonacci(x - 2),
//         }
//     }
//     println!("fibonacci(22):{}", fibonacci(22));
// }

// #[test]
// fn test_eval_string_literal() {
//     let text = r#""Hello World!""#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::String(evaluated) = evaluated {
//         assert!(evaluated == "Hello World!");
//     }
// }

// #[test]
// fn test_eval_string_concat() {
//     let text = r#""Hello" + " " + "World!""#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::String(evaluated) = evaluated {
//         assert!(evaluated == "Hello World!");
//     }
// }

// #[test]
// fn test_eval_array_literal() {
//     let text = "[1, 2 * 2, 3 + 3]";
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::Array(elements) = evaluated {
//         assert!(elements[0] == Value::Integer(1));
//         assert!(elements[1] == Value::Integer(4));
//         assert!(elements[2] == Value::Integer(6));
//     }
// }

// #[test]
// fn test_eval_array_index_expr() {
//     let tests = vec![
//         ("[1, 2, 3][0]", Value::Integer(1)),
//         ("[1, 2, 3][1]", Value::Integer(2)),
//         ("[1, 2, 3][2]", Value::Integer(3)),
//         ("let i = 0; [1][i];", Value::Integer(1)),
//         ("[1, 2, 3][1 + 1];", Value::Integer(3)),
//         ("let myArray = [1, 2, 3]; myArray[2];", Value::Integer(3)),
//         (
//             "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
//             Value::Integer(6),
//         ),
//         (
//             "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
//             Value::Integer(2),
//         ),
//         ("[1, 2, 3][3]", Value::None),
//         ("[1, 2, 3][-1]", Value::None),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("{} == {}", evaluated, expected);
//         assert!(evaluated == expected);
//     }
// }

// #[test]
// fn test_eval_map_literal() {
//     let text = r#"
//     let two = "two";
// 	{
// 		"one": 10 - 9,
// 		two: 1 + 1,
// 		"thr" + "ee": 6 / 2,
// 		4: 4,
// 		true: 5,
// 		false: 6
// 	}
//     "#;
//     let expected = vec![
//         (String::from("one"), 1),
//         (String::from("two"), 2),
//         (String::from("three"), 3),
//         (String::from("4"), 4),
//         (String::from("true"), 5),
//         (String::from("false"), 6),
//     ];
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     if let Value::Map(pairs) = evaluated {
//         for (key, expected_value) in expected {
//             if let Value::Integer(evaluated_value) = pairs.get(&key).unwrap().clone() {
//                 println!("{} == {}", evaluated_value, expected_value);
//                 assert!(evaluated_value == expected_value);
//             }
//         }
//     }
// }

// #[test]
// fn test_eval_map_index_expr() {
//     let tests = vec![
//         (r#"{"foo": 5}["foo"]"#, Value::Integer(5)),
//         (r#"{"foo": 5}["bar"]"#, Value::None),
//         (r#"let key = "foo"; {"foo": 5}[key]"#, Value::Integer(5)),
//         (r#"{}["foo"]"#, Value::None),
//         (r#"{5: 5}[5]"#, Value::Integer(5)),
//         (r#"{true: 5}[true]"#, Value::Integer(5)),
//         (r#"{false: 5}[false]"#, Value::Integer(5)),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("{} == {}", evaluated, expected);
//         assert!(evaluated == expected);
//     }
// }

// #[test]
// fn test_eval_field_expr() {
//     let tests = vec![
//         (r#"{"foo": 5}.foo]"#, Value::Integer(5)),
//         (r#"{"foo": 5}.bar]"#, Value::None),
//         (r#"let key = "foo"; {"foo": 5}.key"#, Value::None),
//         (r#"{}.foo"#, Value::None),
//         (r#"{5: 5}.5"#, Value::Integer(5)),
//         (r#"{true: 5}.true"#, Value::Integer(5)),
//         (r#"{false: 5}.false"#, Value::Integer(5)),
//     ];
//     for (text, expected) in tests {
//         let source = crate::parser::Parser::new(text).parse();
//         let mut context = Context::default();
//         let evaluated = eval_block_expr(&source, &mut context);
//         println!("{} == {}", evaluated, expected);
//         assert!(evaluated == expected);
//     }
// }

// #[test]
// fn test_eval_request_literal() {
//     let text = r#"
//     rq request`
//       GET http://${host}/api
//       Host: example.com
//     `[
//       status == 200,
//       regex(text, "^\d{4}-\d{2}-\d{2}$") == "2022-02-22"
//     ];
//     request"#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     println!("evaluated:{}", evaluated);
//     if let Value::Request(name, pieces, asserts) = evaluated {
//         assert!(name == "request");
//         assert!(pieces.len() == 3);
//         assert!(pieces[0].to_string() == "\"GET http://\"");
//         assert!(pieces[1].to_string() == "host");
//         assert!(pieces[2].to_string() == "\"/api\nHost: example.com\n\"");
//         assert!(asserts.len() == 2);
//         assert!(asserts[0].to_string() == "(status == 200)");
//         assert!(asserts[1].to_string() == r#"(regex(text, "^\d{4}-\d{2}-\d{2}$") == "2022-02-22")"#);
//     }
// }

// #[test]
// fn test_eval_request_expr() {
//     let text = r#"
//     rq request`
//       GET http://${host}/get
//       Host: ${host}
//       Connection: close
//     `[status == 200];
//     let host = "httpbin.org";
//     let response = request();
//     response.status"#;
//     let source = crate::parser::Parser::new(text).parse();
//     let mut context = Context::default();
//     let evaluated = eval_block_expr(&source, &mut context);
//     println!("evaluated:{}", evaluated);
//     assert!(evaluated == Value::Integer(200));
// }

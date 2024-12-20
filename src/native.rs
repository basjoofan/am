use std::collections::HashMap;

use crate::http;
use crate::Value;

pub fn get(name: &str) -> Option<isize> {
    match name {
        "http" => Some(-1),
        "print" => Some(0),
        "println" => Some(1),
        "format" => Some(2),
        "length" => Some(3),
        "append" => Some(4),
        _ => None,
    }
}

pub fn call(index: isize) -> fn(Vec<Value>) -> Value {
    match index {
        -1 => http,
        0 => print,
        1 => println,
        2 => format,
        3 => length,
        4 => append,
        _ => panic!("Unexpected native function"),
    }
}

fn println(objects: Vec<Value>) -> Value {
    match format(objects) {
        error @ Value::Error(_) => error,
        value => {
            println!("{}", value);
            Value::None
        }
    }
}

fn print(objects: Vec<Value>) -> Value {
    match format(objects) {
        error @ Value::Error(_) => error,
        value => {
            print!("{}", value);
            Value::None
        }
    }
}

fn format(mut objects: Vec<Value>) -> Value {
    objects.reverse();
    match objects.pop() {
        Some(Value::String(mut string)) => {
            let regex = regex::Regex::new(r"\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\}").unwrap();
            let matches = regex.find_iter(&string);
            let mut ranges = Vec::new();
            matches.for_each(|m| ranges.push(m.range()));
            ranges.reverse();
            let variables = objects.iter();
            if variables.len() != ranges.len() {
                Value::Error(format!(
                    "wrong number of arguments. got={}, want={}",
                    variables.len(),
                    ranges.len()
                ))
            } else {
                for (range, variable) in ranges.into_iter().zip(variables) {
                    string.replace_range(range, &variable.to_string());
                }
                Value::String(string)
            }
        }
        None => Value::Error("function length need a parameter".to_string()),
        _ => Value::Error("first parameter must be a string".to_string()),
    }
}

fn length(objects: Vec<Value>) -> Value {
    if objects.len() != 1 {
        Value::Error(format!("wrong number of arguments. got={}, want=1", objects.len()))
    } else if let Some(object) = objects.first() {
        match object {
            Value::String(string) => Value::Integer(string.len() as i64),
            Value::Array(elements) => Value::Integer(elements.len() as i64),
            Value::Map(pairs) => Value::Integer(pairs.len() as i64),
            _ => Value::Error(format!("function length not supported type {}", object.kind())),
        }
    } else {
        Value::Error("function length need a parameter".to_string())
    }
}

fn append(mut objects: Vec<Value>) -> Value {
    objects.reverse();
    match objects.pop() {
        Some(Value::Array(mut array)) => {
            while let Some(object) = objects.pop() {
                array.push(object);
            }
            Value::Array(array)
        }
        None => Value::Error("function length need a parameter".to_string()),
        _ => Value::Error("first parameter must be a array".to_string()),
    }
}

fn http(objects: Vec<Value>) -> Value {
    if objects.len() != 1 {
        Value::Error(format!("wrong number of arguments. got={}, want=1", objects.len()))
    } else if let Some(object) = objects.first() {
        match object {
            Value::String(message) => {
                let client = http::Client::default();
                let (request, response, time, error) = client.send(message);
                let mut result = HashMap::new();
                result.insert(String::from("request"), request.to_value());
                result.insert(String::from("response"), response.to_value());
                result.insert(String::from("time"), time.to_value());
                result.insert(String::from("error"), Value::String(error));
                Value::Map(result)
            }
            _ => Value::Error(format!("function send not supported type {}", object.kind())),
        }
    } else {
        Value::Error("function send need a parameter".to_string())
    }
}

#[test]
fn test_format() {
    let tests = vec![
        (
            vec![
                Value::String(String::from("Hello, {name}!")),
                Value::String(String::from("World")),
            ],
            Value::String(String::from("Hello, World!")),
        ),
        (
            vec![
                Value::String(String::from(r#"{ "name": "{name}" , age: 2 }"#)),
                Value::String(String::from("Bob")),
            ],
            Value::String(String::from(r#"{ "name": "Bob" , age: 2 }"#)),
        ),
        (
            vec![
                Value::String(String::from(r#"{ "name": "{name}" , age: {age} }"#)),
                Value::String(String::from("Bob")),
                Value::Integer(2),
            ],
            Value::String(String::from(r#"{ "name": "Bob" , age: 2 }"#)),
        ),
    ];
    for (test, expected) in tests {
        let actual = format(test);
        println!("{}=={}", actual, expected);
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_append() {
    let tests = vec![
        (
            vec![Value::Array(vec![Value::Integer(1)]), Value::Integer(2)],
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        ),
        (
            vec![Value::Array(vec![Value::Integer(1)]), Value::String(String::from("string"))],
            Value::Array(vec![Value::Integer(1), Value::String(String::from("string"))]),
        ),
        (
            vec![Value::Array(vec![Value::Integer(1)]), Value::Boolean(true)],
            Value::Array(vec![Value::Integer(1), Value::Boolean(true)]),
        ),
    ];
    for (test, expected) in tests {
        let actual = append(test);
        println!("{}=={}", actual, expected);
        assert_eq!(actual, expected);
    }
}

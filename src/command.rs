use crate::Compiler;
use crate::Expr;
use crate::Machine;
use crate::Parser;
use std::collections::HashSet;
use std::io::stdin;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::Duration;

pub const NAME: &str = env!("CARGO_PKG_NAME");

pub fn repl() {
    let mut lines = stdin().lock().lines();
    let mut globals = Vec::new();
    let mut compiler = Compiler::new();
    loop {
        if let Some(Ok(text)) = lines.next() {
            if text == "exit" {
                break;
            }
            if text.trim().is_empty() {
                continue;
            }
            match Parser::new(&text).parse() {
                Ok(source) => match compiler.compile(source) {
                    Ok(opcodes) => {
                        let mut machine = Machine::new(compiler.consts(), &mut globals, opcodes);
                        machine.run();
                        println!("{}", machine.past());
                    }
                    Err(message) => println!("{}", message),
                },
                Err(message) => println!("{}", message),
            };
        }
    }
}

pub fn eval(text: String) {
    let mut globals = Vec::new();
    let mut compiler = Compiler::new();
    match Parser::new(&text).parse() {
        Ok(source) => match compiler.compile(source) {
            Ok(opcodes) => {
                let mut machine = Machine::new(compiler.consts(), &mut globals, opcodes);
                machine.run();
                println!("{}", machine.past());
            }
            Err(message) => println!("{}", message),
        },
        Err(message) => println!("{}", message),
    };
}

pub fn run(path: Option<PathBuf>) {
    let text = read_to_string(path.unwrap_or(std::env::current_dir().unwrap()));
    eval(text);
}

pub fn test(name: Option<String>, _: u32, _: Duration, _: u32, path: Option<PathBuf>) {
    let text = read_to_string(path.unwrap_or(std::env::current_dir().unwrap()));
    let mut globals = Vec::new();
    let mut compiler = Compiler::new();
    match Parser::new(&text).parse() {
        Ok(mut source) => {
            let tests = source
                .iter()
                .filter_map(|expr| match expr {
                    Expr::Test(name, _) => Some(name.to_owned()),
                    _ => None,
                })
                .collect::<HashSet<_>>();
            match name {
                Some(name) => {
                    if tests.contains(&name) {
                        source.push(Expr::Call(Box::new(Expr::Ident(name)), vec![]));
                    } else {
                        println!("Test not found: {}", name);
                        return;
                    }
                }
                None => {
                    for name in tests {
                        source.push(Expr::Call(Box::new(Expr::Ident(name)), vec![]))
                    }
                }
            }
            match compiler.compile(source) {
                Ok(opcodes) => {
                    let mut machine = Machine::new(compiler.consts(), &mut globals, opcodes);
                    machine.run();
                }
                Err(message) => println!("{}", message),
            }
        }
        Err(message) => println!("{}", message),
    };
    //     let text = read_to_string(std::env::current_dir().unwrap());
    //     let mut context = Context::default();
    //     let source = Parser::new(&text).parse();
    //     print_error(eval_block_expr(&source, &mut context));
    //     match name {
    //         Some(name) => {
    //             if let Some(block) = source.tests.get(&name) {
    //                 print_error(eval_test(&block, &mut context));
    //             }
    //         }
    //         None => {
    //             source.tests.into_iter().for_each(|(_, block)| {
    //                 let mut context = context.clone();
    //                 std::thread::spawn(move || {
    //                     print_error(eval_test(&block, &mut context));
    //                 });
    //             });
    //         }
    //     }
}

// fn eval_test(exprs: &[Expr], context: &mut Context) -> Value {
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

// pub fn blow(name: String, concurrency: u32, duration: Duration, iterations: u32, file: Option<PathBuf>) {
//     let text = read_to_string(std::env::current_dir().unwrap());
//     let mut context = Context::default();
//     let source = Parser::new(&text).parse();
//     print_error(eval_block_expr(&source, &mut context));
//     let (sender, receiver) = mpsc::channel();
//     let continuous = Arc::new(AtomicBool::new(true));
//     let iterations = iterations / concurrency;
//     for _ in 0..concurrency {
//         let continuous = continuous.clone();
//         let mut context = context.clone();
//         context.set_sender(&sender);
//         context.set_name(&name);
//         let name = name.clone();
//         std::thread::spawn(move || {
//             let mut i = iterations;
//             while continuous.load(Ordering::Relaxed) && i > 0 {
//                 print_error(eval_call_name(&name, &mut context));
//                 i -= 1;
//             }
//         });
//     }
//     handle_ctrlc(continuous.clone());
//     std::thread::spawn(move || {
//         std::thread::sleep(duration);
//         continuous.store(false, Ordering::Relaxed)
//     });
//     std::mem::drop(sender);
//     process_record(receiver, file);
// }

// pub fn test(tag: String, file: Option<PathBuf>) {
//     let text = read_to_string(std::env::current_dir().unwrap());
//     let mut context = Context::default();
//     let source = Parser::new(&text).parse();
//     print_error(eval_block_expr(&source, &mut context));
//     let (sender, receiver) = mpsc::channel();
//     for test in source.tests.into_iter() {
//         if let (Some(tags), Some(name)) = match test {
//             Expr::Function(_, Some(tags), Some(name), _, _) => (Some(tags), Some(name)),
//             Expr::Request(_, Some(tags), name, _, _) => (Some(tags), Some(name)),
//             _ => (None, None),
//         } {
//             if tags.contains(&tag) {
//                 let mut context = context.clone();
//                 context.set_sender(&sender);
//                 context.set_name(&name);
//                 std::thread::spawn(move || {
//                     print_error(eval_call_name(&name, &mut context));
//                 });
//             }
//         }
//     }
//     std::mem::drop(sender);
//     process_record(receiver, file);
// }

// fn process_record(receiver: Receiver<(String, String, Record)>, file: Option<PathBuf>) {
//     let schema = record::schema();
//     let mut writer = record::writer(&schema, file);
//     let mut stats = Stats::default();
//     for (id, name, record) in receiver {
//         // print record
//         println!("=== TEST  {}/{}", name, record.name);
//         let mut result = true;
//         record.asserts.iter().for_each(|assert| {
//             result &= assert.result;
//             println!("{}", assert);
//         });
//         if result {
//             println!("--- PASS  {}/{} ({:?})", name, record.name, record.time.total);
//         } else {
//             println!("--- FAIL  {}/{} ({:?})", name, record.name, record.time.total);
//         }
//         // stat record
//         stats.add(&record.name, record.time.total.as_millis());
//         // store record
//         if let Some(ref mut writer) = writer {
//             let _ = writer.append(record.to(id, name, &schema));
//         }
//     }
//     if let Some(ref mut writer) = writer {
//         let _ = writer.flush();
//     }
//     print!("{}", stats);
// }

// fn print_error(value: Value) {
//     if value.is_error() {
//         println!("{}", value)
//     }
// }

fn read_to_string(path: PathBuf) -> String {
    let mut text = String::new();
    read(path, &mut text).expect("Could not read source file");
    text
}

fn read(path: PathBuf, text: &mut String) -> std::io::Result<()> {
    if path.is_dir() {
        let mut entries = std::fs::read_dir(path)?
            .map(|r| r.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()?;
        entries.sort();
        for entry in entries {
            read(entry, text)?;
        }
    } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new(NAME)) {
        text.push_str(&std::fs::read_to_string(path)?)
    }
    Ok(())
}

// fn handle_ctrlc(continuous: Arc<AtomicBool>) {
//     let _ = ctrlc::set_handler(move || {
//         continuous.store(false, Ordering::Relaxed);
//     });
// }

use crate::interpreter::Value;
use super::Module;
use std::fs;

fn print_nfn(args: &[Value]) -> Result<Value, String> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{}", arg);
    }

    println!();
    Ok(Value::Nil)
}

fn println_nfn(args: &[Value]) -> Result<Value, String> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{}", arg);
    }

    println!();
    Ok(Value::Nil)
}

fn input_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 {
        return Err(format!("too many arguments for function io::input, got {}", args.len()));
    }

    // if a prompt is given p[rint it
    if let Some(p) = args.get(0) {
        print!("{}", p);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).map_err(|e| format!("io::input[error] failed to read input: {}", e))?;

    // remove any trailin chars such as newlines or carrier
    // this is because raw input is like this:
    // input\r\n
    // so get rid of it, i guess we can use .trim() but we'll do it this way for now
    if input.ends_with('\n') {
        input.pop(); // remove last char
        if input.ends_with('\r') {
            input.pop();
        }
    }

    Ok(Value::String(input))
}

fn read_file_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 {
        return Err(format!("too many arguments for io::read_file, got {}", args.len()));
    }

    if let Some(p) = args.get(0) {
        // referenced from https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
        let contents = match p {
            Value::String(s) => fs::read_to_string(s).map_err(|e| format!("io::read_file[error] failed to read input: {}", e))?,
            _ => return Err(format!("expected a string value for path argument, got {}", p))
        };

        Ok(Value::String(contents))
    } else {
        Ok(Value::Nil)
    }
}

fn write_file_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() > 2 {
        return Err(format!("too many arguments for io::write_file, got {}", args.len()));
    }

    // arg 1: file path
    // arg 2: file content
    // if let Some(p) = args.get(0) {
    //     let path = p.to_string();
    //     Ok(())
    // }

    let path = args[0].clone().as_string()?;
    let content = args[1].clone().as_string()?;
    
    fs::write(path, content).map_err(|e| format!("io::write_file[error] failed to write input: {}", e))?;

    Ok(Value::Bool(true))
}

pub const IO_MOD: Module = Module {
    name: "io",
    funcs: &[
        ("print", print_nfn),
        ("println", println_nfn),
        ("input", input_nfn),
        ("read_file", read_file_nfn),
        ("write_file", write_file_nfn),
    ],
};
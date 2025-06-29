use crate::interpreter::Value;
use super::Module;

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


pub const IO_MOD: Module = Module {
    name: "io",
    funcs: &[
        ("print", print_nfn),
        ("println", println_nfn),
        ("input", input_nfn),
    ],
};
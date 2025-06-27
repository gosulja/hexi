use std::fs;
use crate::interpreter::Value;
use crate::stdlib::Module;

fn read_file_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 {
        return Err(format!("too many arguments for fs::read, got {}", args.len()));
    }

    if let Some(p) = args.get(0) {
        // referenced from https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
        let contents = match p {
            Value::String(s) => fs::read_to_string(s).map_err(|e| format!("fs::read failed to read input: {}", e))?,
            _ => return Err(format!("expected a string value for path argument, got {}", p))
        };

        Ok(Value::String(contents))
    } else {
        Ok(Value::Nil)
    }
}

fn write_file_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() > 2 {
        return Err(format!("too many arguments for fs::write, got {}", args.len()));
    }

    // arg 1: file path
    // arg 2: file content
    // if let Some(p) = args.get(0) {
    //     let path = p.to_string();
    //     Ok(())
    // }

    let path = args[0].clone().as_string()?;
    let content = args[1].clone().as_string()?;

    fs::write(path, content).map_err(|e| format!("fs::write failed to write input: {}", e))?;

    Ok(Value::Bool(true))
}

pub const FS_MOD: Module = Module {
    name: "fs",
    funcs: &[
        ("read", read_file_nfn),
        ("write", write_file_nfn),
    ],
};
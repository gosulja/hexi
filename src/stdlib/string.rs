use crate::interpreter::Value;
use super::Module;

fn len_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function string::len, got {}", args.len()))
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(format!("not a string in string::abs, got {}", args[0])),
    }
}

pub const STRING_MOD: Module = Module {
    name: "string",
    funcs: &[
        ("len", len_nfn),
    ],
};
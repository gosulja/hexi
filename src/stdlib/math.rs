use crate::interpreter::Value;
use super::Module;

fn abs_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::abs, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs() as f64)),
        _ => Err(format!("not a number in math::abs, got {}", args[0])),
    }
}

pub const MATH_MOD: Module = Module {
    name: "math",
    funcs: &[
        ("abs", abs_nfn)
    ],
};
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

fn sqrt_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::abs, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sqrt() as f64)),
        _ => Err(format!("not a number in math::abs, got {}", args[0])),
    }
}

fn pow_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function math::pow, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp) as f64)),
        _ => Err(format!("not a number in math::pow, got {}", args[0])),
    }
}

fn floor_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::floor, got {}", args.len()));
    }

    match &args[0]{
        Value::Number(n) => Ok(Value::Number(n.floor() as f64)),
        _ => Err(format!("not a number in math::floor, got {}", args[0])),
    }
}

fn ceil_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::ceil, got {}", args.len()));
    }

    match &args[0]{
        Value::Number(n) => Ok(Value::Number(n.ceil() as f64)),
        _ => Err(format!("not a number in math::ceil, got {}", args[0])),
    }
}

fn sin_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::sin, got {}", args.len()));
    }

    match &args[0]{
        Value::Number(n) => Ok(Value::Number(n.sin() as f64)),
        _ => Err(format!("not a number in math::floor, got {}", args[0])),
    }
}

fn cos_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for function math::cos, got {}", args.len()));
    }

    match &args[0]{
        Value::Number(n) => Ok(Value::Number(n.cos() as f64)),
        _ => Err(format!("not a number in math::cos, got {}", args[0])),
    }
}

fn max_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function math::max, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n.max(*m) as f64)),
        _ => Err(format!("not a number in math::floor, got {}", args[0])),
    }
}

fn min_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function math::min, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n.min(*m) as f64)),
        _ => Err(format!("not a number in math::min, got {}", args[0])),
    }
}

pub const MATH_MOD: Module = Module {
    name: "math",
    funcs: &[
        ("abs", abs_nfn),
        ("sqrt", sqrt_nfn),
        ("pow", pow_nfn),
        ("floor", floor_nfn),
        ("ceil", ceil_nfn),
        ("sin", sin_nfn),
        ("cos", cos_nfn),
        ("max", max_nfn),
        ("min", min_nfn),
    ],
};
use crate::interpreter::Value;
use super::Module;

fn len_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments or too little for function string::len, got {}, want 1", args.len()))
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(format!("not a string in string::abs, got {}", args[0])),
    }
}

fn to_number_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments or too little for function string::to_number, got {}, want 1", args[0]));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.parse::<f64>().unwrap())),
        _ => Err(format!("not a string in string::to_number, got {}", args[0])),
    }
}

fn upper_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments or too little for function string::len, got {}, want 1", args.len()))
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(format!("not a string in string::upper, got {}", args[0])),
    }
}

fn lower_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments or too little for function string::lower, got {}, want 1", args.len()))
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(format!("not a string in string::lower, got {}", args[0])),
    }
}

fn trim_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments or too little for function string::trim, got {}, want 1", args.len()))
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(format!("not a string in string::trim, got {}", args[0])),
    }
}

fn starts_with_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function string::starts_with, got {}, want 2", args.len()))
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(p)) => Ok(Value::Bool(s.starts_with(p))),
        _ => Err(format!("not a string in string::trim, got {}", args[0])),
    }
}

fn ends_with_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function string::ends_with, got {}, want 2", args.len()))
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(p)) => Ok(Value::Bool(s.ends_with(p))),
        _ => Err(format!("not a string in string::trim, got {}", args[0])),
    }
}

fn contains_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("too many arguments or too little for function string::contains, got {}, want 2", args.len()))
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(substr)) => Ok(Value::Bool(s.contains(substr))),
        (Value::String(_), _) => Err(format!("string::contains expects second argument to be a string, got {}", args[1])),
        _ => Err(format!("not a string in string::contains, got {}", args[0])),
    }
}

fn replace_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!("too many arguments or too little for function string::replace, got {}, want 3", args.len()));
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(from), Value::String(to)) => {
            Ok(Value::String(s.replace(from, to)))
        },
        (Value::String(_), Value::String(_), _) => {
            Err(format!("string::replace expects third argument to be a string, got {}", args[2]))
        },
        (Value::String(_), _, _) => {
            Err(format!("string::replace expects second argument to be a string, got {}", args[1]))
        },
        _ => Err(format!("string::replace expects first argument to be a string, got {}", args[0])),
    }
}

fn sub_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!("too many arguments or too little for function string::sub, got {}, want 3", args.len()));
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::Number(start), Value::Number(end)) => {
            let start_idx = *start as usize;
            let end_idx = *end as usize;

            if start_idx > s.len() || end_idx > s.len() || start_idx > end_idx {
                return Err("string::sub: invalid indices".to_string());
            }

            Ok(Value::String(s[start_idx..end_idx].to_string()))
        },
        (Value::String(_), Value::Number(_), _) => {
            Err(format!("string::sub expects third argument to be a number, got {}", args[2]))
        },
        (Value::String(_), _, _) => {
            Err(format!("string::sub expects second argument to be a number, got {}", args[1]))
        },
        _ => Err(format!("string::sub expects first argument to be a string, got {}", args[0])),
    }
}

fn format_nfn(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("string::format expects at least one argument".to_string());
    }

    let format_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("string::format expects first argument to be a string, got {}", args[0])),
    };

    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    let mut arg_index = 1;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if chars.peek() == Some(&'}') {
                chars.next(); // eat '}'
                
                if arg_index >= args.len() {
                    return Err("string::format: not enough arguments for format placeholders".to_string());
                }

                // convert 
                let arg_str = match &args[arg_index] {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => {
                        // make numbers look pretty
                        if n.fract() == 0.0 {
                            format!("{}", *n as i64)
                        } else {
                            format!("{}", n)
                        }
                    },
                    Value::Bool(b) => b.to_string(),
                    Value::Array(arr) => {
                        // [v1, v2, ...]
                        let elements: Vec<String> = arr.iter().map(|v| match v {
                            Value::String(s) => format!("\"{}\"", s),
                            Value::Number(n) => {
                                if n.fract() == 0.0 {
                                    format!("{}", *n as i64)
                                } else {
                                    format!("{}", n)
                                }
                            },
                            Value::Bool(b) => b.to_string(),
                            _ => format!("{:?}", v),
                        }).collect();
                        format!("[{}]", elements.join(", "))
                    },
                    _ => format!("{:?}", args[arg_index]), 
                };

                result.push_str(&arg_str);
                arg_index += 1;
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    if arg_index < args.len() {
        return Err("string::format: too many arguments for format placeholders".to_string());
    }

    Ok(Value::String(result))
}
pub const STRING_MOD: Module = Module {
    name: "string",
    funcs: &[
        ("len", len_nfn),
        ("upper", upper_nfn),
        ("lower", lower_nfn),
        ("trim", trim_nfn),
        ("starts_with", starts_with_nfn),
        ("ends_with", ends_with_nfn),
        ("contains", contains_nfn),
        ("replace", replace_nfn),
        ("sub", sub_nfn),
        ("parse", to_number_nfn),
        ("fmt", format_nfn)
    ],
};
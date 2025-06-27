use std::collections::HashMap;
use std::fs;
use crate::ast::Object;
use crate::interpreter::Value;
use crate::stdlib::Module;
use serde_json::{self, Value as JsonValue};

fn parse_nfn(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("too many arguments for json::parse, got {}", args.len()));
    }

    let content = match &args[0] {
        Value::String(s) => s,
        _ => return Err("expected a JSON string".to_string())
    };

    let parsed: JsonValue = serde_json::from_str(content.as_str()).map_err(|e| format!("error while parsing json: {}", e))?;

    fn json_to_value(json: JsonValue) -> Result<Value, String> {
        match json {
            JsonValue::Object(m) => {
                let mut hashmap = HashMap::new();
                for (key, value) in m {
                    hashmap.insert(key.to_string(), json_to_value(value)?);
                }
                Ok(Value::Object(hashmap))
            }
            JsonValue::Array(a) => {
                let v = a.into_iter().map(json_to_value).collect::<Result<Vec<Value>, String>>()?;
                Ok(Value::Array(v))
            }
            JsonValue::String(s) => Ok(Value::String(s)),
            JsonValue::Number(n) => Ok(Value::Number(n.as_f64().ok_or("invalid number format")?)),
            JsonValue::Bool(b) => Ok(Value::Bool(b)),
            JsonValue::Null => Ok(Value::Nil),
        }
    }

    json_to_value(parsed)
}

pub const JSON_MOD: Module = Module {
    name: "json",
    funcs: &[
        ("parse", crate::stdlib::json::parse_nfn),
    ],
};
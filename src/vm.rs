use std::collections::HashMap;

use crate::value::Value;

pub struct ExeState {
    globals: HashMap<String, Value>,
    constants: Vec<String>,
}

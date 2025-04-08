use std::collections::HashMap;

use crate::{
    bytecode::ByteCode,
    parse::ParseProto,
    value::{lib_print, Value},
};

pub struct ExeState {
    pub(crate) globals: HashMap<String, Value>,
    pub(crate) stack: Vec<Value>,
}

impl ExeState {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        globals.insert("print".into(), Value::Function(lib_print));
        Self {
            globals: globals,
            stack: vec![],
        }
    }

    pub(crate) fn execute(&mut self, proto: &ParseProto) {
        for code in proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, index) => {
                    let name = &proto.constants[index as usize];
                    if let Value::String(name) = name {
                        let value = self.globals.get(name).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, value);
                    }
                }
                ByteCode::LoadConst(dst, index) => {
                    let value = proto
                        .constants
                        .get(index as usize)
                        .unwrap_or(&Value::Nil)
                        .clone();
                    self.set_stack(dst, value);
                }
                ByteCode::Call(func_idx, _no_args) => {
                    if let Some(func) = &self.stack.get(func_idx as usize) {
                        if let Value::Function(f) = func {
                            f(self);
                            return;
                        } else {
                            panic!("Invalid function");
                        }
                    } else {
                        panic!("Invalid register")
                    }
                }
            }
        }
    }

    fn set_stack(&mut self, dst: u8, value: Value) {
        self.stack.insert(dst as usize, value.clone());
    }
}

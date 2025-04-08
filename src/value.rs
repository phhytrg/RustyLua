use std::fmt::Debug;

use crate::vm::ExeState;

/// Type in Lua is bound to value
#[derive(Clone)]
pub enum Value {
    Nil,
    String(String),
    Function(fn(&mut ExeState) -> i32),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
        }
    }
}

pub fn lib_print(exe: &mut ExeState) -> i32 {
    println!("{:?}", exe.stack[1]);
    0
}

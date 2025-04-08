use crate::{
    bytecode::ByteCode,
    lex::{Lex, Token},
    value::Value,
};

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}

pub(crate) fn load(input: std::fs::File) -> ParseProto {
    let mut constants = vec![];
    let mut byte_codes = vec![];
    let mut lex = Lex::new(input);

    while let Some(token) = lex.next() {
        match token {
            Token::Name(name) => match lex.next() {
                Some(s) => {
                    constants.push(Value::String(name));
                    byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));
                    match s {
                        Token::String(s) => {
                            constants.push(Value::String(s));
                            byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                            byte_codes.push(ByteCode::Call(0, 1));
                        }
                        _ => panic!("Unexpected token"),
                    }
                }
                None => break,
            },
            Token::String(_) => panic!("Not yet support"),
        }
    }

    ParseProto {
        constants,
        byte_codes,
    }
}

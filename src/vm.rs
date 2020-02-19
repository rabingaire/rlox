use crate::chunk;
use crate::value;

#[allow(non_camel_case_types)]
pub enum InterpretResult {
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR,
}

pub struct VM {
  chunk: chunk::Chunk,
}

impl VM {
  pub fn new() -> VM {
    VM {
      chunk: chunk::Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: value::ValueArray { values: Vec::new() },
      },
    }
  }
  pub fn interpret(&mut self, c: chunk::Chunk) -> InterpretResult {
    self.chunk = c;
    InterpretResult::INTERPRET_OK
  }
}

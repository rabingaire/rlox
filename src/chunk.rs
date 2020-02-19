use crate::value;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum OpCode {
  OP_CONSTANT(usize),
  OP_RETURN,
}

#[derive(Clone)]
pub struct Chunk {
  pub code: Vec<OpCode>,
  pub lines: Vec<u32>,
  pub constants: value::ValueArray,
}

// impl of Chunk
impl Chunk {
  pub fn write(&mut self, op: OpCode, line: u32) {
    self.code.push(op);
    self.lines.push(line);
  }
}

pub fn add_constant(c: &mut Chunk, v: value::Value) -> usize {
  let index = c.constants.values.len();
  c.constants.values.push(v);
  index
}

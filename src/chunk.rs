use crate::value;

#[allow(non_camel_case_types)]
pub enum OpCode {
  OP_CONSTANT,
  OP_RETURN,
}

pub struct Chunk {
  pub code: Vec<usize>,
  pub constants: value::ValueArray,
}

pub fn add_constant(c: &mut Chunk, v: value::Value) -> usize {
  let index = c.constants.values.len();
  c.constants.values.push(v);
  index
}

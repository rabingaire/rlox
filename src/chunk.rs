#[allow(non_camel_case_types)]
pub enum OpCode {
  OP_RETURN,
  OP_INVALID,
}

pub struct Chunk {
  pub code: Vec<OpCode>,
}

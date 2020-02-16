use crate::chunk;

pub fn disassemble_chunk(c: &chunk::Chunk, name: &str) {
  println!("== {} ==", name);
  let mut i = 0;
  while i < c.code.len() {
    i = disassemble_instruction(c, i);
  }
}

#[allow(unreachable_patterns)]
pub fn disassemble_instruction(c: &chunk::Chunk, offset: usize) -> usize {
  print!("{:04} ", offset);
  let instruction = &c.code[offset];
  match instruction {
    chunk::OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
    _ => {
      // Print opcode
      println!("Unknown opcode");
      offset + 1
    }
  }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
  print!("{}\n", name);
  return offset + 1;
}

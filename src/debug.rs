use crate::chunk;
use crate::value;

pub fn disassemble_chunk(c: &chunk::Chunk, name: &str) {
  println!("== {} ==", name);
  let mut i = 0;
  while i < c.code.len() {
    i = disassemble_instruction(c, i);
  }
}

#[allow(unreachable_patterns)]
fn disassemble_instruction(c: &chunk::Chunk, offset: usize) -> usize {
  print!("{:04} ", offset);

  if offset > 0 && c.lines[offset] == c.lines[offset - 1] {
    print!("   | ");
  } else {
    print!("{:4}", c.lines[offset]);
  }

  let instruction = &c.code[offset];
  match instruction {
    // TODO:: this need to be refactored right now I assume the position of
    // op_code on enum definition and match but we need some eligent construct
    0 => constant_instruction("OP_CONSTANT", c, offset),
    1 => simple_instruction("OP_RETURN", offset),
    _ => {
      // Print opcode
      println!("Unknown opcode");
      offset + 1
    }
  }
}

fn constant_instruction(name: &str, c: &chunk::Chunk, offset: usize) -> usize {
  let constant = c.code[offset + 1] as usize;
  print!("{:>16} {:4} ", name, offset);
  value::print_value(&c.constants.values[constant]);
  return offset + 2;
}

fn simple_instruction(name: &str, offset: usize) -> usize {
  print!("{}\n", name);
  return offset + 1;
}

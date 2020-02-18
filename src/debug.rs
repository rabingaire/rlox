use crate::chunk;
use crate::value;

pub fn disassemble_chunk(c: &chunk::Chunk, name: &str) {
  println!("== {} ==", name);
  for (offset, _) in c.code.iter().enumerate() {
    disassemble_instruction(c, offset)
  }
}

#[allow(unreachable_patterns)]
fn disassemble_instruction(c: &chunk::Chunk, offset: usize) {
  print!("{:04} ", offset);

  if offset > 0 && c.lines[offset] == c.lines[offset - 1] {
    print!("   | ");
  } else {
    print!("{:4}", c.lines[offset]);
  }

  let instruction = &c.code[offset];
  match instruction {
    chunk::OpCode::OP_CONSTANT(constant) => {
      constant_instruction("OP_CONSTANT", c, offset, *constant)
    }
    chunk::OpCode::OP_RETURN => simple_instruction("OP_RETURN"),
    _ => println!("Unknown opcode"),
  }
}

fn constant_instruction(name: &str, c: &chunk::Chunk, offset: usize, constant: usize) {
  print!("{:>16} {:4} ", name, offset);
  value::print_value(&c.constants.values[constant]);
}

fn simple_instruction(name: &str) {
  print!("{}\n", name);
}

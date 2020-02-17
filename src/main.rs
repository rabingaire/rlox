mod chunk;
mod debug;
mod value;

fn main() {
    let mut c = chunk::Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: value::ValueArray { values: Vec::new() },
    };

    let constant = chunk::add_constant(&mut c, value::Value(1.2));
    c.write(chunk::OpCode::OP_CONSTANT as usize, 123);
    c.write(constant, 123);
    c.write(chunk::OpCode::OP_RETURN as usize, 123);

    debug::disassemble_chunk(&c, "test chunk");
}

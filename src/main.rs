mod chunk;
mod debug;

fn main() {
    let mut c = chunk::Chunk { code: Vec::new() };
    c.code.push(chunk::OpCode::OP_RETURN);
    c.code.push(chunk::OpCode::OP_INVALID);

    debug::disassemble_chunk(&c, "test chunk");
}

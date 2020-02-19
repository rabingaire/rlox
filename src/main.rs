mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut v = vm::VM::new();
    let mut c = chunk::Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: value::ValueArray { values: Vec::new() },
    };

    let constant = chunk::add_constant(&mut c, value::Value(1.2));
    c.write(chunk::OpCode::OP_CONSTANT(constant), 123);
    c.write(chunk::OpCode::OP_RETURN, 123);
    v.interpret(c.clone());
    debug::disassemble_chunk(&c, "test chunk");
}

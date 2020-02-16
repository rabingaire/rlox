#[derive(Debug)]
pub struct Value(pub f64);

pub struct ValueArray {
  pub values: Vec<Value>,
}

pub fn print_value(value: &Value) {
  println!("{:?}", value);
}

#[derive(Debug, Clone)]
pub struct Value(pub f64);

#[derive(Clone)]
pub struct ValueArray {
  pub values: Vec<Value>,
}

pub fn print_value(value: &Value) {
  println!("{:?}", value);
}

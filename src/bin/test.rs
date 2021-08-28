use std::env;
use std::env::args;
use typename::TypeName;

fn main() {
  println!("{:?}: {}", args(), args().nth(1).type_name_of());
  println!("{}", args().nth(2).expect("must entered number"));
  println!("{}", env::var("NODE_ENV").expect("must create variable NODE_ENV"));
}
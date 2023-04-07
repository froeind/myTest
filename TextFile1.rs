fn main() {
  let a:u32 = 42;
  let b = a;
  let c:usize = 23;
  let d = 'x';
  let e = false;
 
  println!("a: {}", std::mem::size_of_val(&a));
  println!("b: {}", std::mem::size_of_val(&b));
  println!("c: {}", std::mem::size_of_val(&c));
  println!("d: {}", std::mem::size_of_val(&d));
  println!("e: {}", std::mem::size_of_val(&e));
}
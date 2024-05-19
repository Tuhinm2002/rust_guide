fn main(){
    // let x = 32; this will give error for not being mutable
  let mut x = 32;
  println!("The value of x is: {x}");
  x = 42;
  println!("The new value is: {}",x);

}
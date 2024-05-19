use::std::i32;

pub fn test_closure() {
    let _add= |x:i32| print!("{}",x);
    _add(6 as i32);
}

fn main(){
    test_closure();
    let sub_ = |x:i32,y:i32| print!("{}",x-y);
    sub_(5 as i32,6 as i32); 
}
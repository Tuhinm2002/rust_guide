fn print_string(str : &String){
    println!("{}",str);
}

fn main(){
    let str = String::from("hello world");
    print_string(&str);
}
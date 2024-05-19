fn main(){
    let array = [1,2,3];
    for a in array.iter(){
        print!("{}",a);
    }

    new_iter(array)
}

fn new_iter(array:[u32;3]){
    print!("\n");
    println!("New array");
for a in array{
    print!("{}",a);
}
}
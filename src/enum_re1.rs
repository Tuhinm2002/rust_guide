#[allow(unused)]
enum Mouse{
    leftClick,
    rightClick,
    Scroll(i32),
    Move(i32,i32)
}

fn print_match_val(values: Mouse){
    match values {
        Mouse::leftClick => print!("{}","Moving left (calling from function) "),
        Mouse::rightClick => print!("{}","Moving right (calling from function) "),
        Mouse::Scroll(val) => print!("{} (calling from function) ",val),
        Mouse::Move(left,right) => print!("{} {} (calling from function) ",left,right),
    }
}

fn main() {

    let left = Mouse::leftClick;
    match left {
        Mouse::leftClick => print!("{}","Moving left"),
        Mouse::rightClick => print!("{}","Moving right"),
        Mouse::Scroll(val) => print!("{}",val),
        Mouse::Move(left,right) => print!("{} {}",left,right),
    }
    println!(" ");
    print_match_val(left);
    println!(" ");
    let move_mouse = Mouse::Move(55,-155);
    print_match_val(move_mouse);
}
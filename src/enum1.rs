#[allow(unused)]

enum Directions{
    Right,
    Left
}

// fn print_directions(go: &Directions){
    
// }

fn main(){
    let go = Directions::Left;

    match go {
        Directions::Left => println!("Going left"),
        Directions::Right => println!("Going Right")
    }
    // print_directions(go);
}
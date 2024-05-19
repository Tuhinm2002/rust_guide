#[allow(unused)]

fn main(){
    let x = [1,2,3];
    let y = x.map(|v| v + 1);
    let b = assert_eq!(y,[2,3,4]);
    // println!("{:?}",b);
    for a in y {
        println!("{}",a);
    }
}
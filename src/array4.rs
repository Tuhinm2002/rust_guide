#[allow(unused)]

fn main(){
    const X:usize = 5;
    let mut arr: [u32;X] = [0;5];
    for a in arr.iter(){
        print!("{}",a);
    }
    println!("\n");
    // let new_arr: [u32;5] = todo!();
    for c in arr.iter_mut().enumerate(){
        let (i,_ele) = c;
        *_ele = 7;
    }
    print!("{:?}",arr);
}
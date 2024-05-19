fn change_and_print(arr : &mut[i32;5]){
    for a in &arr.iter_mut().enumerate(){
        let (i,_x) = a;
        arr[i] = *_x as i32;
    }
}

fn main(){
    let mut arr:[i32;5] = [0;5];
    println!("{:?}",arr);
    change_and_print(&mut arr);
}
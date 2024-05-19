pub fn print_arr(arr:&mut[[u32;5];5]){
    println!("{:?}",arr);
}

pub fn change_val_arr(arr:&mut[[u32;5];5]){
    for rows in arr.iter_mut(){
        // double for loop needed
        for cols in rows.iter_mut(){
        // let i:&u32 = &rows[0];
        *cols = ((*cols * 10)+10) as u32;
        // ((i) * (2 as u32)) + 1 as u32
        
        }
    }

    println!("New arr {:?}",arr);
}

fn main(){
    let mut arr:[[u32;5];5] = [[0;5];5];
    print_arr(&mut arr);
    change_val_arr(&mut arr)
}
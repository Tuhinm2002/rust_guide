use std::u32;
use::std::vec::Vec;
#[allow(unused)]

fn check_empty(nums:&Vec<i32>){
match nums.is_empty(){
      true => print!("The vector is empty"),
    false => print!("The vector is not empty ! The elements are {:?}",nums)
    }
}

fn check_parts(nums:&Vec<i32>){
    match nums.is_empty() {
        true => print!("The part of the vector is empty"),
    false => print!("The part of the vector is not empty ! The elements are {:?}",nums)
    }
} // Didn't work

fn main(){
let nums:Vec<i32> = vec![1,2,3];
    let (first_part,_second_part)= nums.split_at(1);
    check_empty(&nums);
    // check_parts(first_part);
    println!("{:?}",first_part);
}
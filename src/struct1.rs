

struct Details{
    name:String,
    age:u32,
    email:String
}

fn print_items(usr1 : Details){
        println!("{}",usr1.name);

}

fn main(){
    let usr1 = Details{
        name : String::from("Tuhin"),
        age : 21,
        email : String::from("tuhinm@hotail.com")
    };

    print_items(usr1);
}
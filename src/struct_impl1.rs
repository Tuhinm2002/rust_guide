struct Details{
    Name : String,
    Age:u32,
    Email:String

}

impl Details{
    fn print_details(&self){
        println!("Name {}",self.Name);
        println!("Age {}",self.Age);
        println!("Email {}",self.Email);
    }
}

fn main(){
 let usr1 = Details{
    Name : String::from("Tuhin"),
    Age : 21,
    Email:String::from("tuhinm@gmail.com")
 };

 Details::print_details(&usr1);

}
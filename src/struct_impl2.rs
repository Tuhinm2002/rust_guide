struct Details{
    name:String,
    age:u32,
    email:String
}

impl Details{
    fn create_new_usr(name1:String,email1:String,age1:u32) -> Details {
        Details {
            name:String::from(name1),
            age:age1,
            email:String::from(email1)
        }
    }

    fn print_details(usr1 : Details){
        println!("Name {}",usr1.name);
        println!("Email {}",usr1.email);
        println!("Age {}",usr1.age)
    }
}

fn main(){
    let usr1 = Details::create_new_usr("Tuhin".to_string(),"tuhinm@gmail.com".to_string(),21);
    Details::print_details(usr1);
}
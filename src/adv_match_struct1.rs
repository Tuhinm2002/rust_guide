#[allow(unused)]
struct Details {
    name:String,
    age:u32,
    email:String
}

impl Details {
    fn create_usr(usr_name:String,usr_age:u32,usr_email:String) -> Self{
        Self {
            name:usr_name,
            age:usr_age,
            email:usr_email
        }
    }

    fn print_usr_details(&self){
        println!("The name is {} ",self.name);
        println!("The age of the usr {} is {} ",self.name,self.age);
        println!("The email address of the usr {} is {} ",self.name,self.email);
    }

    fn match_usr_details(&self){
        match self{
            Details {age:18,..} => println!("The age of the usr {} is {} ",self.name,self.age),
            Details {email,..} => println!("The email is {} ",self.email)
        }
    }
}


fn main(){
    let usr1 = Details::create_usr("Tuhin".to_string(),21,"tuhinm@gmail.com".to_string());
    usr1.print_usr_details();
    println!("Advanced match statements");
    usr1.match_usr_details();
}
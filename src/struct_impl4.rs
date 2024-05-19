#[allow(unused)]
struct Store{
    name:String,
    price:u32
}

impl Store {
    fn create_store(name1:String,price1:u32)->Self{
        Self {
            name:name1,
            price:price1
        }
    }

    fn print_store_items(&self) {
        println!("Name of item is {}",self.name);
        println!("Price of item in $ {}",self.price)
    }
}

fn main(){
    let butter = Store::create_store("butter".to_string(),32);
    butter.print_store_items();
}
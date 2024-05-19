struct Store{
    item:String,
    price:u32
}

impl Store{
    fn create_item_list(item1 : String,price1 : u32) -> Store{
        Store {
            item : String::from(item1),
            price : price1
        }
    }

    fn print_value(cheese : Store){
        println!("Item Name {}",cheese.item);
        println!("Item Price in $ {}",cheese.price);
    }
}


fn main(){
    let cheese = Store::create_item_list("cheese".to_string(),32);
    Store::print_value(cheese);
}
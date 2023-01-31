use crate::entity::Sale;

#[path = "../src/entity/entity.rs"] mod entity;

pub fn main() {
    let mut sale = Sale::new(String::from("my_sale"));

    sale.add_item(String::from("item_name"), 100.00);

    println!("{:?}", sale.name);
}

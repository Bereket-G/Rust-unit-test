use std::ops::Add;

#[derive(Clone, PartialEq, Debug)]
pub struct Item {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, PartialEq)]
pub struct Sale {
    pub name: String,
    pub discount_grant: DiscountGrant,
    pub balance: f32,
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq)]
pub struct DiscountGrant {
    pub name: String,
    pub percent: f32,
}


impl Sale {
    pub fn new(name: String) -> Self {
        Sale {
            name,
            balance: 0.0,
            items: vec![],
            discount_grant: DiscountGrant {
                percent: 0.0,
                name: String::from(""),
            },
        }
    }

    pub fn add_item(&mut self, name: String, price: f32) {
        self.items.push( Item { name, price });
        self.balance = self.balance.add(price);
    }

    pub fn get_item(&mut self, name: String) -> Item {
        let found_item = self.items.iter().find(|x1| { x1.name == name }).unwrap();
        found_item.clone()
    }

    pub fn get_total_balance(&self) -> f32 {
        self.balance - (self.balance * self.discount_grant.percent)
    }

    pub fn apply_discount(&mut self, discount_grant: String, discount: &f32) {
        self.discount_grant.name = discount_grant;
        self.discount_grant.percent = *discount;
    }
}

#[path = "../src/main.rs"]
mod main;
#[path = "../src/entity/entity.rs"]
mod entity;

#[cfg(test)]
mod tests {
    use crate::entity::{Sale, DiscountGrant};
    use crate::entity::Item;


    fn before_each() -> (Sale, Item) {
        let sale = Sale::new(String::from("test_card"));
        let item = Item {
            name: String::from("item_name"),
            price: 5.0,
        };
        (sale, item)
    }

    #[test]
    fn test_add_item_with_success() {
        let (mut sale, item) = before_each();
        sale.add_item(item.clone().name, item.clone().price);
        assert_eq!(sale.items.len(), 1);
    }

    #[test]
    fn test_add_item_increase_sale_balance() {
        let (mut sale, item) = before_each();
        sale.add_item(item.clone().name, item.clone().price);
        assert_eq!(item.price, sale.balance);
    }

    #[test]
    fn test_get_item_with_success() {
        let (mut sale, item) = before_each();
        sale.add_item(item.clone().name, item.clone().price);
        let fetched_item = sale.get_item(item.name.clone());
        assert_eq!(item.name, fetched_item.name);
    }

    #[test]
    fn test_get_total_balance_when_no_discount() {
        let (mut sale, item) = before_each();
        sale.add_item(item.clone().name, item.clone().price);
        assert_eq!(sale.balance, sale.get_total_balance());
    }

    #[test]
    fn test_set_discount_grant() {
        let discount_grant = DiscountGrant {
            percent: 0.1,
            name: String::from("10PERCENT0FF"),
        };
        let (mut sale, item) = before_each();

        sale.add_item(item.clone().name, item.clone().price);
        sale.apply_discount(discount_grant.name.clone(), &discount_grant.percent);
        let total_balance = sale.get_total_balance();
        assert_ne!(total_balance, sale.balance);
    }

    #[test]
    fn test_get_total_balance_with_discount_grant_discount() {
        let (mut sale, item) = before_each();
        sale.add_item(item.clone().name, item.clone().price);
        let discount_grant = DiscountGrant {
            percent: 0.1,
            name: String::from("10PERCENT0FF"),
        };
        sale.apply_discount(discount_grant.name.clone(), &discount_grant.percent);
        let result = sale.balance - (sale.balance * discount_grant.percent);
        assert_eq!(sale.get_total_balance(), result);
    }
}

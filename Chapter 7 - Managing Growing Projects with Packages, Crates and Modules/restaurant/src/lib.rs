pub mod back_of_house;
mod front_of_house;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // relative
    front_of_house::hosting::add_to_waitlist();

    // bring path into scope
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with gluten-free toast
    let mut meal = back_of_house::Breakfast::summer("GF");

    // Change our mind about the type of bread
    meal.toast = String::from("White");
    println!("I'd like {} toast, please.", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

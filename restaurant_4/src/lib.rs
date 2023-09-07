mod back_of_house {
    pub enum Appetizer {//si la hacemos pública, todas sus variantes son públicas
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
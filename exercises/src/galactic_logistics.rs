use std::fmt;

//#[derive(Clone, Copy)] We don't need this if implementing From (below)
enum CargoCategory {
    Mineral,
    Medicine,
    Technology
}

impl fmt::Display for CargoCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CargoCategory::Mineral => write!(f, "Mineral"),
            CargoCategory::Medicine => write!(f, "Medicine"),
            CargoCategory::Technology => write!(f, "Technology"),
        }
    }
}

/*
    By implementing next function, we change the rules. We create an identical copy
 */
impl From<&CargoCategory> for CargoCategory {
    fn from(category_ref: &CargoCategory) -> Self {
        match category_ref {
            CargoCategory::Mineral => CargoCategory::Mineral,
            CargoCategory::Medicine => CargoCategory::Medicine,
            CargoCategory::Technology => CargoCategory::Technology,
        }
    }
}

struct CargoItem {
    name: String,
    category: CargoCategory,
    price_per_unit: f64
}

struct Ship {
    name: String,
    capacity: u32,
    cargo_hold: Vec<CargoItem>
}

struct SpaceStation {
    name: String,
    docked_ships: Vec<Ship>
}


fn print_ship_manifest(mut ship: Ship){

    for item in &mut ship.cargo_hold {//item is a reference
        let category = CargoCategory::from(&item.category);
        println!("Cargo Item: {}, Cargo Category: {}, Cargo Price per Unit: {}", item.name, category, item.price_per_unit );
    }
}

fn quality_check(){

}

fn docking(){

}

fn task_a() {
    let item_a = CargoItem {
        name: String::from("ESP32"),
        category: CargoCategory::Technology,
        price_per_unit: 3.0
    };
    let item_b = CargoItem {
        name: String::from("Ciclonithe"),
        category: CargoCategory::Medicine,
        price_per_unit: 5.0
    };

    let mut v: Vec<CargoItem> = Vec::new();
    v.push(item_a);
    v.push(item_b);


    let ship = Ship {
        name: String::from("Carabela de Colon"),
        capacity: 5,
        cargo_hold: v
    };

    print_ship_manifest(ship);

}
pub fn go(){
    task_a()
}
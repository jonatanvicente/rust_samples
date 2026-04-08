use std::fmt;
use std::ops::Index;

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

fn apply_space_wear(mut cargoItem:CargoItem){

    cargoItem.price_per_unit *= 0.9;
    println!("New price_per_unit: {}", cargoItem.price_per_unit);

}

fn dock_ship(station: &mut SpaceStation, ship: Ship) {
    station.docked_ships.push(ship);
}

fn transfer_cargo(source: &mut Ship, destination: &mut Ship, item_name: &str){

    //find the item in the source ship by name.
    let target_index: Option<usize> = source.cargo_hold.iter().position(|item| item.name == item_name);

    //remove it from the source
    if let Some(index) = target_index {
        let removed_ship = source.cargo_hold.remove(index);
        println!("Removed ship: {}", removed_ship.name);
        destination.cargo_hold.push(removed_ship);
        println!("Added ship: {}", destination.cargo_hold.index(0).name);
    }else {
        println!("Ship '{}' not found", item_name);
    }
}

fn calculate_total_value(station: &SpaceStation) -> f64 {
    let result:f64 = station.docked_ships.iter().flat_map(|ship| &ship.cargo_hold).map(|item| item.price_per_unit).sum();
    println!("Total value of cargo in station '{}': {}", station.name, result);
    result

}



pub fn go() {
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

    //
    let item_c = CargoItem {
        name: String::from("ESP32"),
        category: CargoCategory::Technology,
        price_per_unit: 3.0
    };

    apply_space_wear(item_c);

    //
    let mut spaceStation = SpaceStation {
        name: String::from("Galaktika"),
        docked_ships: Vec::new()
    };
    let ship = Ship {
        name: String::from("Union T962"),
        capacity: 5,
        cargo_hold: Vec::new()
    };
    dock_ship(&mut spaceStation, ship);
    //

    let uranium = CargoItem {
        name: String::from("Uranium"),
        category: CargoCategory::Mineral,
        price_per_unit: 95.0
    };
    let cargo_name = uranium.name.clone();

    let mut v: Vec<CargoItem> = Vec::new();
    v.push(uranium);

    let mut origin_ship = Ship {
        name: String::from("Aerospatial Train"),
        capacity: 5,
        cargo_hold: v
    };

    let mut destination_ship = Ship {
        name: String::from("Intergalactical Transporter"),
        capacity: 2,
        cargo_hold: Vec::new()
    };
    transfer_cargo(&mut origin_ship, &mut destination_ship, cargo_name.as_str());
    //
    calculate_total_value(&make_space_station());
    //
    refactoring();
}



fn refactoring() {

    //double borrow trap
    let space_station = make_space_station();

    let v_ships = space_station.docked_ships;
    let mut v: Vec<CargoItem> = v_ships.into_iter().flat_map(|ship| ship.cargo_hold).collect();
    for item in v {  // item is CargoItem (owned) - v is consumed
        apply_space_wear(item);
    }

}

fn make_space_station() -> SpaceStation {

    let item_a = CargoItem {
        name: String::from("Raspberry Pi"),
        category: CargoCategory::Technology,
        price_per_unit: 3.0
    };
    let item_b = CargoItem {
        name: String::from("Cianure"),
        category: CargoCategory::Medicine,
        price_per_unit: 5.0
    };

    let mut v: Vec<CargoItem> = Vec::new();
    v.push(item_a);
    v.push(item_b);

    let ship = Ship {
        name: String::from("Cargoship"),
        capacity: 5,
        cargo_hold: v
    };

    let mut v_ship: Vec<Ship> = Vec::new();
    v_ship.push(ship);

    let spaceStation = SpaceStation {
        name: String::from("Enterprise"),
        docked_ships: v_ship
    };

    spaceStation
}


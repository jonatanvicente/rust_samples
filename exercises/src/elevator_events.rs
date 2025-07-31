#![allow(dead_code)]

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
pub enum Event {
    Button_pressed(Button),
    Door_opened,
    Door_closed,
    Car_arrived(Floor),
}
type Floor = i32; // Floor number

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

/// The car has arrived on the given floor.
pub fn car_arrived(floor: i32) -> Event {
    Event::Car_arrived(floor)
}

/// The car doors have opened.
pub fn car_door_opened() -> Event {
    Event::Door_opened
}

/// The car doors have closed.
pub fn car_door_closed() -> Event {
    Event::Door_closed
}

/// A directional button was pressed in an elevator lobby on the given floor.
pub fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::Button_pressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::Button_pressed(Button::CarFloor(floor))
}

pub fn go(){
    println!(
        "A ground floor passenger has pressed the up button: {:?}", //necessary debug enabled
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}




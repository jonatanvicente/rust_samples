//Añadida env variable para mostrado de backtrace completa de error RUST_BACKTRACE=full

fn main() {
    panic!("crash and burn");
}
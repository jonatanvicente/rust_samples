

### cargo_workspace

- Distribución de directorios.
- Inclusión de crates en Cargo.toml de crate que precisa otro
- Un solo Cargo.lock asegura que se utiliza la misma versión de las dependencias en todos los crates
   - El Cargo.lock global contiene referencia a las versiones externas de librerías usadas en los crates.
   - Para usarlas, es necesario incluirlas en cada Cargo.toml de cada crate, que referencia al Cargo.lock global y se asegura de utilizar la misma versión de las dependencias en todos los crates.
  
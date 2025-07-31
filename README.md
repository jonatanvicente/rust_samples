
<img align="right" width="100" height="100" src="rust-logo-blk.svg">

# Samples RUST     

* **array_type**.
	- Uso de arrays
	- Mutable / immutable
	- Lectura de parámetros por teclado
* **cargo_workspace**.
    - Creación de workspace con Cargo
* **clone**.
	- Clonado de variable
* **constants**
	- Uso de mut (mutable)
* **control_flow**
	- Uso de variables mut
	- Uso de bucle loop
	- Contadores y ruptura de bucles
	- Distinct
	- Else if
	- Infinite loop
	- Loop con retorno
	- Loop con labels
	- While
	- Loop over collections
	- Reverse loop
* **closures**. Proyecto de ejemplo con closures.
* **<span style="color:red">closures_2</span>**. 
	- Análisis de closures.
    - Closures equivalentes, con y sin anotaciones. Inferencia de tipos.
* **<span style="color:red">closures_3</span>**.
	- Captura de referencias y valores in/out en closures (FnMut, FnOnce y Fn)
* **enums**
* **errors_recoverable**
	- Uso de Result <T, E> para obtención de mensajes de error
	- Uso de método unwrap_or_else para evitar match anidados	
* **errors_unrecoverable**
	- Invocación de macro panic!
	- Asignación de env var RUST_BACKTRACE=full para mostrado de traza completa
	- Closures
* **exercises**. Several exercises from the book "Comprehensive Rust" (https://google.github.io/comprehensive-rust/index.html)
    - Fibonacci Sequence
    - Nested Arrays (loops anidated)
    - Collatz Conjecture (loops and tests)
    - Geometry vectors (arrays desestructured as vectors)
    - Events (enums)
    - Expression Evaluation ()
* **floating_point**. Variables float
* **functions**. Declaración e invocaciones
* **functions_with_return**
* **guessing_game**
	- Importacion de librerías
	- Generador de números aleatorios
	- Lectura de entradas de teclado
	- Adición de dependencias en Cargo.toml
* **guessing_game_1**
	- Shadowing: reutilización de variables
	- Conversión de String en tipos numéricos
* **guessing_game_2**
	- Inferencia de tipos en comparación de variables	
* **guessing_game_3**
	- match
	- Instrucciones en caso de match
	- match: Result Ok y Err
* **hash_maps**. 
* **hash_maps_scopes**. 
	- Utilización de variables fuera de ámbito 
	- Inserción si key no existe
	- Borrado / sustitución de elementos
* **if_let**. Uso y comparaciones con Enums
* **iterators**. 
	- Creación y consumo de iterators. Mutabilidad de variables
    - Closures con iterators y consumo
* **lifetimes**. Ámbito de variables
* **<span style="color:red">lifetimes_functions</span>**. Ámbito de variables en su utilización con funciones y referencias
* **<span style="color:red">lifetimes_structs</span>**. Ámbito de variables en structs
* **<span style="color:red">lifetimes_elision</span>**. 
	- Análisis de lifetimes en funciones
    - Análisis de reglas del compilador para asignación de lifetimes
* **<span style="color:red">lifetimes_static</span>**.
    - Uso de lifetime 'static
	- Análisis de lifetimes en genéricos y traits unidos
* **match_control_flow**
	- Uso de None y Some (null no existe)
	- match
* **methods**
	- Instrucciones para hacer Debug
	- Struct, impl y contextos
	- Parámetro self
	- Porqué usar métodos en lugar de funciones
* **methods_several_params**
	- Múltiples implementaciones del mismo struct
* **<span style="color:purple">minigrep</span>**. Grep project that contains several concepts:
	- Organizing code
	- Using vectors and strings
	- Handling errors
	- Using traits and lifetimes where appropriate
	- Writing tests
	- Closures, iterators and trait objects.
* **<span style="color:purple">minigrep_2</span>**. 
* **<span style="color:purple">minigrep_3</span>**. Separation of Concerns for Binary Projects
* **<span style="color:purple">minigrep_4</span>**. 
	- Manejo de Err messages amigables
    - Uso de unwrap_or_else
    - Introducción a closures
* **<span style="color:purple">minigrep_5</span>**.
  	- Uso de Box<dyn Error> para manejar errores de forma genérica
	- Uso de operador ? en lugar de método expect
    - Uso de retorno Ok(()) cuando no hay nada específico que devolver
* **<span style="color:purple">minigrep_6</span>**. Evolución de código a una librería. Cambio de ámbito de funciones.
* **<span style="color:green">minigrep_tdd</span>**. 
	- Uso de #[allow(unused_variables)]
    - Definición de lifetimes
* **<span style="color:purple">minigrep_env</span>**.
    - Uso de variables de entorno (std::env) a partir de proyecto **minigrep_tdd**
* **<span style="color:purple">minigrep_errs</span>**.
	- Evolución de minigrep_env utilizando standard_out y standard_error
* **<span style="color:purple">minigrep_iterator</span>**.
    - Evolución de minigrep_env utilizando iterators
* **numeric_operations**. Suma, división, resta, multiplicación
* **ownership**
	- Move
	- Proceso en memoria
	- Drop function (variable sale de scope)
	- Punteros
	- Limpieza de variables
	- Uso de copy
* **ownership_functions**
	- Entrada / salida de scope de variables
	- Invocación automática de Drop
* **ownership_mutable_references**
	- Variables mutables / immutables
	- Referencias a variables mut
	- Creación de nuevos ámbitos
	- Scope de referencias
	- Punteros colgantes y variables borrowed (prestadas)
* **ownership_parameters**
	- Retorno de valores múltiples
* **ownership_references**
	- Referencias (punteros)
	- Parámetros como referencia
	- Mutable / immutable
	- Owner de un valor / Referencia al valor (puntero)
* **ownership_return_scope**
	- Owner de una variable
	- Casuística de invocación automática drop
	- Scope de variables de retorno
* **profiles**. Uso de profiles disponibles en Cargo.
* **project_1**
	- Invocación a println
	- Llamada a macros o a functions
* **project_2**. Ejemplo de proyecto
* **pub_use**
	- Importación de módulos en Cargo.toml
	- Uso de pub use en lib.rs para utilización pública de un módulo descrito 
	- Importación del módulo desde main.rs
* **restaurant**
	- Creación y uso de módulos. Module tree de ejemplo
* **restaurant_2**. Creación de crate (lib) e invocación a módulos de librería
* **restaurant_3**. Librería
	- Struct con campos públicos y privados
	- Módulos e implementación de struct
* **restaurant_4**. Enum pública / privada en módulo
* **several_modules**.
	- Creación de lib con módulo.
	- Importación del módulo front_of_house desde lib.rs
	- Estructura de folders para la visibilidad de módulo y función.
* **shadowing**.
	- Uso de let
	- Reutilización de variables
	- Ámbitos con curly brackets
* **slices**
	- Slices como tipos de referencia en lugar de referencia a Collections completas
* **slices_int**. Uso de slices con tipos int
* **slices_parameters**. 
	- Uso de variables / referencias y slices equivalentes
* **<span style="color:purple">smart_pointers_box</span>**
	- Smart Pointer: Box < T >
* **<span style="color:purple">smart_pointers_deref</span>**
    - Referenciar / Dereferenciar punteros (**deref**)
    - Deref Coercion
* **<span style="color:purple">smart_pointers_drop</span>**
	- Personalización de un valor cuando sale de ámbito
* **statements_expressions**
	- No retornan values
	- Uso de sentencias / funciones
* **strings**
	- Creación, append y diversas funciones de Strings
	- Enunciados de ejercicios en https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary
* **strings_iterating**. 
	- Iteración sobre elementos de un String
	- Ejemplo de que Unicode scalar values pueden almacenarse en más de 1 byte cada char (see https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)
* **strings_slicing**. Segmentación de un String
* **struct_dgb_macro_print**
	- Uso de debug. Retorno del owner de values
* **struct_full_example**. Ejemplo de un programa que calcula el área de un rectángulo
* **struct_full_example_structs**
	- Debug
	- Uso de referencias y evitar move
* **struct_full_example_tuples**. Uso de tuplas
* **struct_init**. Uso de structs
* **struct_tuple**. Structs con y sin campos
* **<span style="color:green">tests</span>**. Uso general de tests
* **<span style="color:green">tests_order</span>**. 
  * Ejecución de tests en orden pretendido, ignore de otros, mostrado de output,etc
* **<span style="color:green">tests_panic</span>**. Captura de panic! en tests
* **<span style="color:green">test_panic_2</span>**. Captura de mensajes de error de la app y mostrado de panic!
* **<span style="color:green">test_panic_3</span>**. Uso de Result en lugar de panic!
* **<span style="color:red">traits</span>**. Trait con importación de crates.
* **<span style="color:red">traits_default_implementation</span>**. Trait con implementación de métodos por default
* **typle_type**
	- Uso de diferentes tipos en tuplas
	- Desestructuración de tuplas
* **use_keyword**
	- crates
	- Referencia de tipos con el mismo nombre (importaciones)
	- Módulos e invocación de funciones de módulos
* **variables**. Tipos de variables
* **vectors**. Declaración y uso de vectores
* **vectors_enum**. Uso de enums para construir vectores con elementos de distinto tipo


See also [Comprehensive Rust Guide](https://google.github.io/comprehensive-rust/index.html) for exercises and explanations.



[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_EN.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_ES.md)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct_CA.md) 




### Smart Pointers: deref

* Referenciar / Dereferenciar punteros: comportamiento por defecto
* Creación de un tipo que se comporta como un puntero, implementación propia de deref
* Implementación del trait Deref Coercion:
     - Convierte un referencia de un tipo **que implemente Deref** en una de otro tipo. P. ej. puede convertir &String a &str
       (String implementa Deref)
* Rust ejecuta la Deref Coercion cuando:
     - **From &T to &U when T: Deref<Target=U>**
     - **From &mut T to &mut U when T: DerefMut<Target=U>**
     - **From &mut T to &U when T: Deref<Target=U>**
     - 1os 2 casos son iguales (el 2o implica mutabilidad)
     - 3er caso: Rust pasará ref mut a inmutable. REverso NO es posible: referencias inmutables nunca pueden pasar a mutables.




# Lifetimes elision

El compilador usa **3 reglas** para fijar el lifetime de una referencia cuando no se especifica explícitamente. Esto se llama elision de lifetimes.

#### 1ª regla

* Aplica a **input lifetimes** (argumentos de una función).
* El compilador asigna un lifetime parameter a cada parametro que es una referencia. Es decir,
una función con un parámetro obtiene un lifetime parameter, dos parámetros obtienen dos lifetime parameters, etc.
```
fn first_word(s: &str) -> &str {

//pasa a ser ...

fn first_word<'a>(s: &'a str) -> &str {
```

#### 2ª regla

* Aplica a **output lifetimes**
* Si hay un input lifetime parameter -> se asigna a todos los output lifetime parameters.
```
//aplicando la segunda regla, el ejemplo anterior pasa a ser ...

fn first_word<'a>(s: &'a str) -> &'a str {
```

#### 3ª regla

* Aplica a output lifetimes
* **Sólo se aplica a las firmas de métodos**
* Si hay multiple input lifetime parameters, pero uno de ellos es &self o &mut self (porque es un método) -> el 
lifetime de self se asigna a todos los output lifetime parameters.

Si el compilador llega al final y aún hay referencias sin lifetimes, el compilador da error.
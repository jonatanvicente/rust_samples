

## Static Lifetime

'static denota que la referencia afectada puede vivir la entera duración del programa. Todas las cadenas literales tienen
el lifetime 'static.

```
let s: &'static str = "I have a static lifetime.";

```
El texto es almacenado en los binarios del programa -> siempre estará accesible.
Por eso, el lifetime de todas las cadenas literales es 'static.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

* Véase longest_with_an_announcement en lib.rs
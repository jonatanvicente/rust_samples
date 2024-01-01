


### Box < T >

Los usaremos con mayor frecuencia:
* Cuando tenemos un tipo cuyo tamaño no se puede conocer tiempo de compilación y deseamos utilizar un valor de ese tipo 
en un contexto que requiere un tamaño exacto
* Cuando tenemos una gran cantidad de datos y deseamos transferir el ownership, asegurándonos de que los datos no se 
copien cuando lo hagamos
* Cuando queremos poseer un valor y solo nos importa que sea un tipo que implemente un trait particular en lugar de ser 
de un tipo específico

### Recursive Type
* Puede tener otro valor del mismo tipo como parte de sí mismo.
* En teoría el anidamiento podría continuar indefinidamente. Plantean un problema porque en tiempo de compilación hace
falta saber cuánto ocupa un tipo.

#### Const List
* Similar a LinkedList
* Podemos construir const lists de pares recursivos. Cada item contiene el valor del item actual y el siguiente. 
El último elemento será Nil (no contiene valor, OJO no es un null).
* OJO porque aunque RUST sabe cuánto ocupa un puntero, no cuánto ocupa el valor de la variable
* La mayoría de las veces usaremos un Vector en su lugar.
* Box<T> es un smart pointer porque implementa los traits Deref (permite a los valores de Box ser tratados como 
referencias), y Drop (limpiar valores cuando el Box sale del ámbito).
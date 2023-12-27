#[test]
fn iterator_sum() {
    
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    
    //el método sum() itera por todos y efectúa la suma
    let total: i32 = v1_iter.sum();
    //no podremos usar vi_iter despues de llamar a sum porque sum toma la propiedad del iterator
    assert_eq!(total, 6);
}



#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    
    /*
        El Iterator DEBE SER MUT: la llamada a iterator cambia el estado interno (lo usa el iterator
        para mantener la referencia en la secuencia.
          - Si deseamos un iterator que tenga sea el propietario de v1 y retorne owned values -> llamar a into_iter en lugar de iter.
          - Si queremos iterar sobre referencias mutables, llamar iter_mut en lugar de iter.
     */
    let mut v1_iter = v1.iter();
    
    /*
        OJO el iterator devuelve referencias INMUTABLES
     */
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}



fn main() {

    simple_iterator();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    
}

fn simple_iterator(){
    //All Iterators implements Iterator trait
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn closure_make_iterator(){
    
    let v1: Vec<i32> = vec![1, 2, 3];

    //Compiler error: iterator adaptors are lazy, and we need to consume the iterator here.
    //v1.iter().map(|x| x + 1);
    
    /*
       Para evitarlo -> usaremos collect(): este método consume el iterator y coloca los valores resultantes
            en una Collection
     */
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}



#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

/*
   Esta función toma el ownership del vector y la size como parametros. Retorna un vector que contiene sólo shoes de la talla especificada.
 */
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    /*
       Filter tiene como argumento una closure:
       - La closure toma un item y devuelve un booleano
                - Si es true -> el valor será incluido en la iteración producida por el filter
                - Si es false -> el valor será descartado
       En este caso, retorna sólo los shoes que tienen la talla especificada:
          - into_iter() crea un iterator que toma el ownership de cada item del vector.
          - Luego llamamos a filter() para adaptar el iterator a uno nuevo que contenga sólo elementos para los cuales la closure devuelva true.
          - La closure captura shoe_size del entorno y compara con cada size, dejando sólo los coincidentes.
          - Llamamos a collect() para reunir los valores retornados por el iterator en un vector retornado por la function (ver el test más abajo).
     */
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        
        let in_my_size = shoes_in_size(shoes, 10);
        
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

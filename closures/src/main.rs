#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    
    /*
            USO DE CLOSURE:
              - Tomamos la preferencia del usuario de tipo Option<ShirtColor>, llamando a unwrap_or_else en user_preference.
     */
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
             unwrap_or_else -> closure without any arguments that returns a T value:
                 - If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some.
                 - If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.
             
           || self.most_stocked() ->  closure sin parametros (si los tuviera, irían entre las dos ||).
           La implementación de unwrap_or_else evaluará la closure después si el resultado es necesario.
         */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    
    /*
      Invocación. La librería no necesita saber nada sobre Inventory o ShirtColor, o la lógica.
      La closure captura una referencia inmutable a self (Inventory) y la pasa con el código especificado a unwrap_or_else.
      Las funciones no pueden hacer esto.
    */
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    distinct_number();
    else_if();
    if_in_a_let();
    //infinite_loop();
    loop_returning_value();
    loop_labels();
    conditional_loops_with_while();
    loop_over_collection();
    for_over_collection();
    reverse_loop();
}


fn distinct_number() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

}

fn else_if (){
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
}


fn if_in_a_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn infinite_loop() { //OJO
    loop {
        println!("again!");
    }
}

fn loop_returning_value() {
        let mut counter = 0;

        let result = loop { //result is a variable to hold the value returned from the loop
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
}


fn loop_labels() {
        let mut count = 0;
        'counting_up: loop {//label: nos permite aplicar sentencias break o continue a un loop anidado haciendo referencia a la tag
            //tag = '[tag]
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;//tag
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
}

fn conditional_loops_with_while() {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }
        println!("LIFTOFF!!!");
}

fn loop_over_collection() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is:{}", a[index]);

            index += 1;
        }
}

fn for_over_collection() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
}

fn reverse_loop() {
        for number in (1..4).rev() { //rev() = reverse range
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
}
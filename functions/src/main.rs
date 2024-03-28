fn main() {
    println!("Hello, world!");

    another_function();
    second_function(5);
    print_labeled_measurement(5, 'h');

    // assignment doesn't return value ie can't do a = b = c
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // return value is the final expressio in the block
    let z = weird_return();
    println!("The value of z is: {z}")
}

fn another_function() {
    println!("Another function.");
}


fn second_function(x: i32) {
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn weird_return() -> i32 {
    5
}
fn main() {
    println!("Hello, world!");
    another_function(17);
    print_labeled_measurement(25, 'c');
    this_is_an_expression();
    let result = five();
    println!("The value of result is {result}");
}

fn another_function(num: i32) {
    println!("The value of the argument is {num}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// expressions
fn this_is_an_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}")
}

fn five() -> i32 {
    5 // this is an expression, no semicolon
}

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, z) = tup;
    println!("The value of y is {y}");
    println!("The value of z is {z}");
    // tup.0 = 5;

    let unit_tuple: () = ();
    println!("The value of unit_tuple is {unit_tuple:?}");
}

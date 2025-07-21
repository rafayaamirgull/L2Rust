fn main() {
    // Primitive data types
    // All variables in RUST are immutable by default
    let _x: i32 = 1;
    let _y: i32 = -2;
    let _z: f32 = 1.01;
    println!("x:{}, y:{}, z:{}", _x, _y, _z);

    let mut xmut: i32 = 1;
    // Mutable variables
    // but if you excede the limit (i32), you will get a runtime exception
    for i in 0..10 {
        xmut = xmut + i;
    }
    println!("xmut:{}", xmut);

    // Bool
    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true: {}, is_false: {}", is_true, is_false);

    // Strings
    // They don't have a fixed type/length
    // lenght is dynamic/can be changed at runtime
    let name: &str = "Rafay";
    let surname: &str = "Aamir";
    println!("name: {}, surname: {}", name, surname);

    let n_ch = 10;
    let charn = name.chars().nth(n_ch);
    // // not a good way
    // println!("name[{}]: {}", n_ch, charn.unwrap());

    // better way
    match charn {
        Some(c) => println!("name[{}]: {}", n_ch, c),
        None => println!("name[{}]: out of bounds", n_ch),
    }
}

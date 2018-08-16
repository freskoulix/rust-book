fn main() {
    let x: i8 = -128;
    println!("The value of x is: {}", x);
    let x: u8 = 255;
    println!("The value of x is: {}", x);

    let x: i16 = -32768;
    println!("The value of x is: {}", x);
    let x: u16 = 65535;
    println!("The value of x is: {}", x);

    let x: i32 = -2147483648;
    println!("The value of x is: {}", x);
    let x: u32 = 4294967295;
    println!("The value of x is: {}", x);

    let x: i64 = -922337203700000000;
    println!("The value of x is: {}", x);
    let x: u64 = 18446744070000000000;
    println!("The value of x is: {}", x);

    let x: i128 = -17014118350000000000000000000000000000;
    println!("The value of x is: {}", x);
    let x: u128 = 340282366900000000000000000000000000000;
    println!("The value of x is: {}", x);

    let x: isize = -922337203700000000;
    println!("The value of x is: {}", x);
    let x: usize = 18446744070000000000;
    println!("The value of x is: {}", x);

    let x: isize = -922337203700000000;
    println!("The value of x is: {}", x);
    let x: usize = 18446744070000000000;
    println!("The value of x is: {}", x);

    let x = 98_222;
    println!("The value of x is: {}", x);

    let x = 0xff;
    println!("The value of x is: {}", x);

    let x = 0o77;
    println!("The value of x is: {}", x);

    let x = 0b1111_0000;
    println!("The value of x is: {}", x);

    let x = b'A';
    println!("The value of x is: {}", x);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The values of x and y are: {}, {}", x, y);

    // addition
    let sum = 5 + 10;
    println!("sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is: {}", remainder);

    let t = true;
    println!("t value is: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("f value is: {}", f);

    let c = 'z';
    println!("c value is: {}", c);
    let z = 'ℤ';
    println!("z value is: {}", z);
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat value is: {}", heart_eyed_cat);

    let c = "\u{0000}";
    println!("c value is: {}", c);

    let c = "\u{d7ff}";
    println!("c value is: {}", c);

    let c = "\u{e000}";
    println!("c value is: {}", c);

    let c = "\u{10ffff}";
    println!("c value is: {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup value is: {:?}", tup);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("tuple values are: {}, {}, {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    println!("a values are: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("Months are: {:#?}", months);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a values are: {:?}", a);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first and second values are: {} and {}", first, second);
}

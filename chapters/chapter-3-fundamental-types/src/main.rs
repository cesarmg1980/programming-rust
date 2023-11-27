fn main() {
    println!("Chapter 3: Fundamental Types");
    println!("* You can explicitly type everything");
    println!("Printing Vector from 'build_vector_typed' --> {:?}", build_vector_typed());
    println!("* Or if the type is obvious you can leave the typing out");
    println!("Printing Vector from 'build_vector_not_typed' --> {:?}", build_vector_not_typed());
    assert_eq!(build_vector_typed(), build_vector_not_typed());

    println!();
    println!("Convert from one integer type to another");
    println!("10_i8 as u16 to 10_u16 --> in range");
    assert_eq!(10_i8 as u16, 10_u16); // in range

    println!("2525_u16 as i16 to 2525_i16 --> in range");
    assert_eq!(2525_u16 as i16, 2525_i16); // in range
    
    println!("-1_i16 as i32 to -1i32 --> sign extended");
    assert_eq!(-1_i16 as i32, -1i32);
    println!("65535_u16 as i32 to 65535_i32 --> sign extended");
    assert_eq!(65535_u16 as i32, 65535_i32);

    println!(r#"Conversions that are out of range for the destination
    produce values that are equivalent to the original
    modulo 2^N, where N is the width of the destination in bits, 
    this is called sometimes 'truncation'"#);
    println!("1000_i16 as u8 result in 232_u8");
    assert_eq!(1000_i16 as u8, 232_u8);
    println!("65535_u32 as i16 results in -1_i16");
    assert_eq!(65535_u32 as i16, -1_i16);
    println!("-1_i8 as u8 results in 255_u8");
    assert_eq!(-1_i8 as u8, 255_u8);
    println!("255_u8 as i8 results in -1_i8");
    assert_eq!(255_u8 as i8, -1_i8);

    println!("The standard library provides some operations as methods on integers");
    println!("2_u16.pow(4) is 16");
    assert_eq!(2_u16.pow(4), 16);
    println!("(-4_i32).abs() is 4");
    assert_eq!((-4_i32).abs(), 4);
    println!("0b101101_u8.count_ones() is 4");
    assert_eq!(0b101101_u8.count_ones(), 4);
    println!(r#"println!('{{}}', (-4).abs()); doesn't compile. 
    The reason is that Rust doesn't know -4 type (i16, i32)"#);
    print!("println!('{{}}', (-4_i32).abs()); --> ");
    println!("{}", (-4_i32).abs());
    print!("println!('{{}}', i32::abs(-4)); --> ");
    println!("{}", i32::abs(-4));

}

fn build_vector_typed() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector_not_typed() -> Vec<i16>{
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

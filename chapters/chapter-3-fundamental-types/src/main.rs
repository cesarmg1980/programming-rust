fn main() {
    example_fundamental_types();
    example_fixed_width_numeric_types();    
    example_checked_wrapping_saturating_and_overflowing_arithmetic();
    example_floating_point_types();
    example_tuples();
    arrays();
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

fn example_fundamental_types() {
    println!("Chapter 3: Fundamental Types");
    println!("* You can explicitly type everything");
    println!("Printing Vector from 'build_vector_typed' --> {:?}", build_vector_typed());
    println!("* Or if the type is obvious you can leave the typing out");
    println!("Printing Vector from 'build_vector_not_typed' --> {:?}", build_vector_not_typed());
    assert_eq!(build_vector_typed(), build_vector_not_typed());
}

fn example_fixed_width_numeric_types() {
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

fn example_checked_wrapping_saturating_and_overflowing_arithmetic() {
    println!();
    println!("Checked");
    println!("The sum of 10 and 20 can be represented as a u8");
    println!("assert_eq!(10_u8.checked_add(20), Some(30))");
    assert_eq!(10_u8.checked_add(20), Some(30));
    println!("The sum of 100 and 200 cannot");
    println!("assert_eq!(100_u8.checked_add(200), None)");
    assert_eq!(100_u8.checked_add(200), None);
    println!("Wrapping");
    println!("The first product can be represented as u16");
    println!("The second cannot");
    println!("assert_eq!(100_u16.wrapping_mul(200), 20000)");
    println!("assert_eq!(500_u16.wrapping_mul(500), 53392)");
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    println!("Operations on signed types may wrap to negative values");
    println!("assert_eq!(500_i16.wrapping_mul(500), -12144)");
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    println!("Saturating");
    println!("assert_eq!(32760_i16.saturating_add(10), 32767)");
    assert_eq!(32760_i16.saturating_add(10), 32767);
    println!("Overflowing");
    println!("assert_eq!(255_u8.overflowing_sub(2), (253, false))");
    println!("assert_eq!(255_u8.overflowing_add(2), (1, true))");
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}

fn example_floating_point_types() {
    println!();
    println!("Floating Point Types");
    println!("assert_eq!((-1. / f32::INFINITY).is_sign_negative())");
    assert!((-1. / f32::INFINITY).is_sign_negative());
    println!("assert_eq!(-f32.MIN, f32::MAX)");
    assert_eq!(-f32::MIN, f32::MAX);
    println!("assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5)");
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    println!("assert_eq!((-1.01f64).floor(), -2.0)");
    assert_eq!((-1.01f64).floor(), -2.0);
    println!("println!('{{}}', (2.0_f64).sqrt())");
    println!("{}", (2.0_f64).sqrt());
    println!("println!('{{}}', f64::sqrt(2.0))");
    println!("{}", f64::sqrt(2.0));
}

fn example_tuples() {
    println!();
    println!("Tuples");
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("assert_eq!(head, 'I see the igenvalue ')");
    assert_eq!(head, "I see the eigenvalue ");
    println!("assert_eq!(tail, 'in thine eye')");
    assert_eq!(tail, "in thine eye");
}

fn arrays() {
    println!();
    println!("Arrays");
    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 4);
    assert_eq!(taxonomy.len(), 3);
    println!("assert_eq!(lazy_caterer[3], 4)");
    println!("assert_eq!(taxonomy.len(), 3)");

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);
    println!("assert!(sieve[211])");
    println!("assert!(!sieve[9876])");
    
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
    println!("assert_eq!(chaos, [1, 2, 3, 4, 5])")
}

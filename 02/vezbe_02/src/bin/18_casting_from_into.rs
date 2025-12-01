use std::convert::{From, TryFrom, TryInto};

fn main() {
    // Example 1: Using From / Into for standard library types
    {
        // From<&str> for String is provided by std
        let s: &str = "hello";
        let string1 = String::from(s);
        let string2: String = s.into();  // Into<String> works because From<&str> ≤> Into<String>
        println!("string1 = {}, string2 = {}", string1, string2);
    }

    // Example 2: Primitive conversions between numeric types via From/Into
    {
        // many primitive conversions in std are implemented
        let x_u8: u8 = 100;
        let x_u16: u16 = x_u8.into(); // u8 -> u16: widening, infallible
        let x_u16: u16 = u16::from(x_u8); // equivalent using From
        println!("u8 -> u16: {} -> {}", x_u8, x_u16);

        let x_i32: i32 = 5000;
        let x_i64: i64 = x_i32.into(); // widening signed int
        let x_i64: i64 = i64::from(x_i32); // equivalent using From
        println!("i32 -> i64: {} -> {}", x_i32, x_i64);

        // Now we try to convert from a larger to a smaller type
        // let y_i16: i16 = 300;
        // let y_i8: i8 = y_i16.into(); // This line would not compile: no From<i16> for i8
    }

    // Example 3: Fallible conversion with TryFrom / TryInto
    {
        // Suppose we want to convert i64 → u8, but check for overflow
        let big: i64 = 1000;
        // This conversion may fail: 1000 > u8::MAX (255)
        let maybe_small: Result<u8, _> = u8::try_from(big);
        match maybe_small {
            Ok(v) => println!("Converted: {}", v),
            Err(e) => println!("Conversion failed: {:?} (value out of range)", e),
        }

        let small: i64 = 200;
        let maybe_small2: Result<u8, _> = small.try_into();
        println!("i64({}) → u8 via try_into(): {:?}", small, maybe_small2);
    }
}

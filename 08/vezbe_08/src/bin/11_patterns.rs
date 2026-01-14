enum Weather {
    Sunny,
    Cloudy,
    Rainy(Option<u8>),
}

fn main() {
    let today_weather = Weather::Rainy(Some(70));

    // =========================================================
    // IRREFUTABLE PATTERNS (always match)
    // =========================================================

    // Simple binding
    let x = 10;

    // Wildcard
    let _ = today_weather;

    // Tuple destructuring (always matches)
    let pair = (1, 2);
    let (a, b) = pair;

    // Function parameter (irrefutable)
    takes_irrefutable((3, 4));

    // for loop pattern (irrefutable)
    let numbers = vec![(1, 2), (3, 4)];
    for (x, y) in numbers {
        println!("x = {}, y = {}", x, y);
    }

    // =========================================================
    // REFUTABLE PATTERNS (may fail)
    // =========================================================

    // match: refutable arms, exhaustive overall
    match today_weather {
        Weather::Sunny => {
            println!("Sunny");
        }
        Weather::Cloudy => {
            println!("Cloudy");
        }
        Weather::Rainy(Some(intensity)) => {
            println!("Rainy: {}%", intensity);
        }
        Weather::Rainy(None) => {
            println!("Rainy: unknown intensity");
        }
    }

    // if let: refutable, optional execution
    if let Weather::Rainy(Some(intensity)) = today_weather {
        println!("Carry umbrella: {}%", intensity);
    }

    // while let: refutable loop condition
    let mut opt = Some(1);
    while let Some(v) = opt {
        println!("Value: {}", v);
        opt = None;
    }

    // Illegal: refutable pattern in `let`
    // let Weather::Rainy(Some(i)) = today_weather;
}

// Irrefutable parameter pattern
fn takes_irrefutable((x, y): (i32, i32)) {
    println!("x = {}, y = {}", x, y);
}

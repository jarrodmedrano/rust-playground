#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

fn create_car_colour() -> CarColour {
    let my_car_colour: CarColour = CarColour::Red;
    my_car_colour
}

// return string and string example of generics and enums
fn check_under_five(num_check: u8) -> GivenResult<String, String> {
    if num_check < 5 {
        GivenResult::Ok(String::from("Under five"))
    } else {
        GivenResult::Err(String::from("Over five"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums() {
        let car_colour: CarColour = create_car_colour();

        dbg!(car_colour);

        let is_under_five: GivenResult<String, String> = check_under_five(2);
        dbg!(is_under_five);

        let is_under_five: GivenResult<String, String> = check_under_five(5);
        dbg!(is_under_five);
    }
}

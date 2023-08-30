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

fn check_under_five_built_in(num_check: u8) -> Result<String, String> {
    if num_check < 5 {
        Ok(String::from("Under five"))
    } else {
        Err(String::from("Over five"))
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
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

        let remainder = remainder_zero(12.2);
        dbg!(remainder);
    }
}

use core::num;

//creating enums excerises
//tell rust that the enum is dubggable
#[derive(Debug)]
enum CarColour{
    Red,
    Green,
    Blue,
    Silver
}

//tell rust that the enum is dubggable
#[derive(Debug)]
//<>  means generic type return, in the case below, it returns 2 genertic type name T and E
//  will tell rust when using the enum wheteher T and E return type is,
//  when using it and give us the ability to change type along the way
enum GivenResult<T, E>{
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T>{
    None,
    Some(T)
}


//function that will return a car colour
fn handle_car_colour_blue() -> CarColour{
    let my_car_colour: CarColour = CarColour::Blue; //the enum CarColour becomes a type in this case
    my_car_colour
}

//telling the function of the return type is a enum and will return a u8 and a String for the both parameters for this use case
fn check_under_five(num_check: u8) -> GivenResult<u8, String>{
    if num_check < 5{
        GivenResult::Ok(num_check)

    }else{
        GivenResult::Err("Not Under 5!".to_string())
    }
}

//Rust builtin version of enum
fn check_under_five_built_in(num_check: u8) -> Result<u8, String>{
    if num_check < 5{
        Ok(num_check)

    }else{
        Err("Not Under 5!".to_string())
    }
}

//option enum
fn remainder_zero(num_check: f32) -> GivenOption<f32>{
    //retrn something or nothing
    let remainder = num_check% 10.0;
    if remainder != 0.0{
        GivenOption::Some(remainder)
    }else{
        GivenOption::None
    }
}

//Rust built in option enum
fn remainder_zero_built_in(num_check: f32) -> Option<f32>{
    //retrn something or nothing
    let remainder = num_check% 10.0;
    if remainder != 0.0{
        Some(remainder)
    }else{
        None
    }
}

//unit test
#[cfg(test)]
mod test{
    //use super to import everything
    use super::*;

    #[test]
    fn tests_enums(){
        let car_colour= handle_car_colour_blue();
        dbg!(car_colour);

        let under_five_res = check_under_five_built_in(2);
        dbg!(under_five_res);
        let under_five_res = check_under_five_built_in(7);
        dbg!(under_five_res);

        let remainder = remainder_zero_built_in(12.2);
        dbg!(remainder);
    }
}
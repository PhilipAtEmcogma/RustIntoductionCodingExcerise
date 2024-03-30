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

//unit test
#[cfg(test)]
mod test{
    //use super to import everything
    use super::*;

    #[test]
    fn tests_enums(){
        let car_colour= handle_car_colour_blue();
        dbg!(car_colour);

        let under_five_res = check_under_five(2);
        dbg!(under_five_res);
        let under_five_res = check_under_five(7);
        dbg!(under_five_res);
    }
}
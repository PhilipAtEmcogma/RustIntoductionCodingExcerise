//creating enums excerises
//tell rust that the enum is dubggable
#[derive(Debug)]
enum CarColour{
    Red,
    Green,
    Blue,
    Silver
}

//function that will return a car colour
fn handle_car_colour_blue() -> CarColour{
    let my_car_colour: CarColour = CarColour::Blue; //the enum CarColour becomes a type in this case
    my_car_colour
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
    }
}
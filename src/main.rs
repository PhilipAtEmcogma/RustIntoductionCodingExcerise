
const OUR_COURSE: &str = "Rust with AutoGPT"; //stored in static memory

fn main() {
    println!("Pratice proeject for {}", OUR_COURSE);

    //stack
    let x: i32;
    x =2;
    println!("x is {}", x);

    let y:i32 = 5;

    //For loop
    for i in 0..y{
        if i != 4 {
            print!("{}, ", i)
        } else {
            println!("{}", i);
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {}", z);
    z = 10;
    println!(", but z now is {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);


    let is_zero_remainder: bool = 10%4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '🤗';
    println!("emoji_char is {}", emoji_char);

    //array of floats
    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_float is {:?}", my_floats);

    //closures
    //take my_floats and add 2 to each number stored in there and put it into a new array called my_float_new
    let my_floats_new: [f32; 10] = my_floats.map(|n: f32| n+2.0);
    println!("my_float_new is {:?}", my_floats_new);




}
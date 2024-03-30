//importing the m1_enums.rs module
mod m1_enums;
mod m2_struct;
mod m3_traits;

//const OUR_COURSE: &str = "Rust with AutoGPT"; //stored in static memory

fn main() {
    // println!("Pratice proeject for {}", OUR_COURSE);

    // //stack
    // let x: i32;
    // x =2;
    // println!("x is {}", x);

    // let y:i32 = 5;

    // //For loop
    // for i in 0..y{
    //     if i != 4 {
    //         print!("{}, ", i)
    //     } else {
    //         println!("{}", i);
    //     }
    // }

    // // Mutation
    // let mut z: i32 = 5;
    // print!("z was {}", z);
    // z = 10;
    // println!(", but z now is {}", z);

    // let freezing_temp: f64 = -2.4;
    // println!("freezing_temp is {}", freezing_temp);


    // let is_zero_remainder: bool = 10%4 != 0;
    // println!("is_zero_remainder is {}", is_zero_remainder);

    // let my_char: char = 'z';
    // println!("my_char is {}", my_char);

    // let emoji_char: char = '🤗';
    // println!("emoji_char is {}", emoji_char);

    // //array of floats
    // let my_floats: [f32; 10] = [0.0; 10];
    // println!("my_float is {:?}", my_floats);

    // //closures
    // //take my_floats and add 2 to each number stored in there and put it into a new array called my_float_new
    // let my_floats_new: [f32; 10] = my_floats.map(|n: f32| n+2.0);
    // println!("my_float_new is {:?}", my_floats_new);

    /* 
    //String literal
    let name: &str = "Philip";
    println!("name is {:?}", name);

    //Dynamic size string variable
    let dynamic_name: String = String::from("Philip Wong");
    println!("dynamic_name is {:?}", dynamic_name);
    println!("my dynamic_name stored in memory {:p}", &dynamic_name); //p indicates pointer

    // let dynamic_name: String = name.to_string(); //converting string literal to dynamic string
    // println!("dynamic_name is {:?}", dynamic_name);
    // let dynamic_name: String = "Philip".to_string();
    // println!("dynamic_name is {:?}", dynamic_name);

    //slicing a string via copying array
    let str_slice: &str = &dynamic_name[0..6];
    println!("str_slice is {:?}", str_slice);

    // to insert a character into a vector, can use insert or push
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.insert(3, 'l');
    chars.push('o');
    chars.push('.');

    //println!("chars is {:?}", chars);
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap(); //use pop to remove character
    println!("removed_char is {:?}", removed_char);

    chars.iter().for_each(|c| print!("{}", c)); //instead of println, can also store into a new varaiable 

    let chars_again: Vec<char> = vec!('h', 'e', 'l', 'l', 'o'); //another way to create a string
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect(); //collect all the items in the vector and store it as a string
    dbg!(collected);

    for c in chars_again{
        print!("{}", c);
        if c == 'o'{
            println!(", world!");
        }
    }
    */

    //Closures
    // let num = 5;
    // let add_num = |x:i32| x+num; //implemented function, a closure
    // let new_num = add_num(7);
    // println!("{}", new_num);

    /* 
    //Number Literals and Raw Strings
    //Number literals (from Rust Book)
    //using _ as separators for big numbers is a features for Rust, which makes it easier to read
    println!("Big Number is {}", 98_222_000);
    //Hex Number
    println!("Hex is {}", 0xFFF);
    //Octo number
    println!("Octo is {}", 0o77);
    //Binary
    println!("Binary is {}", 0b111100001111);
    //Bytes
    println!("Butes for 'A' is {}", b'A');
    //Raw - String Literal
    let text = r#"{\"message" : "Rust is Awesome"}"#;
    println!("{}", text);
    */



}
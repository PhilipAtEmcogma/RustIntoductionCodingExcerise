#[derive(Debug)]
enum Message{
    Quit,
    ChangeColour(i32, i32, i32),
    Move{x:i32, y:i32},
    Write(String)
}

fn process_message(msg: Message){
    match msg{
        Message::Quit =>{
            println!("I Quit!");
        },
        Message::ChangeColour(red, green, blue) => {
            println!("Red {}, Green {}, Blue{}", red, green, blue);
        },
        Message::Move { x, y } => {
            println!("X is {}, Y is {}", x, y);
        },
        Message::Write(text) => {
            println!("{}", text);
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn tests_large_enum(){
        let my_quit = Message::Quit;
        let my_colour = Message::ChangeColour(10, 20, 30);
        let my_move = Message::Move { x: 40, y: 50 };
        let my_write = Message::Write("Hello World!".to_string());
        process_message(my_write)
    }

    #[test]
    fn tests_match_literals(){

        //reminder match is alot like switch
        let number = 20;

       let res = match number{
            //notice the =>, use in match.
            1 => "This is the first!",
            2|3|5|7|15|20 => "we found it in the sequence!",
            _ => "It was something else" //the _ means everything else that wasn't included above it, belongs here
        };
        
        print!("{}\n", res)
    }

    #[test]
    fn tests_match_option(){
        let some_num: Option<i32> = Some(20);

        let my_int: i32 = if let Some(i) = some_num{
            i
        } else{
            panic!("There was a problem!")
        };

        println!("My int: {}", my_int);

        // let res = match some_num{
        //     Some(i) =>i, //if its some return i
        //     None =>{
        //         panic!("There was a problem!")
        //     }
        // };

        // println!("{}", res)
    }

    #[test]
    fn tests_match_result(){
        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("These was an error");

        let res = match some_res{
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }; //puting a ; indicate it only return val within the code block

        println!("{}", res);
    }

    #[test]
    fn tests_match_guard(){
        let pair = (2, -2);
        match pair{
            (x, y) if x == y => println!("They match!"),
            (x, y) if x+y == 0 => println!("They neutralize"),
            (_, y) if y ==2 => println!("Y is indeed +2"), //don't care what the 1st parameter is , 
            _ => println!("We are not nothered") //any other combination, goes here
        };
    }

    #[test]
    fn tests_match_struct(){

        struct Location{
            x:i32,
            y:i32
        }

        let location = Location{x:0, y:20};

        match location{
            Location{x, y:0} => println!("Y is on the axis"),
            Location{x:0, y} => println!("X is on the axis"),
            Location{x, y} => println!("X and Y is on the axis"),
        }; //not returning anything
    }
}





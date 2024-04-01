

#[cfg(test)]
mod test{
    use super::*;

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
}





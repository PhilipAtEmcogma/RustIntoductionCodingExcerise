

#[cfg(test)]
mod tests{
    //use super::*;

    /*
        MACRO CAPTURES - below are some examples of Macro captures
            expr:
                matches to a valid rust expression
                    "hello".to_string(), vec![1,2,3], 1+2, 1

            stmt:
                matches to a rust statement
                    let x = 5, x.push(1), return Some(x)

            ident:
                matches to a rust identifier
                    variable name, function name, module name

            ty:
                mateches to a rust type
                    i32, Vec<String>, Option<T>

            path
                matches to a rust path
                    std::collections::HashMap
        
        REPITITION SPECIFIER
            * - match zero or more repititions
            + - match one or more repititions
            ? - match zero or one repitition
    
    */

    // macro_rules! mad_skills {
    //     //the input () is known as macro capture, is what we expect to recieve into the macro
    //     // ($x: expr) => {
    //     //     format!("You send an expression: {}", $x)
    //     // };

    //     ($x: ty) => {
    //         match stringify!($x){
    //             "i32" => "You sent an i32 type".to_string(),
    //             _ => "You sent something else".to_string(),
    //         }
    //     }
    // }

    //create a vector macro and put at least 1 variables in
    macro_rules! my_vec {
        //+ indicates at the user need to provide at least 1 reptition of macro captures, in the case below ($x:expr)
        ($($x:expr),+) => {
            {
                let mut temp_vec = Vec::new();

                //reptition block
                $(
                    temp_vec.push($x);
                )+

                temp_vec
            }
        };
    }
    #[test]
    fn tests_declarative_macro(){
        
        let mut x: Vec<i32> = vec!();
        let mut y = my_vec!(1);

        dbg!(y);

        //rust built in macro
        //let x = vec!(1,2,3);

        // let some_var = mad_skills!(1+2);
        // let some_var = mad_skills!(i32);
        // dbg!(some_var);

        //below is some example of the declarative macros, it can be identified with !
        // println!("hello");
        // dbg!("hello 2");
        // let x = vec![1,2,3];
        // let formatted = format!("Hello 3 with vec {:?}", x);
        // dbg!(formatted);
    }
}
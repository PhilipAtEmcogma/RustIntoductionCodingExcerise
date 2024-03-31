// fn example_0(){
//     let r;
//     {
//         let x = 5;
//         r = &x; 
//         //x does not live long engough error, its because as long as the program step out of the code block, 
//         //x will be out of scope and errased
//     }

//     println!("r: {}", r); //r is refference to x above, but since x will be out of scope after the code block above. 
//     //r will become a dangling reference
// }
//to sovle the lifetime problem above, simply remove the code block, and run the code without it
// fn example_0(){
//     let r;
//     let x = 5;
//     r = &x; 
//         //x does not live long engough error, its because as long as the program step out of the code block, 
//         //x will be out of scope and errased
//     println!("r: {}", r); //r is refference to x above, but since x will be out of scope after the code block above. 
//     //r will become a dangling reference
// }

fn example_1(){
    //allocate space in memory
    let highest_age;

    //initialise variables
    let alice_age = 20;
    let bob_age = 30;

    //call function
    highest_age = largest(&alice_age, &bob_age); 
    //passing alice and bob age into function as reference, because we don't want to mutate their age

    //print output
    println!("Highest age is {}", highest_age);

    //using pointers reference instead of passing variables into and out from an function as parameters, is more preferable in Rust
    fn largest(compare_1: &i32, compare_2: &i32) -> i32{
        if compare_1 > compare_2{
            *compare_1 //returning the de-referenced value
        }else{
            *compare_2 //returning the de-referenced value
        }
    }   
}


fn example_2(){
    //allocate space in memory
    let highest_age;
    let new_value;

    //initialise variables
    let alice_age = 20;

    //a way to solve lifetime issues, if we can't take the code out of a code block is by
    // creating a temporary variable and use that to pass it out
    {
        let bob_age = 30;

        //call function
        highest_age = largest(&alice_age, &bob_age); 
        //passing alice and bob age into function as reference, because we don't want to mutate their age

        new_value = *highest_age;
    } //bob_age will be out of scope after here, and cannot be used

    //print output
    println!("Highest age is {}", new_value);

    //using pointers reference instead of passing variables into and out from an function as parameters, is more preferable in Rust
    // returning an reference to i32
    //<'a> lifetime generics 
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32{
        if compare_1 > compare_2{
            compare_1
        }else{
            compare_2
        }
    }   
}

fn example_3_generics(){
    //allocate space in memory
    let highest_age;
    let new_value;

    //initialise variables
    let alice_age = 20;

    //a way to solve lifetime issues, if we can't take the code out of a code block is by
    // creating a temporary variable and use that to pass it out
    {
        let bob_age = 30;

        //call function
        highest_age = largest::<i32>(&alice_age, &bob_age); 
        //passing alice and bob age into function as reference, because we don't want to mutate their age

        new_value = *highest_age;
    } //bob_age will be out of scope after here, and cannot be used

    //print output
    println!("Highest age is {}", new_value);

    //using pointers reference instead of passing variables into and out from an function as parameters, is more preferable in Rust
    // returning an reference to i32
    //<'a> lifetime generics 
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T{
        if compare_1 > compare_2{
            compare_1
        }else{
            compare_2
        }
    }   
}

#[allow(dead_code, unused_variables)]
struct Person<'a>{
    name: &'a str,
    points: &'a f32
}

fn example_4_with_struct(){
    //allocate space in memory
    let highest_age;
    let new_value;

    //initialise variables
    let alice = Person{name: "alice", points: &20.0};

    //a way to solve lifetime issues, if we can't take the code out of a code block is by
    // creating a temporary variable and use that to pass it out
    {
        let bob = Person{name: "Bob", points: &40.5};

        //call function
        highest_age = largest::<f32>(alice.points, bob.points); 
        //passing alice and bob age into function as reference, because we don't want to mutate their age

        new_value = *highest_age;
    } //bob_age will be out of scope after here, and cannot be used

    //print output
    println!("Highest age is {}", new_value);

    //using pointers reference instead of passing variables into and out from an function as parameters, is more preferable in Rust
    // returning an reference to i32
    //<'a> lifetime generics 
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T{
        if compare_1 > compare_2{
            compare_1
        }else{
            compare_2
        }
    }   
}

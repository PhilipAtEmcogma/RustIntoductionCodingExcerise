


//assigning debug to the user struct, so we can use debug in its instances
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

//impl = implentation function
impl User {
    //read and mutate the values for signin_count in the struct
    fn increment_signin_count(&mut self){
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str){
        self.email = String::from(new_email);
    }
}

//change variable values in a struct via a function.
fn change_username(user: &mut User, new_username: &str){
    user.username = String::from(new_username);
}

//unit test
#[cfg(test)]
mod test{
    //use super to import everything
    use super::*;

    #[test]
    fn tests_structs(){
        //few ways to change the values of a variable in a struct instance:
        //  1. directly changng the variable value, after changing the struct to a mut, e.g. let mut_1: User = User{...}
        //  2. do it as a function, example above, change_username, after changing the struct to a mut, e.g. let mut_1: User = User{...}
        //creating a user_1 using type User
        let mut user_1 = User{
            username: String::from("username1"),
            email: String::from("some@example.com"),
            active: true,
            sign_in_count: 1
        };

        change_username(&mut user_1, "SomebodyThatIUsed2Know");
        //dbg!(user_1);

        let mut user_2 = User{
            username: String::from("username2"),
            email: String::from("someone2@example.com"),
            active: false,
            sign_in_count: 0
        };
        //dbg!(user_2);

        user_2.increment_signin_count(); //increment sgnin_count directly by calling the impl function
        user_2.change_email("anotheremail.email.com");
        dbg!(user_2);

    }
}
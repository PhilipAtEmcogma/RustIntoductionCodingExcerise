#[cfg(test)]
mod test{

    use my_proc_macro::function_to_string;

    const OUTPUT: &str = "HELLO WORLD";

    #[function_to_string]
    fn some_function_for_ai_llm(_whatever_param: &str){
        //test functions
        // we shall give it to an AI to guess an output in a structed manner
        println!("{}", OUTPUT)
    }
    
    #[test]
    fn tests_proc_macro(){
        let x = some_function_for_ai_llm("boo");
        dbg!(x);
    }
}
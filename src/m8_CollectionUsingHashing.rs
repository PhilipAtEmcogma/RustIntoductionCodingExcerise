use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn tests_hashmap(){
        let person_1: &str = "alice";
        let person_2: &str = "bob";

        //hashmap is like key-value pair, for example:
        // &str -> Person
        // u8 -> &str

        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 50);

        let test_res = results_hm.get(person_1);
        dbg!(test_res.unwrap());

        if results_hm.contains_key("alice"){
            dbg!("Alice is here");
        }

    }

    #[test]
    fn tests_hashset(){
        let mut name_hs: HashSet<&str> = HashSet::new();
        name_hs.insert("alice");
        name_hs.insert("bob");
        name_hs.insert("charlie");

        if name_hs.contains("bob"){
            dbg!("Bob is here!");
        }

    }
}
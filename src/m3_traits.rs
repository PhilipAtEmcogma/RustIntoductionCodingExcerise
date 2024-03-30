
trait Attacker {
    fn choose_style(&self) -> String;
    //fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character{
    Warrior,
    Archer,
    Wizard
}

//implementing attacher to the character, meaning adding the attacker to the character enum
impl Attacker for Character{
    fn choose_style(&self) -> String {
        //match is similar to a switch, use to loop through all possible combinations
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "tai chi".to_string(),
        }
    }
}

//unit test
#[cfg(test)]
mod test{
    //use super to import everything
    use super::*;

    #[test]
    fn tests_traits(){
        let my_character = Character::Archer;
        let chosen_fighting_style = my_character.choose_style();
        dbg!(chosen_fighting_style);
    
    }
}
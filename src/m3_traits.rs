trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => String::from("sword"),
            Character::Archer => String::from("bow"),
            Character::Wizard => String::from("wand"),
        }
    }
    fn choose_weapon(&self) -> String {
        String::from("bow")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Archer;
        let chosen_fighting_style = my_character.choose_style();
        dbg!(chosen_fighting_style);
    }
}

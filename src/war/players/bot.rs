use crate::deck::Deck;
use super::super::players::WarPlayer;

pub struct WarBot {
    name: String, 
    pub hand: Deck, 
}

impl WarBot {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(), 
            hand: Deck::empty(),
        }
    }
}

impl WarPlayer for WarBot {
    fn name(&self) -> &str { &self.name } 
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
}

use crate::{BuildInto, Input};

impl BuildInto for WarBot {
    type Target = dyn WarPlayer;

    fn build() -> Box<Self::Target> {
        println!("What do you want to name the bot?");
        
        let name = Input::ask_for_name();
        
        Box::new(Self::new(&name))
    }
}

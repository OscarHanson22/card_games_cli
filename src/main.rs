use go_fish::{Menu, CardGameBuilder, CardSetChanger, Quit, BuildInto};
use go_fish::war::game::War;
use go_fish::go_fish::game::GoFish; 

fn main() {
    let card_game_builder = CardGameBuilder::new(Box::new([
        ("Go Fish", GoFish::build), 
        ("War", War::build),
    ]));

    let main_menu = Menu::new(Box::new([
        ("Play card game", Box::new(card_game_builder)),
        ("Change card set", Box::new(CardSetChanger::new())), 
        ("Quit", Box::new(Quit)),
    ]));

    main_menu.begin();
}

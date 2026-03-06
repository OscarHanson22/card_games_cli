use crate::deck::Deck;
use super::players::GoFishPlayer;

#[derive(Debug)]
pub struct PlayerInfo {
    pub name: String,
    pub amount_of_cards: usize,
    pub amount_of_books: usize, 
}

impl std::convert::From<&Box<dyn GoFishPlayer>> for PlayerInfo {
    fn from(player: &Box<dyn GoFishPlayer>) -> Self {
        Self {
            name: player.name().to_string(),
            amount_of_cards: player.hand().len(),
            amount_of_books: player.books().len(), 
        }
    }
}

#[derive(Debug)]
pub struct PublicInformation {
    pub deck_length: usize,
    pub player_info: Vec<PlayerInfo>,
}

impl std::convert::From<&GoFish> for PublicInformation {
    fn from(game: &GoFish) -> Self {
        Self {
            deck_length: game.deck.len(), 
            player_info: game.players.iter().map(|player| PlayerInfo::from(player)).collect(),
        }
    }
}

pub struct GoFish {
    players: Vec<Box<dyn GoFishPlayer>>,
    deck: Deck,
}

impl GoFish {
    pub fn new(players: Vec<Box<dyn GoFishPlayer>>) -> Self {
        Self {
            players, 
            deck: Deck::shuffled_52_card(),
        }
    }

    pub fn deal(&mut self, n_cards: usize) {
        for _ in 0..n_cards {
            for player in &mut self.players {
                if let Some(card) = self.deck.draw() {
                    player.hand_mut().stack(card);
                }
            }
        }

        for player in &mut self.players {
            player.hand_mut().sort();
        }
    }

    fn amount_of_cards_in_deck(&self) -> usize {
        self.deck.len()
    }

    fn player(&self, player_index: usize) -> &Box<dyn GoFishPlayer> {
        &self.players[player_index]
    }

    fn player_mut(&mut self, player_index: usize) -> &mut Box<dyn GoFishPlayer> {
        &mut self.players[player_index]
    }

    pub fn play(&mut self) {
        let amount_of_players = self.players.len();

        // play the game until the deck and all players' hands are empty
        while !self.players.iter().all(|player| player.hand().is_empty()) {
            let mut current_player = 0;
            let mut current_player_repeat = false;

            while current_player < amount_of_players {
                // give public information to current player
                let public_information = PublicInformation::from(&*self); // de-re-reference to reduce mutability
                self.player_mut(current_player).handle_public_information(public_information);

                // if the game should print information for the current player (for human players). 
                let inform = self.player(current_player).inform();

                // get names of the other players
                let other_players: Vec<(usize, &str)> = self.players.iter()
                    .map(|player| player.name())
                    .enumerate()
                    .filter(|(idx, _)| *idx != current_player)
                    .collect();

                // current player selects someone to ask for a type of card
                if inform { println!() }
                let asked_player = self.player(current_player).ask(other_players);
                if inform { println!() }
                
                // current player picks a type card to ask for
                let picked_rank = match self.player(current_player).pick() {
                    Some(rank) => rank, 
                    None => {
                        // if they have no cards to pick from, they can draw a card
                        if let Some(card) = self.deck.draw() {
                            self.player_mut(current_player).hand_mut().stack(card);
                            card.rank
                        // if there are no cards in the deck, they have no actions to take and the next player's turn begins. 
                        } else {
                            println!();
                            println!("There are no cards left in the deck."); 
                            println!();

                            if current_player_repeat {
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                };

                // prints who asked who for what card rank
                println!();
                println!("{} asks {} if they have any {}'s.", 
                    self.player(current_player).name(), 
                    self.player(asked_player).name(), 
                    picked_rank
                );

                // the cards of the asked for type are removed from the asked players hand
                let gathered_cards = self.player_mut(asked_player)
                    .hand_mut()
                    .remove(|card| card.rank == picked_rank);

                // if there are no cards that match, the asking player must draw a card
                println!();
                if gathered_cards.len() == 0 {
                    println!("{} says GO FISH.", self.player(asked_player).name());
                    println!();

                    if let Some(drawn_card) = self.deck.draw() {
                        self.player_mut(current_player).hand_mut().stack(drawn_card);
                        self.player(current_player).print_drawn_card(&drawn_card);
                        if inform { println!() }
                    } else {
                        println!("No cards left in deck.");
                        println!();
                    }

                    current_player_repeat = false;

                // otherwise, the asking player gains the cards of the asked player and can ask again
                } else {
                    println!("{} had:", self.player(asked_player).name());
                    print!("{}", gathered_cards.as_printable().peek(3));
                    println!();
                                        
                    self.player_mut(current_player)
                        .hand_mut()
                        .stack_deck(gathered_cards);

                    current_player_repeat = true;

                    if self.player(current_player).inform() {
                        println!("You can ask again.");
                    } else {
                        println!("{} can ask again.", self.player(current_player).name());
                    }
                    println!();
                }

                // sorts the hand for easy viewing
                self.player_mut(current_player).hand_mut().sort();
                // creates books (stacks of 4 cards of the same rank) from the current player's hand, if any exist, and add them to the player's books
                self.player_mut(current_player).book();

                // prints information for human players and books for all to see 
                self.player(current_player).print_hand();
                self.player(current_player).print_books();

                // if the current player was not told to go fish, they can go again
                if !current_player_repeat {
                    // waits (for human players) to continue
                    println!();
                    self.player(current_player).wait();
                    println!("- - - - - - - - - -");
                    current_player += 1;
                }
            }
        }

        let leaderboard = Leaderboard::from_players(&self.players);
        // println!("- - - - - - - - - -");
        println!();
        println!("LEADERBOARD: ");
        println!();
        println!("{}", leaderboard); 
    }
}

struct Leaderboard<'a> {
    players: Vec<&'a dyn GoFishPlayer>,
}

impl<'a> Leaderboard<'a> {
    fn sort(&mut self) {
        self.players.sort_by(|p1, p2| p2.books().len().cmp(&p1.books().len()));
    }

    fn from_players(players: &'a [Box<dyn GoFishPlayer>]) -> Self {
        let players: Vec<&dyn GoFishPlayer> = players.iter().map(|boxed_player| &**boxed_player).collect();

        let mut leaderboard = Self {
            players, 
        };

        leaderboard.sort();
        leaderboard
    }
}

use std::fmt;

impl fmt::Display for Leaderboard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ordinal::Ordinal;

        let leaderboard_width = 19 + self.players.iter()
            .map(|player| player.name().len())
            .max()
            .expect("Leaderboard should have at least one player.");

        let mut leaderboard_string = String::new();
        let mut previous_score = None;
        let mut place = 0;

        for player in &self.players {
            let score = player.books().len();

            if previous_score != Some(score) {
                place += 1;
            } 

            let mut leaderboard_entry = String::new();
            leaderboard_entry.push_str(&Ordinal(place).to_string().to_uppercase());
            leaderboard_entry.push_str(" PLACE: ");
            leaderboard_entry.push('\"');
            leaderboard_entry.push_str(player.name());
            leaderboard_entry.push_str("\" ");
            let amount_of_padding = leaderboard_width - leaderboard_entry.len();
            leaderboard_entry.push_str(&".".repeat(amount_of_padding));
            leaderboard_entry.push(' ');
            leaderboard_entry.push_str(&score.to_string());
            leaderboard_entry.push_str(" BOOKS\n");

            leaderboard_string.push_str(&leaderboard_entry);

            previous_score = Some(score);
        }

        write!(f, "{}", leaderboard_string)
    }
}

use crate::{BuildInto, CardGame, CardGamePlayerBuilder, Input};
use super::players::{human_player::HumanPlayer, random_bot::RandomBot};

impl CardGame for GoFish {
    fn play(&mut self) {
        self.play();
    }
}

impl BuildInto for GoFish {
    type Target = dyn CardGame;

    fn build() -> Box<Self::Target> {
        println!("Now building a game of Go Fish.");
        println!();

        let player_builder = CardGamePlayerBuilder::new(Box::new([
            ("Human Player", HumanPlayer::build), 
            ("Random Bot", RandomBot::build),
        ]));

        let players = player_builder.build_players();
        let amount_of_players = players.len();

        let mut go_fish = GoFish::new(players);

        let default_amount_of_cards_to_deal = if amount_of_players < 4 { 7 } else { 5 };
        let maximum_amount_of_cards_to_deal = go_fish.amount_of_cards_in_deck() / amount_of_players;

        println!("How many cards do you want to deal to each player?");
        println!("Press ENTER for default ({}) or enter a value from 1 to {}.", default_amount_of_cards_to_deal, maximum_amount_of_cards_to_deal);

        let amount_cards_to_deal = Input::ask_until(
            |input| input > 0 && input <= maximum_amount_of_cards_to_deal, 
            Some(default_amount_of_cards_to_deal)
        );

        println!();

        go_fish.deal(amount_cards_to_deal);

        println!("Game of Go Fish created successfully!");
        println!();
        println!("Now playing...");
        println!();

        Box::new(go_fish)
    }
}
use crate::deck::{Deck, Card};
use super::players::WarPlayer;

pub struct War {
    players: Vec<Box<dyn WarPlayer>>, 
    amount_of_cards_to_draw_during_war: usize,
}

impl War {
    pub fn new(players: Vec<Box<dyn WarPlayer>>, amount_of_cards_to_draw_during_war: usize) -> Self {
        Self {
            players,
            amount_of_cards_to_draw_during_war, 
        }
    }

    pub fn deal(&mut self, amount_of_decks: usize) {
        for _ in 0..amount_of_decks {
            let mut deck = Deck::shuffled_52_card();

            while !deck.is_empty() {
                for player in &mut self.players {
                    if let Some(card) = deck.draw() {
                        player.hand_mut().stack(card);
                    }
                }
            }
        }
    }   

    fn player(&self, player_index: usize) -> &Box<dyn WarPlayer> {
        &self.players[player_index]
    }

    fn player_mut(&mut self, player_index: usize) -> &mut Box<dyn WarPlayer> {
        &mut self.players[player_index]
    }

    /// Returns player indices corresponding to the highest card rank i.e. the winners. 
    /// 
    /// Ties for highest card rank are added to the candidates. 
    ///
    fn determine_candidates(from_competition: &[(usize, Card)]) -> Vec<usize> {
        if from_competition.len() == 0 {
            return Vec::new();
        }

        let highest_rank = from_competition.iter()
            .max_by(|(_, card_1), (_, card_2)| (card_1.rank as usize).cmp(&(card_2.rank as usize)))
            .expect("Should have at least one card in competition.")
            .1
            .rank;

        let candidates: Vec<usize> = from_competition.iter()
            .filter(|(_, card)| card.rank == highest_rank)
            .map(|(index, _)| *index)
            .collect();

        candidates
    }

    fn pause() {
        println!("Press ENTER to continue...");
        std::io::stdin().read_line(&mut String::new()).expect("Something went wrong.");
    }

    fn pluralize(string: &str, count: usize) -> String {
        let string = string.to_string();
        if count == 1 {
            string
        } else {
            string + "s"
        }
    }

    fn list(items: &[&str]) -> String {
        let mut list_string = String::new();

        if items.len() == 1 {
            list_string.push_str(items[0]);
        } else if items.len() == 2 {
            list_string.push_str(items[0]);
            list_string.push_str(" and ");
            list_string.push_str(items[1]);
        } else {
            for item_index in 0..items.len() - 1 {
                list_string.push_str(items[item_index]);
                list_string.push_str(", ");
            }
            list_string.push_str("and ");
            list_string.push_str(items[items.len() - 1]);
        }

        list_string
    }

    fn war(&mut self, candidates: &[usize], continuing_war: bool) -> (usize, Deck) {
        let amount_of_candidates = candidates.len();

        let candidate_names = |candidates: &[usize]| candidates.iter().map(|candidate| self.player(*candidate).name()).collect::<Vec<&str>>();
        
        let war_status_string = if continuing_war { "are continuing to wage war!" } else { "are waging war!" };
        println!("{} {}", Self::list(&candidate_names(candidates)), war_status_string);
        println!();

        // Draw `amount_of_cards_to_draw_during_war` cards from each candidate
        let mut candidate_draws: Vec<Deck> = Vec::new();

        for candidate in candidates {
            let candidate_name = self.player(*candidate).name();
            let candidate_amount_of_cards = self.player(*candidate).hand().len();
            
            if candidate_amount_of_cards == 0 {
                println!("{} does not have enough cards to continue waging war.", candidate_name);
                println!();   

                candidate_draws.push(Deck::empty());
            } else {
                println!("{} is now waging war.", candidate_name);  
                println!("{} has {} {}.", candidate_name, candidate_amount_of_cards, Self::pluralize("card", candidate_amount_of_cards));  
                println!();             
                
                let amount_of_cards_to_draw = self.amount_of_cards_to_draw_during_war;
                let candidate_wagings = self.player_mut(*candidate).war_draw(amount_of_cards_to_draw);
                candidate_draws.push(candidate_wagings);
            }
            
            Self::pause();
        }

        let mut competition: Vec<(usize, Card)> = Vec::new();

        for candidate_index in 0..amount_of_candidates {
            let candidate = candidates[candidate_index];
            let candidate_draw = &mut candidate_draws[candidate_index];

            if candidate_draw.len() == 0 {
                continue;
            }

            let last_card_index = candidate_draw.len() - 1;
            let flipped_card = candidate_draw.card_at_index(last_card_index);

            competition.push((candidate, flipped_card));
        }

        let mut cards_to_gain: Deck = Deck::from(candidate_draws);

        let war_candidates = Self::determine_candidates(&competition);

        // higher degree war, pretty rare
        if war_candidates.len() > 1 {
            let (after_war_candidate, after_war_cards_to_gain) = self.war(&war_candidates, true);
            cards_to_gain.stack_deck(after_war_cards_to_gain);
            
            // Only print after first war
            if !continuing_war { 
                println!("{} returns from a long war victorious!", self.player(after_war_candidate).name()); 
                println!();
            }

            (after_war_candidate, cards_to_gain)
        } else {
            (war_candidates[0], cards_to_gain)
        }
    } 

    pub fn play(&mut self) {
        println!("Let's play war!");
        println!();

        let mut eliminations: Vec<Vec<usize>> = Vec::new();
        let mut eliminated: Vec<bool> = (0..self.players.len()).map(|_| false).collect();

        let active_players = |eliminated_: &[bool]| -> Vec<usize> { eliminated_.iter().enumerate().filter(|(_, e)| !**e).map(|(i, _)| i).collect() };

        // While two or more players are still playing
        while active_players(&eliminated).len() > 1 {
            let mut competition: Vec<(usize, Card)> = Vec::new();

            for (player_index, player) in &mut self.players.iter_mut().enumerate() {
                // Skip eliminated players' turns
                if eliminated[player_index] {
                    continue;
                }

                // Print name and amount of cards
                println!("{}'s turn!", player.name());

                let amount_of_cards_in_hand = player.hand().len();
                if player.inform() {
                    println!("You have {} {}.", amount_of_cards_in_hand, Self::pluralize("card", amount_of_cards_in_hand));
                } else {
                    println!("{} has {} {}.", player.name(), amount_of_cards_in_hand, Self::pluralize("card", amount_of_cards_in_hand));
                }

                // Let player draw a card
                let drawn_card = player.draw().expect("Length checked above, should be at least one.");

                if !player.inform() {
                    println!("{} drew a:", player.name());
                    println!("{}", drawn_card.as_printable());
                    println!();
                }

                // Add player to the current competition
                competition.push((player_index, drawn_card));

                Self::pause();
            }

            // Finding winner, or candidates to win
            let candidates = Self::determine_candidates(&competition);

            let mut winner = candidates[0];
            let mut cards_from_war = Deck::empty();

            // War if two or more people have the highest rank together
            if candidates.len() > 1 {
                (winner, cards_from_war) = self.war(&candidates, false);
            }

            let winner_name = self.player(winner).name();

            println!("{} wins!", winner_name);

            // The winner gains all cards from the current competition and any wars that occurred
            let mut gained_cards = Deck::from(competition.into_iter().map(|(_, c)| c).collect::<Vec<Card>>());
            gained_cards.stack_deck(cards_from_war);

            println!("{} gains:", winner_name);
            println!("{}", gained_cards.as_printable().peek(3));

            gained_cards.shuffle(); // mimics how the game happens in real life
            self.player_mut(winner).hand_mut().stack_deck_under(gained_cards);
            
            Self::pause();

            let mut all_eliminations_in_round = Vec::new();

            // Eliminates all players with no cards
            for (player_index, player) in self.players.iter().enumerate() {
                if eliminated[player_index] {
                    continue;
                }

                if player.hand().len() == 0 {
                    println!("{} is out of cards!", player.name());
                    println!();
                    Self::pause();

                    eliminated[player_index] = true; 
                    all_eliminations_in_round.push(player_index);
                }
            }

            if all_eliminations_in_round.len() > 0 {
                eliminations.push(all_eliminations_in_round);
            }
        }

        // Designate winning player and print out a leaderboard
        let winner = *active_players(&eliminated).iter().next().expect("Should have only one winner.");

        println!("The winner is {}!", self.player(winner).name());
        println!();

        let longest_name_length = self.players.iter().map(|player| player.name().len()).max().expect("Should have at least one player");
        let leaderboard_width = 20 + longest_name_length; 

        println!("{}LEADERBOARD:", " ".repeat(leaderboard_width / 2 - 4));
        println!("{}", "-".repeat(leaderboard_width + 4));
        println!();

        let place_string = ordinal::Ordinal(1).to_string().to_uppercase();
        let winner_name = self.player(winner).name();
        let length_of_dots = leaderboard_width - place_string.len() - winner_name.len();

        println!(" {} {} {}", place_string, ".".repeat(length_of_dots), winner_name);

        for (index, eliminated_together) in eliminations.into_iter().rev().enumerate() {
            let place = index + 2;

            for eliminated in eliminated_together {
                let place_string = ordinal::Ordinal(place).to_string().to_uppercase();
                let player_name = self.player(eliminated).name();
                let length_of_dots = leaderboard_width - place_string.len() - player_name.len();

                println!(" {} {} {}", place_string, ".".repeat(length_of_dots), player_name);
            }
        }
        println!();
        println!("{}", "-".repeat(leaderboard_width + 4));
        println!();
    }
}

use crate::{BuildInto, CardGame, CardGamePlayerBuilder, Input};
use super::players::{human::HumanWarPlayer, bot::WarBot};

impl CardGame for War {
    fn play(&mut self) {
        self.play();
    }
}

impl BuildInto for War {
    type Target = dyn CardGame;

    fn build() -> Box<Self::Target> {
        println!("Now building a game of War.");
        println!();

        let player_builder = CardGamePlayerBuilder::new(Box::new([
            ("Human Player", HumanWarPlayer::build), 
            ("Bot", WarBot::build),
        ]));

        let players = player_builder.build_players();

        println!("How many cards do you want to draw during a war?");
        println!("Press ENTER for default (3) or enter a value from 1 upwards.");

        let amount_of_cards_to_draw_during_war = Input::ask_until(|input| input > 0, Some(3));
        
        println!();

        let mut war = War::new(players, amount_of_cards_to_draw_during_war);

        println!("How many decks of cards do you want to use?");
        println!("Press ENTER for default (1) or enter a value from 1 upwards.");

        let amount_of_decks_to_deal = Input::ask_until(|input| input > 0, Some(1));

        println!();

        war.deal(amount_of_decks_to_deal);

        println!("Game of War created successfully!");
        println!();
        println!("Now playing...");
        println!();

        Box::new(war)
    }
}
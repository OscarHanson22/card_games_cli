pub mod human_player;
pub mod random_bot;
// pub mod good_bot; // un-comment when done with bot prototype

use crate::deck::{Deck, Card, Rank, print_side_by_side};
use super::game::PublicInformation;

/// A trait that defines a player of Go Fish. 
pub trait GoFishPlayer {
    /// Should return the name of the player. 
    fn name(&self) -> &str;

    /// Should return a reference to the player's hand. 
    fn hand(&self) -> &Deck;
    /// Should return a mutable reference to the player's hand. 
    fn hand_mut(&mut self) -> &mut Deck;

    /// Should return a reference to the books of the player. 
    fn books(&self) -> &Vec<Deck>;
    /// Should return a mutable reference to the books of the player. 
    fn books_mut(&mut self) -> &mut Vec<Deck>;

    /// Should return a card in the player's hand (if they have any cards). 
    fn pick(&self) -> Option<Rank>;
    /// Should return the index (other_players[_].0) of the player they want to ask for a specific type of card. 
    fn ask(&self, other_players: Vec<(usize, &str)>) -> usize;

    #[allow(unused_variables)]
    /// Gives the player access to and lets them handle public game information (such as the amount of cards in the deck or how many cards a player has).
    fn handle_public_information(&mut self, public_information: PublicInformation) { }

    /// Removes all books from the player's hand and adds them to their books. 
    fn book(&mut self) {
        for rank in Rank::ranks() {
            if self.hand().cards(|card| card.rank == rank).len() == 4 {
                let book = self.hand_mut().remove(|card| card.rank == rank);
                self.books_mut().push(book);
            }
        }
    }

    /// Prints the books of the player.
    fn print_books(&self) {
        if self.books().len() == 0 {
            if self.inform() {
                println!("You currently have no books.");
            } else {
                println!("{} currently has no books.", self.name());
            }

            return;
        }

        if self.inform() {
            println!("You currently have books: ");
        } else {
            println!("{} currently has books: ", self.name());
        }

        let amount_of_books = self.books().len();

        let mut books_with_formatting = self.books()
            .iter()
            .map(|book| book.as_printable());

        let books_per_row = 5;
        let mut amount_of_rows = amount_of_books / books_per_row + 1;

        if amount_of_books % books_per_row == 0 {
            amount_of_rows -= 1;
        }

        for _ in 0..amount_of_rows {
            let book_row: Vec<_> = books_with_formatting.by_ref().take(books_per_row).collect();
            print_side_by_side(&book_row, " ");
        }

        // print_side_by_side(&books_with_formatting, " ");
    }

    /// Whether or not to print game information for human players. 
    /// If set to false (default), the following functions will not print anything. 
    /// If set to true, the following functions will print information for human players. 
    fn inform(&self) -> bool { false }

    /// Prints the player's hand (for human players to see their cards). 
    fn print_hand(&self) {
        if self.inform() {
            println!("You now have: ");
            print!("{}", self.hand().as_printable().peek(3));
        }
    }

    /// Prints the card `drawn_card` (for human players to see their drawn card).
    fn print_drawn_card(&self, drawn_card: &Card) {
        if self.inform() {
            println!("You drew a: \n{}", drawn_card.as_printable());
        }
    }

    /// Allows for an arbitrary pause in game execution (for human players). 
    fn wait(&self) { 
        if self.inform() {
            println!("Press ENTER to continue...");
            std::io::stdin().read_line(&mut String::new()).expect("Something went wrong.");
            // println!("{}", "\n".repeat(100));
        }
    }
}
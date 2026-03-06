use std::fmt;

pub use crate::card::{Card, Rank, Suit};

/// A representation of a deck of playing cards. 
pub struct Deck(Vec<Card>);

impl Deck {
    /// Creates a deck with no cards.
    pub fn empty() -> Self {
        Deck(Vec::new())
    }

    /// Creates a standard, unshuffled deck of 52 cards. 
    fn unshuffled_52_card() -> Self {
        let mut deck = Vec::new();

        for suit in Suit::suits() {
            for rank in Rank::ranks() {
                deck.push(Card::new(rank, suit));
            }
        }

        Deck(deck)
    }

    /// Shuffles the cards in the deck randomly. 
    /// 
    /// Note - does not imitate the behavior of real-world shuffles. 
    pub fn shuffle(&mut self) {
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        self.0.shuffle(&mut thread_rng());
    }

    /// Sorts the cards of the deck. 
    /// 
    /// Useful for sorting a player's hand to make it easier to read. 
    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.rank.cmp(&b.rank));
    }

    /// Creates a standard shuffled 52 card deck. 
    pub fn shuffled_52_card() -> Self {
        let mut deck = Self::unshuffled_52_card();
        deck.shuffle();
        deck
    }

    /// Adds a card `card` to the top of the deck. 
    pub fn stack(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Stacks all cards from `other_deck` onto the deck. 
    pub fn stack_deck(&mut self, other_deck: Self) {
        for card in other_deck.0 {
            self.0.push(card);
        }
    }

    /// Adds a card `card` to the bottom of the deck. 
    pub fn stack_under(&mut self, card: Card) {
        let _ = self.0.splice(..0, [card]);
    }

    /// Stacks all cards from `other_deck` to the bottom of the deck.
    ///
    /// The equivalent of taking a deck of cards (`other_deck`) and cutting it underneath the deck. 
    /// This preserves the order of both decks and does not reverse `other_deck`.
    pub fn stack_deck_under(&mut self, other_deck: Self) {
        let _ = self.0.splice(..0, other_deck.0);
    }

    /// Draws and returns the top card from the deck. 
    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }

    /// Returns the length of the deck. 
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the deck is empty. 
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the card at index `index`. 
    ///
    /// Panics if index is out-of-bounds. 
    pub fn card_at_index(&self, index: usize) -> Card {
        self.0[index]
    }

    /// Removes and returns all cards that match `predicate`.
    pub fn remove<P>(&mut self, predicate: P) -> Self
    where
        P: Fn(&Card) -> bool,
    {
        let removed_cards: Vec<Card> = self.0.iter().filter(|card| predicate(*card)).copied().collect();
        self.0 = self.0.iter().filter(|card| !predicate(*card)).copied().collect();
        Self(removed_cards)
    }

    /// Returns copies of all of the cards that match 'predicate'. 
    pub fn cards<P>(&self, predicate: P) -> Self
    where
        P: Fn(&Card) -> bool,
    {
        Self(self.0.iter().filter(|card| predicate(*card)).copied().collect())
    }

    /// Returns a `PrintDeck` from the deck. 
    pub fn as_printable(&self) -> PrintDeck {
        PrintDeck::from(self)
    }
}

use std::convert::From;

impl From<Vec<Card>> for Deck {
    fn from(vec: Vec<Card>) -> Self {
        Deck(vec)
    }
}

impl From<Vec<Deck>> for Deck {
    fn from(vec: Vec<Deck>) -> Self {
        Self::from(vec.into_iter().map(|deck| deck.0).flatten().collect::<Vec<Card>>())
    }
}

impl From<&[Card]> for Deck {
    fn from(slice: &[Card]) -> Self {
        Deck(slice.to_owned())
    }
}

// impl From<Card> for Deck {
//     fn from(card: Card) -> Self {
//         Deck(Vec::)
//     }
// }

pub struct DeckIter<'a> {
    index: usize, 
    deck: &'a Deck,
}

impl<'a> DeckIter<'a> {
    fn new(deck: &'a Deck) -> Self {
        Self {
            index: 0, 
            deck, 
        }
    }
}

impl<'a> Iterator for DeckIter<'a> {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.deck.len() {
            return None;
        }

        let next = self.deck.card_at_index(self.index);
        self.index += 1;

        Some(next)
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = Card;
    type IntoIter = DeckIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self)
    }
}

// impl Iterator for &Deck {
//     type Item = Card;

//     fn next(&mut self) 
// }

/// Returns the lines of `item` as a Vec of Strings. 
fn lineify<T>(item: &T) -> Vec<String> 
where 
    T: fmt::Display
{
    item.to_string()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

/// Only keeps the first `n`+ 1 elements of each line of `item`. 
fn keep_n_columns<T>(item: &T, n: usize) -> String
where 
    T: fmt::Display
{
    item.to_string()
        .lines()
        .map(|line| {
            let line = line.chars()
                .enumerate()
                .take_while(|(i, _)| *i <= n )
                .map(|(_, c)| c)
                .collect::<String>();

            line + "\n"
        })
        .collect()
}

/// Returns a String with each item in `items` being next to each other separated by `separator`. 
/// If an item has more lines than another, the extra lines will not be in the result. 
/// If an item is represented as a zero-length-string (""), then it will not be printed and will not truncate the amount of lines printed. 
///
/// `items` - a collection of 2D strings (or object that can become a string) (such as a card) that will be strung side by side.
/// `separator` - the separator between each item in `items`. Will only occur between items. 
///
fn string_side_by_side<T>(items: &[T], separator: &str) -> String 
where
    T: fmt::Display,
{
    if items.iter().filter(|item| item.to_string().len() > 0).count() == 0 {
        return String::new();
    }

    let mut side_by_side_string = String::new();

    let lines_of_items: Vec<Vec<String>> = items.iter()
        .filter(|item| item.to_string().len() > 0)
        .map(|item| lineify(item))
        .collect();

    let min_line_depth = lines_of_items.iter()
        .map(|lines| lines.iter().count())
        .min()
        .expect("Should have at least one element.");

    for line_index in 0..min_line_depth {
        for (index, item_lines) in lines_of_items.iter().enumerate() {
            let line = &item_lines[line_index];

            side_by_side_string.push_str(line);

            if index != lines_of_items.len() - 1 {
                side_by_side_string.push_str(separator);
            }
        }

        side_by_side_string.push('\n');
    }

    side_by_side_string
}

/// Prints a string generated by `string_side_by_size(`items`, `separator`)`.
pub fn print_side_by_side<T>(items: &[T], separator: &str) 
where
    T: fmt::Display,
{
    let side_by_side_string = string_side_by_side(items, separator);

    print!("{}", side_by_side_string);
}

/// Used to print a `Deck` with several options. 
///
/// `PrintDeck` implements `Display`. 
///     Decks can only be printed through PrintDecks. 
/// 
pub struct PrintDeck<'a> {
    /// `from_deck` - the `Deck` to print. 
    from_deck: &'a Deck,

    /// `peek_n_amount_of_card` - the amount characters of each card face to show 
    ///     0 shows only the edge,
    ///     1 shows one character of the face of the card past the edge, 
    ///     2 shows two...
    ///     Anything above the width of the card saturates at the width of the card and will not cause any errors. 
    pub peek_n_amount_of_card: usize,

    /// `show_last_n_cards` - the amount of cards to show from the top of the deck down. 
    ///     None prints all cards, 
    ///     Some(0) shows zero cards, 
    ///     Some(1) shows only the top card, 
    ///     Some(2) shows the top two cards, 
    ///     Some(3) shows the top three cards...
    ///     Anything above the amount of cards in the deck will saturate at the length of the deck and will not cause any errors. 
    pub show_last_n_cards: Option<usize>,

    /// `show_faces` - whether or not to show the faces of the cards. 
    pub show_faces: bool, 
}

impl<'a> From<&'a Deck> for PrintDeck<'a> {
    /// Used to construct a `PrintDeck`.
    fn from(deck: &'a Deck) -> Self {
        Self {
            from_deck: deck,
            peek_n_amount_of_card: 0, 
            show_last_n_cards: None, 
            show_faces: true,
        }
    }
}

impl<'a> PrintDeck<'a> {
    /// Changes `peek_n_amount_of_card` to whatever specified. 
    pub fn peek(mut self, peek_n_amount_of_card: usize) -> Self {
        self.peek_n_amount_of_card = peek_n_amount_of_card;
        self
    }

    /// Changes `show_last_n_cards` to whatever specified. 
    pub fn only_show(mut self, show_last_n_cards: usize) -> Self {
        self.show_last_n_cards = Some(show_last_n_cards);
        self
    }

    /// Changes `show_last_n_cards` to whatever specified. 
    pub fn hide_faces(mut self) -> Self {
        self.show_faces = false;
        self
    }
}

impl fmt::Display for PrintDeck<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length_of_deck = self.from_deck.0.len();

        // unwrap `self.show_last_n_cards` 
        let show_last_n_cards = match self.show_last_n_cards {
            Some(n) => n, 
            None => length_of_deck, 
        };

        // early return if the deck has no cards
        if length_of_deck == 0 || show_last_n_cards == 0 {
            return write!(f, "");
        }

        // skip to the last `show_last_n_cards` cards
        let amount_of_cards_to_skip = length_of_deck.saturating_sub(show_last_n_cards);
        let mut cards_to_print = self.from_deck.0.iter().skip(amount_of_cards_to_skip);

        // take the last card from the deck 
        let last_card = cards_to_print.next_back()
            .expect("Should have at least one card in the deck.");

        let last_card_string = last_card.as_printable().show(self.show_faces).to_string();

        let mut hidden_cards_strings = Vec::new();

        // add columns from the left of the remaining cards with
        // width `self.peek_n_amount_of_card + 1` of the cards 'behind' the last card
        for card in cards_to_print {
            hidden_cards_strings.push(keep_n_columns(&card.as_printable().show(self.show_faces), self.peek_n_amount_of_card));
        }

        // finally add the last card 
        hidden_cards_strings.push(last_card_string);

        let card_string_vec = hidden_cards_strings;
        
        // string the hidden cards and last card side by side
        let deck_string = string_side_by_side(&card_string_vec, "");

        write!(f, "{}", deck_string)
    }
}
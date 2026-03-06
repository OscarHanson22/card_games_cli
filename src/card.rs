use std::fmt;
use std::fs::File;
use std::io::Read;

use std::rc::Rc;
use std::cell::RefCell;

// The current CardSet.
thread_local!(
    static CARD_SET: Rc<RefCell<CardSet>> = Rc::new(RefCell::new(CardSet::from("classic.txt")));
);

// Previously loaded card sets
// thread_local!(
//     static CARD_SETS = Rc<RefCell<HashMap<String, CardSet>>> = Rc::new(RefCell::new(HashMap::new()));
// );

/// Sets the global CardSet.
pub fn set_card_set(filename: &str) {
    CARD_SET.with(|card_set| {
        *card_set.borrow_mut() = CardSet::from(filename);
    });
}

/// Returns the currently active CardSet.
fn get_card_set() -> Rc<RefCell<CardSet>> {
    CARD_SET.with(|card_set| {
        card_set.clone()
    })
}

/// The `textures` used to print cards to the console. 
struct CardSet {
    /// The name of the file of card textures. 
    // filename: String,
    /// The card face textures.
    cards: Vec<String>, 
    /// The texture of the back of the card. 
    back: String,
}

impl CardSet {
    /// Creates a CardSet from a card text file. 
    /// 
    /// A card text file has the following elements:
    ///     Textures are separated by a blank line.  
    ///     The card text file must have the back card texture first.
    ///     The card text file must have the card face textures in order 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, A after the back card texture.
    ///     The `S`s on card denote the suit of the card, and will be replaced by the repective suit character. 
    ///         It is recommended to have at least one `S` on every card (besides the back) near the left edge, or else the suit cannot be known from printing or won't show in a deck printing.
    ///     The file itself must be located inside the `card_sets` folder. 
    ///
    /// Note: the cards can be any rectangular size (at least to print well). 
    ///
    /// See `card_sets/classic.txt` for an example. 
    ///
    fn from(from_filename: &str) -> Self {
        let filename_with_path = String::from("card_sets/") + from_filename;
        let mut card_file = File::open(&filename_with_path).expect("Please provide a card set text file for a card set (must be in `card_sets` folder).");
        let mut cards_string_from_file = String::new();
        card_file.read_to_string(&mut cards_string_from_file).expect("Something went wrong reading the cards from the file.");

        let mut cards_iter = cards_string_from_file.split("\r\n\r\n").map(|s| s.to_string());

        let back = cards_iter.next().expect("No card back provided (Should be the first card in the card set text file).");

        let card_strings: Vec<String> = cards_iter.collect();

        if card_strings.len() != 13 {
            panic!("Wrong amount of cards provided in card set text file (Should be a back and 13 rank cards for a total of 14 but found {}", card_strings.len() + 1);
        }

        let mut cards = Vec::new();

        for suit in Suit::suits() {
            for card_string in &card_strings {
                cards.push(card_string.replace("S", &suit.to_string()));
            }
        }

        Self {
            // filename: from_filename.to_string(),
            cards,
            back,
        }
    }

    // fn filename(&self) -> &str {
    //     &self.filename
    // }

    /// Returns the card texture with rank `rank` and suit `suit`. 
    fn get_card(&self, rank: Rank, suit: Suit) -> &str {
        &self.cards[rank as usize + /* Amount of ranks */ 13 * suit as usize]
    }

    /// Returns the back card texture. 
    fn get_back(&self) -> &str {
        &self.back
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// A representation of the suit of a playing card. 
pub enum Suit {
    Clubs = 0, 
    Spades, 
    Hearts, 
    Diamonds
}

impl Suit {
    /// Returns an iterator over all of the suits. 
    pub fn suits() -> impl Iterator<Item = Suit> {
        [
            Suit::Clubs, 
            Suit::Spades, 
            Suit::Hearts, 
            Suit::Diamonds
        ].iter().copied()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// A representation of the rank of a playing card. 
pub enum Rank {
    Two = 0, 
    Three, 
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight,
    Nine, 
    Ten, 
    Jack, 
    Queen, 
    King,
    Ace,
}

impl Rank {
    /// Returns an iterators over all of the ranks. 
    pub fn ranks() -> impl Iterator<Item = Rank> {
        [
            Rank::Two, 
            Rank::Three, 
            Rank::Four, 
            Rank::Five, 
            Rank::Six, 
            Rank::Seven, 
            Rank::Eight,
            Rank::Nine, 
            Rank::Ten, 
            Rank::Jack, 
            Rank::Queen, 
            Rank::King, 
            Rank::Ace, 
        ].iter().copied()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            Rank::Two => "2", 
            Rank::Three => "3", 
            Rank::Four => "4", 
            Rank::Five => "5", 
            Rank::Six => "6", 
            Rank::Seven => "7", 
            Rank::Eight => "8",
            Rank::Nine => "9", 
            Rank::Ten => "10", 
            Rank::Jack => "J", 
            Rank::Queen => "Q", 
            Rank::King => "K", 
            Rank::Ace => "A", 
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// A represetation of a playing card. 
pub struct Card {
    /// The rank of the playing card. 
    pub rank: Rank,
    /// The suit of the playing card. 
    pub suit: Suit, 
}

impl Card {
    /// Creates a card with rank `rank` and suit `suit`. 
    /// 
    /// Order is rank then suit like how the card's name is spoken:
    /// Ace of Spades -> {rank} of {suit} -> rank, suit. 
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            suit, 
            rank, 
        }
    }

    /// Returns a `PrintCard` from the card. 
    pub fn as_printable(&self) -> PrintCard {
        PrintCard::from(self)
    }

    // /// Returns an abbreviation the card. 
    // /// 
    // /// Card::new(Rank::Ace, Suit::Spades) -> "A of ♠".
    // pub fn abbrev(&self) -> String {
    //     use fmt::Write;

    //     let mut abbrev = String::new();
    //     match write!(abbrev, "{} of {}", self.rank, self.suit) {
    //         Ok(abbrev) => abbrev, 
    //         Err(e) => {
    //             println!("Error getting card abbreviation: {e:?}");
    //             panic!();
    //         }
    //     }

    //     abbrev
    // }
}

pub struct PrintCard<'a> {
    from_card: &'a Card, 
    show: bool, 
}

impl<'a> std::convert::From<&'a Card> for PrintCard<'a> {
    fn from(card: &'a Card) -> Self {
        Self {
            from_card: card, 
            show: true,
        }
    }
}

impl<'a> PrintCard<'a> {
    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }
}

impl fmt::Display for PrintCard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.show {
            write!(f, "{}", get_card_set().borrow().get_card(self.from_card.rank, self.from_card.suit))
        } else {
            write!(f, "{}", get_card_set().borrow().get_back())
        }
    }
}

// #[derive(Copy, Clone)]
// struct Color(u8, u8, u8);

// impl Color {
//     const WHITE: Self = Self(255, 255, 255);
//     const RED: Self = Self(255, 0, 0);

//     fn as_string(&self) -> String {
//         let mut string = String::new();
//         string.push_str(&self.0.to_string());
//         string.push(';');
//         string.push_str(&self.1.to_string());
//         string.push(';');
//         string.push_str(&self.2.to_string());
//         string
//     }

//     fn as_text_color(&self) -> String {
//         let mut string = String::new();
//         string.push_str("\x1b[38;2;");
//         string.push_str(&self.as_string());
//         string.push('m');
//         string
//     }

//     fn as_background_color(&self) -> String {
//         let mut string = String::new();
//         string.push_str("\x1b[48;2;");
//         string.push_str(&self.as_string());
//         string.push('m');
//         string
//     }
// }

// struct ColorString(String);

// impl std::fmt::Display for ColorString {}

// fn colorize(string: &str, text_color: Option<Color>, background_color: Option<Color>) -> String {
//     let mut color_string = String::new();

//     if let Some(color) = text_color {
//         color_string.push_str(&color.as_text_color());
//     }

//     if let Some(color) = background_color {
//         color_string.push_str(&color.as_background_color());
//     }

//     color_string.push_str(&string);
//     color_string.push_str("\x1b[0m");
//     color_string
// }
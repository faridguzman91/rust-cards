//name of struct always capitalized
// cards - list of fields that the struct will wrap up
// Vec - array that grows or shrinks in size
// Vec<String> - vector that contains strings
// you can make different instances of decks with a  struct wiht each a diffferent list of cards

// derive a Debug for the print line, defines attributes for the struct,  gives rust compolier more
// instructions
// derive -> attribute, specifies which traits to automatically implement for this strict
// (Debug) -> Trait (Debug), traits a set of functions

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

//add an implementation for a new Instance of Deck
impl Deck {
    //new function with return type anotation -> Deck , or Self, which reinfers its own type
    fn new() -> Self {
        // let- declares a new binding
        // let deck : Deck -> type annotattion- type inlays
        // Deck { cards: vec![] } -> Struct literal, creates an instance of a struct
        // vec![] -> creates an empty vector, ! indicates a macro
        // you can also write it as { cards: Vec::new() } , it is the same

        // list of suits - hearts, spades, clubs, diamonds
        // list of values - ace, two, three
        // double nested for loop

        //indicate whether this array size changes overtime with a vec, if it doesnt , with a normal
        //array

        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        //mut -> make mutable, lets (bindings) are immutable by default, and cant be reassigned
        // loop in the vec array

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                // format teplate literal values
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // cards are mutable so remove the vec![]
        // you can also just return the Deck struct instead of declaring it again
        //  or just remove the semicolon and remove return, because its an implicit return
        Deck { cards }
    }
    //shuffle fn that is changing the Deck struct, makes the self mutable
    fn shuffle(&mut self) {
        //random number variable obviously changes, make it also mutable
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }
}

fn main() {
    // add the implementation of new function of deck struct in the main fn, make it also mutable
    let mut deck = Deck::new();

    deck.shuffle();
    //print the deck with a {:?} formatter, for a debugged printed line, add # for an escaped
    //printed array
    println!("Heres is the deck: {:#?}", deck);
}

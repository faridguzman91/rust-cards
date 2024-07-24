//name of struct always capitalized
// cards - list of fields that the struct will wrap up
// Vec - array that grows or shrinks in size
// Vec<String> - vector that contains strings
// you can make different instances of decks with a  struct wiht each a diffferent list of cards

// derive a Debug for the print line, defines attributes for the struct,  gives rust compolier more
// instructions
// derive -> attribute, specifies which traits to automatically implement for this strict
// (Debug) -> Trait (Debug), traits a set of functions

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    // let- declares a new binding
    // let deck : Deck -> type annotattion- type inlays
    // Deck { cards: vec![] } -> Struct literal, creates an instance of a struct
    // vec![] -> creates an empty vector, ! indicates a macro
    // you can also write it as { cards: Vec::new() } , it is the same

    let deck = Deck { cards: vec![] };
    //print the deck with a {:?} formatter, for a debugged printed line
    println!("Heres is the deck: {:?}", deck);
}

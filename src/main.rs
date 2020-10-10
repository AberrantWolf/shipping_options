#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::State;
use rocket_contrib::templates::Template;

use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeJsonResult;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

// NEXT STEPS -------------------------------------------------------------------------
// TODO: Add some JavaScript to the page to generate a dropdown of fandoms
// TODO: Make the dropdown selection create another selection list of characters
// TODO: Set up a DIV for adding columns, add a button which adds columns to the DIV
// TODO: (Each column is another instance of the fandom -> characters combo made above)
// TODO: ...I'm gonna need a lot more fandoms and characters before taking the joke live...

// BACKLOG ----------------------------------------------------------------------------
// TODO: Add "Ship It" button and record shipped groupings on server... for science
// TODO: (Do I need a privacy policy for doing this?!)
// TODO: Randomly select a character from a fandom (per column)
// TODO: Randomly select fandom AND character (per column)
// TODO: Randomize the whole page's selections
// TODO: (Maybe just... "lock" checkboxes next to selected things?)
// TODO: Allow users to add missing characters and fandoms... (grimace)

// Data about a character in a given fandom
#[derive(Serialize, Deserialize, Debug)]
struct FandomCharacter {
    name: String,
}

// Data about a given fandom
#[derive(Serialize, Deserialize, Debug)]
struct Fandom {
    title: String,
    characters: Vec<FandomCharacter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllFandomData {
    fandoms: Arc<Vec<Fandom>>,
}


#[get("/")]
fn index(all_fandoms: State<AllFandomData>) -> Template {
    let context = all_fandoms;
    Template::render("index", &*context)
}

fn main() {
    let file = File::open("data/base_data.json").expect("Couldn't open file");
    let reader = BufReader::new(file);
    let fandoms: AllFandomData = serde_json::from_reader(reader).expect("Couldn't deserialize json");

    rocket::ignite()
    .manage(fandoms)
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}

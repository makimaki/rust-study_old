#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag="type")]
enum Event {
    #[serde(rename="message")]
    MessageEvent { text: String },
    #[serde(rename="postback")]
    PostbackEvent { data: i32, additional_condition: Option<String>, dummy: Option<i32> },
}

struct Point {
    x: i32,
    y: i32,
}
struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

#[get("/")]
fn index() -> &'static str {

    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    let handlers: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for handler in handlers {
        handler.join().unwrap();
    }

    "Hello, world!"
}

#[post("/json", )]

fn main() {
    let de: Event = serde_json::from_str(r#"{"type":"message", "text":"ミールさんについて"}"#).unwrap();
    println!("deserialized = {:?}", de);
    let se = serde_json::to_string(&de).unwrap();
    println!("serialized = {}", se);
    let de: Event = serde_json::from_str(r#"{"type":"postback", "data":1, "additional_condition": "!!!!"}"#).unwrap();
    println!("deserialized = {:?}", de);
    let se = serde_json::to_string(&de).unwrap();
    println!("serialized = {}", se);

    // let se = serde_json::to_string(&de).unwrap();
    // println!("serialized = {}", se);

    // let point = Point { x: 1, y: 2 };

    // // Convert the Point to a JSON string.
    // let serialized = serde_json::to_string(&point).unwrap();

    // // Prints serialized = {"x":1,"y":2}
    // println!("serialized = {}", serialized);

    // // Convert the JSON string back to a Point.
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // // Prints deserialized = Point { x: 1, y: 2 }
    // println!("deserialized = {:?}", deserialized);
    // rocket::ignite().mount("/", routes![index]).launch();
}

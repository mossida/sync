use crate::events::models::*;

pub fn send(event: Option<Event>) {
    println!("{}", event.unwrap().name)
}
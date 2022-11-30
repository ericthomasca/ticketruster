use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Default, Debug)]
struct Event {
    id: Uuid,
    performer: String,
    date: DateTime<Utc>,
    location: String,
    tickets_available: u32,
    tickets_sold: u32,
}

fn main() {
    let event = Event {
        id: Uuid::new_v4(),
        performer: String::from("Tool"),
        date: chrono::offset::Utc::now(),
        location: String::from("St. John's, NL"),
        tickets_available: 17_000,
        tickets_sold: 2500,
    };

    println!("{:?}", event);
}

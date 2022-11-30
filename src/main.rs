use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
struct Event {
    id: Uuid,
    performer: String,
    date: DateTime<Utc>,
    location: String,
    tickets_available: u32,
    tickets_sold: u32,
}

impl Event {
    pub fn new(
        id: Uuid,
        performer: String,
        date: DateTime<Utc>,
        location: String,
        tickets_available: u32,
        tickets_sold: u32,
    ) -> Event {
        Event {
            id,
            performer,
            date,
            location,
            tickets_available,
            tickets_sold,
        }
    }
}

fn main() {
    let event: Event = Event<Event as Trait>::new{id: Uuid::new_v4(), performer: String::from("Tool"), date: chrono::offset::Utc::now(), location: String::from(""), ticket_available: 17_000, tickets_sold: 2500};
  
    println!("{:?}", event);
}

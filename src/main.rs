use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug)]
struct Event {
    id: Uuid,
    performer: String,
    datetime: NaiveDateTime,
    location: String,
    tickets_available: u32,
    tickets_sold: u32,
}

fn main() {
    let event = Event {
        id: Uuid::new_v4(),
        performer: String::from("Tool"),
        datetime: NaiveDate::from_ymd_opt(2022, 12, 13).unwrap().and_hms_opt(20, 00, 00).unwrap(),
        location: String::from("St. John's, NL"),
        tickets_available: 17_000,
        tickets_sold: 2500,
    };

    println!("--- TICKETRUSTER ---");
    println!("ID: {:?}", event.id);
    println!("Performer: {:?}", event.performer);
    println!("Date and Time: {:?}", event.datetime);
    println!("Location: {:?}", event.location);
    println!("Tickets available: {:?}", event.tickets_available);
    println!("Tickets sold: {:?}", event.tickets_sold);
    
}

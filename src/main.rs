use to_do::structs::{done::Done, pending::Pending};

use crate::to_do::enums::TaskStatus;
mod to_do;

fn main() {
    let done = Done::new("Play football");
    let pending = Pending::new("Cooking dinner");

    println!(
        "Done {}\nPending {}",
        done.super_struct.title, pending.super_struct.title
    );
}

use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{create::Create, delete::Delete, get::Get},
    ItemTypes,
};
mod to_do;

fn main() {
    let to_do_items = to_do_factory("Play football", TaskStatus::PENDING);
    match to_do_items {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title)
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.create(&item.super_struct.title)
        }
    }
}

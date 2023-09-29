use crate::to_do::{
    enums::TaskStatus,
    traits::{create::Create, edit::Edit, get::Get},
};

use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base_struct = Base {
            title: title.to_string(),
            status: TaskStatus::PENDING,
        };

        return Pending {
            super_struct: base_struct,
        };
    }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}

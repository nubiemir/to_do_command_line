use crate::to_do::enums::TaskStatus;

use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base_struct = Base {
            title: title.to_string(),
            status: TaskStatus::DONE,
        };
        return Done {
            super_struct: base_struct,
        };
    }
}

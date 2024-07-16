use crate::to_do::traits::create::Create;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending { super_struct: base };
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}

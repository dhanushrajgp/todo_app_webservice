use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use serde::Serialize;
#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    return ToDoItems::getState();
}

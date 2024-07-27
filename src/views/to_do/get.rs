use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::Responder;
use crate::jwt::JwToken;
pub async fn get(token:JwToken) -> impl Responder {
    return ToDoItems::getState(token.user_id);
}

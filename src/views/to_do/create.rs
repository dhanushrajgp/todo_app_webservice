use crate::json_serialization::to_do_items::ToDoItems;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};
use actix_web::{HttpRequest, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();

    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    return HttpResponse::Ok().json(ToDoItems::getState());
}

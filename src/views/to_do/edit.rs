use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::new(&result.as_str().unwrap().to_string());
            let existing_item = to_do_factory(&to_do_item.title.as_str(), status.clone());
            if &status._stringify()
                == &TaskStatus::from_string(to_do_item.status.as_str().to_string())._stringify()
            {
                return HttpResponse::Ok().json(ToDoItems::getState());
            }
            process_input(existing_item, "edit".to_owned(), &state);
            return HttpResponse::Ok().json(ToDoItems::getState());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    }
}

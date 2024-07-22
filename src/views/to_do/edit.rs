use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use actix_web::{web, HttpResponse};
use crate::diesel;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let connection = establish_connection();
    let results = to_do::table
    .filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
    .set(to_do::columns::status.eq("DONE"))
    .execute(&connection);

    println!("here is the message token: {}", token.message);
    return HttpResponse::Ok().json(ToDoItems::getState());
}

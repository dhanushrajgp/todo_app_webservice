use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use actix_web::{web, HttpResponse};
use crate::diesel;
use diesel::prelude::*;
use crate::schema::to_do;
use crate::database::DB;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken,db:DB) -> HttpResponse {
    let results = to_do::table
    .filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
    .set(to_do::columns::status.eq("DONE"))
    .execute(&db.connection);

    println!("here is the message token: {}", token.message);
    return HttpResponse::Ok().json(ToDoItems::getState());
}

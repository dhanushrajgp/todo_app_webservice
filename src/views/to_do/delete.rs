use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use actix_web::{web, HttpResponse};
use crate::diesel;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::schema::to_do;
use crate::models::item::item::Item;
use crate::database::DB;



pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken,db:DB) -> HttpResponse {
    let connection = establish_connection();

    let items = to_do::table
    .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
    .filter(to_do::columns::user_id.eq(&token.user_id))
    .order(to_do::columns::id.asc())
    .load::<Item>(&db.connection)
    .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);
    return HttpResponse::Ok().json(ToDoItems::getState(token.user_id))
}

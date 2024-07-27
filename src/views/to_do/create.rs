use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::{HttpRequest, HttpResponse};
use  crate::diesel;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::jwt::JwToken;
use crate::database::DB;

pub async fn create(token:JwToken ,req: HttpRequest,db:DB) -> HttpResponse {

    let title = req.match_info().get("title").unwrap().to_string();
    let connection = establish_connection();
    let items = to_do::table
    .filter(to_do::columns::title.eq(&title.as_str()))
    .order(to_do::columns::id.asc())
    .load::<Item>(&connection)
    .unwrap();
    
    if items.len() == 0 {
        let new_post = NewItem::new(title,token.user_id);
        let _ = diesel::insert_into(to_do::table).values(&new_post)
        .execute(&db.connection);
    }

    return HttpResponse::Ok().json(ToDoItems::getState(token.user_id));
}

mod create;
use actix_web::web::{post, scope, ServiceConfig};

pub fn todo_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/item").route("/create/{title}", post().to(create::create)));
}

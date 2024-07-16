mod create;
use actix_web::web::{get, scope, ServiceConfig};

pub fn todo_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/item").route("/create/{title}", get().to(create::create)));
}

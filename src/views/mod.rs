mod app;
mod auth;
mod to_do;
use actix_web::web::ServiceConfig;
use app::app_views_factory;
use auth::auth_views_factory;
use to_do::todo_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    todo_views_factory(app);
    app_views_factory(app);
}

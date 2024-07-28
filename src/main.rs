#[macro_use] extern crate diesel;
extern crate dotenv;
use actix_web::{App, HttpServer,HttpResponse};
mod json_serialization;
mod jwt;
mod models;
mod to_do;
mod config;
mod views;
mod database;
use actix_service::Service;
use actix_cors::Cors;
mod schema;
use futures::future::{ok,Either};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str= include_str!("./output_data.txt");
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
        .allow_any_method()
        .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let passed:bool;
                if *&req.path().contains(&format!("/{}/",ALLOWED_VERSION)){
                    passed = true;
                }
                else{
                    passed = false;
                }
                let end_result= match passed {
                  true => { Either::Left(srv.call(req))}
                false=>{
                    let resp = HttpResponse::NotImplemented().body(format!("only {} API is supported",ALLOWED_VERSION));
                    Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                }
                    };
                async move{
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

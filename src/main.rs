use actix_web::{Responder,web,App,HttpServer,HttpResponse};

async fn req_handler_1() -> impl Responder{
    HttpResponse::Ok().body("Hello Outside World!")
}

async fn req_handler_2() -> impl Responder{
    HttpResponse::Ok().body("Hello Inside World!")
}

async fn req_handler_3() -> impl Responder{
    HttpResponse::Ok().body("Welcome!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .route("/outside", web::get().to(req_handler_1))
            .route("/inside", web::get().to(req_handler_2))
            .route("/welcome", web::get().to(req_handler_3))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
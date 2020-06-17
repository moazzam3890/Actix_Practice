use actix_web::{Responder,web,App,HttpServer,HttpResponse};

async fn req_handler_1() -> impl Responder{ //creation of a request handler
    HttpResponse::Ok().body("Hello Outside World!")
}

async fn req_handler_2() -> impl Responder{ //Returns a HTTP Response (i.e impl Responder)
    HttpResponse::Ok().body("Hello Inside World!")
}

async fn req_handler_3() -> impl Responder{ //Recieves 0 or more parameters from request (i.e.
    HttpResponse::Ok().body("Welcome!")  //impl From Request in .to())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{ //creation of new App instance.
        App::new()
        //Registration of request handler with a HTTP method and applications's route on path
            .route("/outside", web::get().to(req_handler_1))
            .route("/inside", web::get().to(req_handler_2))
            .route("/welcome", web::get().to(req_handler_3))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
/*An actix Microservice for Simple Calculator that has multiple routes:
A. / that turns a hello world message
B. /calculate/{string} that calculates the result of the string
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

//create a function that returns hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World! Calculators are fun!")
}

//create a function that returns the result of the calculation
#[get("/calculate/{input}")]
async fn calculate(input: web::Path<String>) -> impl Responder {
    let result = week4::calculate(input.to_string());
    HttpResponse::Ok().body(result.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(calculate))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

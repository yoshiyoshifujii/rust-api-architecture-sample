use crate::server::requests::PostDocumentRequest;
use crate::server::responses::DocumentIdDto;
use crate::server::RequestContext;
use crate::usecases;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/documents")]
async fn post_document(
    data: web::Data<RequestContext>,
    request: Json<PostDocumentRequest>,
) -> impl Responder {
    match usecases::documents::post_document(&mut data.document_repository(), &request.of()) {
        Ok(output) => HttpResponse::Ok().json(DocumentIdDto::new(&output.id)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

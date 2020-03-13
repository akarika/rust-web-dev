use actix_web::{Responder, HttpResponse};
use crate::models::product::{ProductList, Product};
use crate::models::product::NewProduct;
use actix_web::web;
use actix_web::post;

use actix_web::get;


pub async fn index() -> impl Responder {
    ProductList::list()
}



pub async fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {

    // we call the method create from NewProduct and map an ok status response when
    // everything works, but map the error from diesel error
    // to an internal server error when something fails.
    new_product
        .create()
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

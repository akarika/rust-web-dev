use actix_web::{Responder, HttpResponse};
use crate::models::product::ProductList;

use actix_web::get;

#[get("/products")]
pub async fn index() -> impl Responder {
    ProductList::list()
}

use crate::models::product::NewProduct;
use actix_web::web;
use actix_web::post;

#[post("/products")]
pub async fn create(new_product: web::Json<NewProduct>) -> impl Responder {
    NewProduct::create(&new_product)
}


use actix_web::{Responder, HttpResponse, web, Result};
use crate::models::product::{ProductList, Product};
use crate::models::product::NewProduct;

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


pub async fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    println!("banzai");
    print_type_of(&id);
    Product::find(&id)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
fn print_type_of<T>(_: &T) {
    println!("--- {}", std::any::type_name::<T>())
}

pub async fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    Product::destroy(&id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
pub async fn update(id: web::Path<i32>,new_product: web::Json<NewProduct>)-> Result<HttpResponse, HttpResponse>{
    Product::update(&id, &new_product)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
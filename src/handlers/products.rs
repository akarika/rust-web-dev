use actix_web::{Responder, HttpResponse, Result, HttpRequest, web};
use crate::models::product::{ProductList, Product};
use crate::models::product::NewProduct;
use crate::db_connection::{PgPool,PgPooledConnection};
use std::borrow::Borrow;


fn pg_pool_handler(pool: web::Data<PgPool>)->Result<PgPooledConnection,HttpResponse>{
    pool.get().map_err(|e|HttpResponse::InternalServerError().json(e.to_string()))
}


pub async fn index(_req:HttpRequest,pool : web::Data<PgPool>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool);

    ProductList::list(&pg_pool.unwrap().borrow())
}


pub async fn create(new_product: web::Json<NewProduct>,pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    // we call the method create from NewProduct and map an ok status response when
    // everything works, but map the error from diesel error
    // to an internal server error when something fails.
    new_product
        .create(&pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}


pub async fn show(id: web::Path<i32>,pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    println!("banzai");
    let pg_pool = pg_pool_handler(pool)?;
    Product::find(&id,&pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
fn print_type_of<T>(_: &T) {
    println!("--- {}", std::any::type_name::<T>())
}

pub async fn destroy(id: web::Path<i32>,pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Product::destroy(&id,&pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
pub async fn update(id: web::Path<i32>,new_product: web::Json<NewProduct>,pool: web::Data<PgPool>)-> Result<HttpResponse, HttpResponse>{
    let pg_pool = pg_pool_handler(pool)?;
    Product::update(&id, &new_product,&pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
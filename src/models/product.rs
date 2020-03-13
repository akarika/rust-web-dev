use crate::schema::products;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use crate::schema::products::dsl::*;
use crate::db_connection::establish_connection;
use actix_web::{Responder, HttpRequest, Error, HttpResponse};
use actix_web::body::Body;

use futures::future::{ready, Ready};
//use crate::handlers::products::create;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>, // For a value that can be null,
    // in Rust is an Option type that
    // will be None when the db value is null
}

#[derive(Insertable, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>
}

impl NewProduct {

    // Take a look at the method definition, I'm borrowing self,
    // just for fun remove the & after writing the handler and
    // take a look at the error, to make it work we would need to use into_inner (https://actix.rs/api/actix-web/stable/actix_web/struct.Json.html#method.into_inner)
    // which points to the inner value of the Json request.
    pub fn create(&self) -> Result<Product, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::db_connection::establish_connection;

        let connection = establish_connection();
        diesel::insert_into(products::table)
            .values(self)
            .get_result(&connection)
    }
}


#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        let connection = establish_connection();
        let result = products
            .limit(10)
            .load::<Product>(&connection)
            .expect("Error loading products");

        ProductList(result)
    }
}

impl Responder for ProductList {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
// Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}


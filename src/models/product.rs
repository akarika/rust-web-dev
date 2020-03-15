use crate::schema::products;
use diesel::{RunQueryDsl, PgConnection};
use diesel::QueryDsl;
use crate::schema::products::dsl::*;
use actix_web::{Responder, HttpRequest, Error, HttpResponse};

use futures::future::{ready, Ready};
use crate::db_connection::PgPooledConnection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>, // For a value that can be null,
    // in Rust is an Option type that
    // will be None when the db value is null
}

#[derive(Insertable, Deserialize,AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}

impl NewProduct {
    // Take a look at the method definition, I'm borrowing self,
    // just for fun remove the & after writing the handler and
    // take a look at the error, to make it work we would need to use into_inner (https://actix.rs/api/actix-web/stable/actix_web/struct.Json.html#method.into_inner)
    // which points to the inner value of the Json request.
    pub fn create(&self,connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        diesel::insert_into(products::table)
            .values(self)
            .get_result(connection)
    }
}


#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list(connection: &PgPooledConnection) -> Self {
        let result = products
            .limit(10)
            .load::<Product>(connection)
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

impl Product {
    pub fn find(i: &i32,connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        products.find(i).first(connection)
    }
    pub fn destroy(i:&i32,connection: &PgConnection)-> Result<(),diesel::result::Error>{
        diesel::delete(products.find(i)).execute(connection)?;
        Ok(())
    }
    pub fn update(i:&i32,new_product:&NewProduct,connection: &PgConnection)->Result<(), diesel::result::Error>{
        diesel::update(products.find(i))
        .set(new_product)
        .execute(connection)?;
        Ok(())
    }
}

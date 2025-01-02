use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, document, Bson, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;
mod user_controller;
mod user_service;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("msg: DATABASE_URL is not set in .env file");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    env::set_var("RUST_LOG", "actix_web=debug,actix_web=info");
    env_logger::init();
    println!("Server URL: {}", server_url);
    println!("Database URL: {}", database_url);
    let client_options = ClientOptions::parse(&database_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&database_name);
    let user_collection_name =
        env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let user_collection = db.collection(&user_collection_name);
    let app = App::new().wrap(middleware::Logger::default());
    HttpServer::new(move || App::new().wrap(middleware::Logger::default()))
        .bind(server_url)?
        .run()
        .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
}

fn user_to_document(user: &User) -> Document {
    let User {
        first_name,
        last_name,
        user_name,
        password,
        email,
    } = user;
    doc! {
        "firstName": first_name,
        "lastName": last_name,
        "userName": user_name,
        "password": password,
        "email": email,
    }
}

fn build_user(
    first_name: String,
    last_name: String,
    user_name: String,
    password: String,
    email: String,
) -> User {
    User {
        first_name,
        last_name,
        user_name,
        password,
        email,
    }
}

fn user_from_document(document: Document) -> User {
    let mut _first_name = "".to_string();
    let mut _last_name = "".to_string();
    let mut _user_name = "".to_string();
    let mut _password = "".to_string();
    let mut _email = "".to_string();
    if let Some(&Bson::String(ref first_name)) = document.get("firstName") {
        _first_name = first_name.to_string();
    }
    if let Some(&Bson::String(ref last_name)) = document.get("lastName") {
        _last_name = last_name.to_string();
    }
    if let Some(&Bson::String(ref user_name)) = document.get("userName") {
        _user_name = user_name.to_string();
    }
    if let Some(&Bson::String(ref password)) = document.get("password") {
        _password = password.to_string();
    }
    if let Some(&Bson::String(ref email)) = document.get("email") {
        _email = email.to_string();
    }
    build_user(_first_name, _last_name, _user_name, _password, _email)
}

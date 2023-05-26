use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{sync::{Client, Collection}};
use crate::models::user_model::User;
use crate::models::subscription_model::Subscription;

pub struct MongoRepo {
    pub col: Collection<User>,
    pub sub: Collection<Subscription>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        let sub: Collection<Subscription> = db.collection("Subscription");
        MongoRepo { col, sub }
    }
}

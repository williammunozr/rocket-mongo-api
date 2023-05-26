use crate::repository::mongodb_repo::MongoRepo;
use mongodb::{bson::{extjson::de::Error, oid::ObjectId, doc}, results::{InsertOneResult}};
use crate::models::subscription_model::Subscription;
use chrono::Utc;

impl MongoRepo {
    pub fn create_subscription(&self, new_subscription: Subscription) -> Result<InsertOneResult, Error> {
        let datetime = Utc::now();
        let new_doc = Subscription {
            id: None,
            email: new_subscription.email,
            name: new_subscription.name,
            subscribed_at: Option::Some(datetime.to_string()),
        };
        let subscription = self
            .sub
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating subscription");
        Ok(subscription)
    }

    pub fn get_subscription(&self, id: &String) -> Result<Subscription, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let subscription_detail = self
            .sub
            .find_one(filter, None)
            .ok()
            .expect("Error getting subscription's detail");
        Ok(subscription_detail.unwrap())
    }
}
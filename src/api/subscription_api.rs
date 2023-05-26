use crate::{models::subscription_model::Subscription, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/subscription", data = "<new_subscription>")]
pub fn create_subscription(
    db: &State<MongoRepo>,
    new_subscription: Json<Subscription>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Subscription {
        id: None,
        email: new_subscription.email.to_owned(),
        name: new_subscription.name.to_owned(),
        subscribed_at: None,
    };
    let subscription_detail = db.create_subscription(data);
    match subscription_detail {
        Ok(subscription) => Ok(Json(subscription)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/subscription/<path>")]
pub fn get_subscription(db: &State<MongoRepo>, path: String) -> Result<Json<Subscription>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let subscription_detail = db.get_subscription(&id);
    match subscription_detail {
        Ok(subscription) => Ok(Json(subscription)),
        Err(_) => Err(Status::InternalServerError),
    }
}

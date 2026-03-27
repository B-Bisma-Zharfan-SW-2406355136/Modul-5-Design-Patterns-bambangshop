use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_string: &str = product_type_upper.as_str();
        let subscriber_result: Subscriber = SubscriberRepository::add(product_type_string, subscriber);
        return Ok(subscriber_result);
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_string: &str = product_type_upper.as_str();
        let subscriber_result: Option<Subscriber> = SubscriberRepository::delete(product_type_string, url);
        if subscriber_result.is_none() {
            return Err(compose_error_response(Status::NotFound, String::from("Subscriber not found")));
        }
        return Ok(subscriber_result.unwrap());
    }

    pub fn notify(&self, product_type: &str, product: Product, status: &str) {
        let mut payload: Notification = Notification {
            product_type: String::from(product_type),
            product_title: product.clone().title,
            product_url: product.clone().get_url(),
            subscriber_name: String::from(""),
            status: String::from(status),
        };

        let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type, "");
        for subscriber in subscribers {
            payload.subscriber_name = subscriber.name.clone();
            let subscriber_clone = subscriber.clone();
            let payload_clone = payload.clone();
            thread::spawn(move || subscriber_clone.update(payload_clone));
        }
    }
}
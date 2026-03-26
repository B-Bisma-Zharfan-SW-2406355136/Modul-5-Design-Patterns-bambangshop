use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::notification::Notification;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}
#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::events;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub time: String
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "events"]
pub struct InsertableEvent {
    pub name: String,
    pub date: String,
    pub time: String
}
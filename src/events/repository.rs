#![allow(proc_macro_derive_resolution_fallback)]

use crate::events::{Event, InsertableEvent};
use crate::schema::{events};
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Event>> {
    events::table.load::<Event>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Event> {
    events::table.find(id).get_result::<Event>(connection)
}

pub fn insert(event: InsertableEvent, connection: &PgConnection) -> QueryResult<Event> {
    diesel::insert_into(events::table)
        .values(event)
        .get_result(connection)
}

pub fn update(id: i32, event: InsertableEvent, connection: &PgConnection) -> QueryResult<Event> {
    diesel::update(events::table.find(id))
        .set(&event)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(events::table.find(id)).execute(connection)
}

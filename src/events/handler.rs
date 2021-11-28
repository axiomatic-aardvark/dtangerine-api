use crate::events;
use crate::events::{Event, InsertableEvent};
use crate::connection::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::env;

#[get("/events")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Event>>, Status> {
    events::repository::all(&connection)
        .map(|events| Json(events))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    println!("{:?}", error);
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/events", format = "application/json", data = "<event>")]
pub fn post(
    event: Json<InsertableEvent>,
    connection: DbConn,
) -> Result<status::Created<Json<Event>>, Status> {
    events::repository::insert(event.into_inner(), &connection)
        .map(|event| event_created(event))
        .map_err(|error| error_status(error))
}

fn event_created(event: Event) -> status::Created<Json<Event>> {
    status::Created(
        format!(
            "{host}:{port}/events/{id}",
            host = host(),
            port = port(),
            id = event.id
        )
        .to_string(),
        Some(Json(event)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/events/<id>", format = "application/json", data = "<event>")]
pub fn put(
    id: i32,
    event: Json<InsertableEvent>,
    connection: DbConn,
) -> Result<Json<Event>, Status> {
    events::repository::update(id, event.into_inner(), &connection)
        .map(|event| Json(event))
        .map_err(|error| error_status(error))
}

#[delete("/events/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match events::repository::get(id, &connection) {
        Ok(_) => events::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}

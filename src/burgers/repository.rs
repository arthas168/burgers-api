#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::burgers;
use crate::burgers::Burger;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Burger>> {
    burgers::table.load::<Burger>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Burger> {
    burgers::table.find(id).get_result::<Burger>(connection)
}

pub fn insert(burger: Burger, connection: &PgConnection) -> QueryResult<Burger> {
    diesel::insert_into(burgers::table)
        .values(&InsertableBurger::from_burger(burger))
        .get_result(connection)
}

pub fn update(id: i32, burger: Burger, connection: &PgConnection) -> QueryResult<Burger> {
    diesel::update(burgers::table.find(id))
        .set(&burger)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(burgers::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "burgers"]
struct InsertableBurger {
    name: String,
    description: String,
}

impl InsertableBurger {

    fn from_burger(burger: Burger) -> InsertableBurger {
        InsertableBurger {
            name: burger.name,
            description: burger.description,
        }
    }
}
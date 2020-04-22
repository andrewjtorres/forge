use juniper::{self, RootNode};

pub type Schema = RootNode<'static, Query, Mutation>;

#[derive(Debug)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn mutation() -> bool {
        true
    }
}

#[derive(Debug)]
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn query() -> bool {
        true
    }
}

pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

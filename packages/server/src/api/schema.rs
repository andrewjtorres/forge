use juniper::{self, EmptyMutation, RootNode};

pub struct Context;

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn version() -> &'static str {
        "1"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

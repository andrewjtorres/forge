use juniper::{self, RootNode};
use std::sync::Arc;

use crate::database::pool::PooledConnection;

pub type Schema = RootNode<'static, Query, Mutation>;

pub struct Context {
    pub connection: Arc<PooledConnection>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(connection: PooledConnection) -> Self {
        Self {
            connection: Arc::new(connection),
        }
    }
}

pub struct Mutation;

/// The mutation root of the GraphQL interface.
#[juniper::object(Context = Context)]
impl Mutation {
    fn mutation() -> bool {
        true
    }
}

pub struct Query;

/// The query root of the GraphQL interface.
#[juniper::object(Context = Context)]
impl Query {
    fn query() -> bool {
        true
    }
}

pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

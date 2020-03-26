use chrono::{DateTime, Utc};
use diesel::{self, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

diesel::table! {
    /// Users of the application.
    users (id) {

        /// The unique identifier of the user.
        id -> Uuid,

        /// The user's primary email address.
        email -> Varchar,

        /// The user's encoded password hash.
        hash -> Bytea,

        /// The date and time when the user was created.
        created_at -> Timestamptz,

        /// The date and time when the user was last updated.
        updated_at -> Timestamptz,
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,

    pub hash: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,

    pub email: String,

    pub hash: Vec<u8>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

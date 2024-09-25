//!# DB
//!A generic and reusable postgres ORM for rust applications.
//!
//!
//!## Example
//!```rust
//!use db::PostgresDB;
//!
//!let connection_string = format!(
//!        "host={} user={} password={}",
//!        "127.0.0.1", "someUser", "somePassword"
//!    )
//!
//!let postgres_db = PostgresDB::new()
//!    .setup(connection_string)
//!    .await;
//!
//!if postgres_db.pool.is_some() {
//!    //Do something with the postgres_db
//!}
//!
//!```
//!
pub mod db;

//# DB Types
//The types used by the package
pub mod types;

//#PostgresDB
//An object that provides generic access to query, query_single, and execute commands
pub use crate::db::PostgresDB;

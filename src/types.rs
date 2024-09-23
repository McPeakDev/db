use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls, Row};

pub type PostgresDBPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DBResult<T> = Result<T, String>;
pub type QueryParams<'a> = Option<&'a [&'a (dyn ToSql + Sync)]>;
pub type DBRow = Row;
pub struct DatabaseConnection(pub PooledConnection<'static, PostgresConnectionManager<NoTls>>);
pub struct DB {
    pub host: String,
    pub user: String,
    pub password: String,
}

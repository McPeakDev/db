use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls, Row};

pub type PostgresDBPool = Pool<PostgresConnectionManager<NoTls>>;
pub type PostgresDB = PostgresConnectionManager<NoTls>;

pub type DBResult<T> = Result<T, (u16, String)>;
pub type QueryParams<'a> = Option<&'a [&'a (dyn ToSql + Sync)]>;
pub type DBRow = Row;

pub type ConnectionPool = PooledConnection<'static, PostgresConnectionManager<NoTls>>;

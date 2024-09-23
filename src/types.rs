use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use serde::Deserialize;
use tokio_postgres::{types::ToSql, NoTls, Row};

pub type PostgresDBPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DBResult<T> = Result<T, (StatusCode, String)>;
pub type QueryParams<'a> = Option<&'a [&'a (dyn ToSql + Sync)]>;
pub type DBRow = Row;
pub struct DatabaseConnection(pub PooledConnection<'static, PostgresConnectionManager<NoTls>>);
#[derive(Deserialize)]
pub struct DB {
    pub host: String,
    pub user: String,
    pub password: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PostgresDBPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PostgresDBPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

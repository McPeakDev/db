use crate::types::{DBResult, DatabaseConnection, PostgresDBPool, QueryParams, DB};
use axum::http::StatusCode;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{row::Row, types::ToSql, Error, NoTls};
use tracing::{debug, error};

pub async fn setup(config: &DB) -> PostgresDBPool {
    let manager = PostgresConnectionManager::new_from_stringlike(
        format!(
            "host={} user={} password={}",
            config.host, config.user, config.password
        ),
        NoTls,
    )
    .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    return pool;
}

pub async fn query<'a, T: From<Row>>(
    DatabaseConnection(ref conn): DatabaseConnection,
    query: &str,
    query_params: QueryParams<'a>,
) -> DBResult<Vec<T>> {
    let params = query_params.unwrap_or(&[]);

    let db_result = conn.query(query, params).await;

    handle_query_debug(query, query_params);

    match db_result {
        Ok(rows) => {
            let num_of_rows = rows.len();

            if num_of_rows > 0 {
                let mut results: Vec<T> = vec![];

                for row in rows {
                    let item = row.into();
                    results.push(item);
                }

                return Ok(results);
            }

            return Err((StatusCode::NOT_FOUND, String::from("No results were found")));
        }
        Err(e) => {
            let err = handle_db_error(e);
            error!("DB Error: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
        }
    }
}

pub async fn query_single<'a, T: From<Row>>(
    DatabaseConnection(ref conn): DatabaseConnection,
    query: &str,
    query_params: QueryParams<'a>,
) -> DBResult<T> {
    let params = query_params.unwrap_or(&[]);

    let db_result = conn.query(query, params).await;

    handle_query_debug(query, query_params);

    match db_result {
        Ok(rows) => {
            let num_of_rows = rows.len();

            if num_of_rows > 0 {
                let item = rows[0].clone().into();
                return Ok(item);
            }

            return Err((StatusCode::NOT_FOUND, String::from("No results were found")));
        }
        Err(e) => {
            let err = handle_db_error(e);
            error!("DB Error: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
        }
    }
}

pub async fn execute<'a>(
    DatabaseConnection(ref conn): DatabaseConnection,
    query: &str,
    query_params: QueryParams<'a>,
) -> Result<u64, (StatusCode, String)> {
    let params = query_params.unwrap_or(&[]);

    let db_result = conn.execute(query, params).await;

    handle_query_debug(query, query_params);

    match db_result {
        Ok(rows) => Ok(rows),
        Err(e) => {
            let err = handle_db_error(e);
            error!("DB Error: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
        }
    }
}

fn handle_db_error(e: Error) -> String {
    let db_error_option = e.as_db_error();

    match db_error_option {
        Some(err) => {
            return err.message().to_string();
        }
        None => return e.to_string(),
    }
}

fn handle_query_debug<'a>(query: &str, params_option: QueryParams<'a>) {
    match params_option {
        Some(params) => {
            debug!("Query DB: `{}` with params: `{:?}`", query, params);
        }
        None => debug!("Query DB: `{}`", query),
    }
}

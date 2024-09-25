use crate::types::{ConnectionPool, DBResult, PostgresDBPool, QueryParams};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{row::Row, Error, NoTls};
use tracing::{debug, error};

/// ## PostgresDB
/// An object that provides generic access to query, query_single, and execute commands
#[derive(Clone)]
pub struct PostgresDB {
    connection_string: Option<String>,
    pub pool: Option<PostgresDBPool>,
}

impl PostgresDB {
    /// Create a new PostgresDB instance
    pub fn new() -> Self {
        return Self {
            connection_string: None,
            pool: None,
        };
    }

    /// ## setup
    /// Setup the database with a given connection_string
    pub async fn setup(&self, connection_string: String) -> Self {
        let mut self_clone = self.clone();

        let manager =
            PostgresConnectionManager::new_from_stringlike(&connection_string, NoTls).unwrap();

        let pool = Pool::builder().build(manager).await.unwrap();

        self_clone.pool = Some(pool);
        self_clone.connection_string = Some(connection_string);

        return self_clone;
    }

    async fn get_pool(&self) -> Result<ConnectionPool, String> {
        let pool_option = &self.pool;

        return match pool_option {
            Some(pool) => Ok(pool
                .get_owned()
                .await
                .expect("A DB Connection has not been setup")),
            None => Err(String::from("A DB Connection has not been setup")),
        };
    }

    /// ## query
    /// Query the database for results of the given type
    pub async fn query<'a, T: From<Row>>(
        &self,
        query: &str,
        query_params: QueryParams<'a>,
    ) -> DBResult<Vec<T>> {
        let pool_option = self.get_pool().await;

        if pool_option.is_ok() {
            let pool = pool_option.unwrap();

            let params = query_params.unwrap_or(&[]);

            let db_result = pool.query(query, params).await;

            Self::handle_query_debug(query, query_params);

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

                    return Err((404, String::from("No results were found")));
                }
                Err(e) => {
                    let err = Self::handle_db_error(e);
                    error!("DB Error: {}", err);
                    return Err((500, err));
                }
            }
        }
        let err = pool_option.err().unwrap();
        error!("DB Error: {}", err);
        return Err((500, err));
    }

    /// ## query_single
    /// Query the database for a single result of the given type
    pub async fn query_single<'a, T: From<Row>>(
        &self,
        query: &str,
        query_params: QueryParams<'a>,
    ) -> DBResult<T> {
        let pool_option = self.get_pool().await;

        if pool_option.is_ok() {
            let pool = pool_option.unwrap();

            let params = query_params.unwrap_or(&[]);

            let db_result = pool.query(query, params).await;

            Self::handle_query_debug(query, query_params);

            match db_result {
                Ok(rows) => {
                    let num_of_rows = rows.len();

                    if num_of_rows > 0 {
                        let item = rows[0].clone().into();
                        return Ok(item);
                    }

                    return Err((400, String::from("No results were found")));
                }
                Err(e) => {
                    let err = Self::handle_db_error(e);
                    error!("DB Error: {}", err);
                    return Err((500, err));
                }
            }
        }
        let err = pool_option.err().unwrap();
        error!("DB Error: {}", err);
        return Err((500, err));
    }
    /// ## execute
    /// Commit data to the database.
    pub async fn execute<'a>(&self, query: &str, query_params: QueryParams<'a>) -> DBResult<u64> {
        let pool_option = self.get_pool().await;

        if pool_option.is_ok() {
            let pool = pool_option.unwrap();
            let params = query_params.unwrap_or(&[]);

            let db_result = pool.execute(query, params).await;

            Self::handle_query_debug(query, query_params);

            return match db_result {
                Ok(rows) => Ok(rows),
                Err(e) => {
                    let err = Self::handle_db_error(e);
                    error!("DB Error: {}", err);
                    return Err((500, err));
                }
            };
        }
        let err = pool_option.err().unwrap();
        error!("DB Error: {}", err);
        return Err((500, err));
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
}

use std::collections::HashMap;
use std::hash::Hash;

use log::info;
use sqlx::{Column, Pool, Postgres, Row, TypeInfo, ValueRef};
use sqlx::postgres::{PgPoolOptions, PgRow};

/// Connect to database
pub async fn connect(db: &str, username: &str, password: &str, host: &str, port: &str, db_name: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let start_time = std::time::Instant::now();

    info!("Connecting: {}:{} | Database:{} | User:{}", host, port, db_name, username);

    let url = format!("{}://{}:{}@{}:{}/{}", db, username, password, host, port, db_name);

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url).await?;

    let used_time = start_time.elapsed().as_millis();

    info!("Time used for connecting database: {} ms", used_time);

    Ok(pool)
}

/// Store value in enum
/// so SQL result can be in HashMap<String, SqlResult>
#[derive(Debug)]
pub enum SqlResult {
    BOOL(bool),
    String(String),
    I32(i32),
    DATE(chrono::NaiveDate),
    TIME(chrono::NaiveTime),
    Null(),
    UnknownType(),
}

/// convert PgRow into HashMap<String, SqlResult>
fn into_hashmap(row: PgRow) -> HashMap<String, SqlResult> {
    let mut result: HashMap<String, SqlResult> = HashMap::new();

    for column in row.columns() {
        // print column data types
        // info!("{:16} - {}",column.name(),column.type_info().name());

        let result_value: SqlResult;

        // if the value is null
        if row.try_get_raw(column.name()).unwrap().is_null() {
            result_value = SqlResult::Null();
        } else {
            // if the value is not null
            // use match to get desired type value and put it into hashmap
            match column.type_info().name() {
                "BOOL" => {
                    let value: bool = row.get(column.name());
                    result_value = SqlResult::BOOL(value);
                }
                "TEXT" => {
                    let value: String = row.get(column.name());
                    result_value = SqlResult::String(value);
                }
                "INT4" => {
                    let value: i32 = row.get(column.name());
                    result_value = SqlResult::I32(value);
                }
                "DATE" => {
                    let value: chrono::NaiveDate = row.get(column.name());
                    result_value = SqlResult::DATE(value);
                }
                "TIME" => {
                    let value: chrono::NaiveTime = row.get(column.name());
                    result_value = SqlResult::TIME(value);
                }
                _ => {
                    info!("Can't convert SQL type {} to Rust type.", column.type_info().name());
                    result_value = SqlResult::UnknownType();
                }
            }
        }
        result.insert(column.name().to_string(), result_value);
    }

    return result;
}

/// Query SQL and return HashMap<String, SqlResults>
pub async fn query(pool: Pool<Postgres>, sql: &str) -> Result<Vec<HashMap<String, SqlResult>>, sqlx::Error> {
    let start_time = std::time::Instant::now();

    info!("Querying: {}", sql);

    let rows = sqlx::query(sql)
        .fetch_all(&pool)
        .await?;

    let mut results = Vec::new();

    for row in rows {
        let result = into_hashmap(row);
        results.push(result);
    }

    let used_time = start_time.elapsed().as_millis();

    info!("Time used for the query: {} ms", used_time);

    Ok(results)
}
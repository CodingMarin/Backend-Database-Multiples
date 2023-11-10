use anyhow::{anyhow, Result};
use serde::Deserialize;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::query_file;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use std::fmt;
use warp::reject::Rejection;

#[derive(Debug, Deserialize)]
pub struct DatabaseConnection {
    choice: u32,
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

#[derive(Debug)]
struct MyError(anyhow::Error);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.0)
    }
}

impl warp::reject::Reject for MyError {}

pub async fn handle_connect(
    db_connection: DatabaseConnection,
) -> Result<impl warp::Reply, Rejection> {
    run_database_program(&db_connection).await
}

async fn run_database_program(
    db_connection: &DatabaseConnection,
) -> Result<impl warp::Reply, Rejection> {
    match db_connection.choice {
        1 => {
            let options = SqliteConnectOptions::new()
                .filename(
                    db_connection
                        .database
                        .as_ref()
                        .ok_or_else(|| MyError(anyhow!("Database name is required")))
                        .map_err(|e| warp::reject::custom::<MyError>(e.into()))?
                        .as_str(),
                )
                .create_if_missing(true);

            connect_and_execute(options)
                .await
                .map_err(|e| warp::reject::custom(MyError(e)))
        }
        2 => {
            let host = db_connection
                .host
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Host is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let username = db_connection
                .username
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Username is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let password = db_connection
                .password
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Password is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let database = db_connection
                .database
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Database name is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;

            let options = MySqlConnectOptions::new()
                .host(host)
                .username(username)
                .password(password)
                .database(database);

            connect_and_execute(options)
                .await
                .map_err(|e| warp::reject::custom(MyError(e)))
        }

        3 => {
            let host = db_connection
                .host
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Host is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let username = db_connection
                .username
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Username is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let password = db_connection
                .password
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Password is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;
            let database = db_connection
                .database
                .as_ref()
                .ok_or_else(|| MyError(anyhow!("Database name is required")))
                .map_err(|e| warp::reject::custom::<MyError>(e.into()))?;

            let options = PgConnectOptions::new()
                .host(host)
                .username(username)
                .password(password)
                .database(database);

            connect_and_execute(options)
                .await
                .map_err(|e| warp::reject::custom(MyError(e)))
        }
        _ => Err(warp::reject::custom(MyError(anyhow!(
            "Invalid choice: {}",
            db_connection.choice
        )))),
    }
}


async fn connect_and_execute<O>(options: O) -> Result<String>
where
O: ConnectOptions,
<O as ConnectOptions>::Connection: Sized,
{
let conn = options.connect().await?;

    // Aquí ejecutas una consulta en la base de datos, por ejemplo, SELECT table_name FROM information_schema.tables;
    let result = sqlx::query("SELECT user FROM information_schema.tables;")
    .execute(&'static conn)
    .await?;

    // Transforma los resultados en una cadena para devolverlos
    let result_string = result
    .into_iter()
    .map(|row| row.table_name.unwrap_or_default())
    .collect::<Vec<String>>()
    .join(", ");

    Ok(format!(
        "Operación realizada con éxito. Tablas encontradas: {}",
        result_string
    ))
}

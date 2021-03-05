use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::{http, Filter};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub version: String,
    pub topic: Option<String>,
    pub content: String,
    pub time_recived: u64,
}

fn get_dummy_data() -> Result<Data, String> {
    Ok(Data {
        version: "".to_string(),
        content: "".to_string(),
        topic: None,
        time_recived: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

fn init_db() -> Result<Connection, String> {
    match rusqlite::Connection::open("db.sqlite") {
        Ok(conn) => match conn.execute(
            "CREATE TABLE IF NOT EXISTS reports (
        id INTEGER PRIMARY KEY,
        version TEXT NOT NULL,
        topic TEXT,
        content TEXT NOT NULL,
        time_recived INTEGER  NOT NULL
      )",
            NO_PARAMS,
        ) {
            Ok(_) => Ok(conn),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

async fn get_report() -> Result<impl warp::Reply, warp::Rejection> {
    println!("nono");
    match get_dummy_data() {
        Ok(val) => Ok(warp::reply::with_status(
            warp::reply::json(&val),
            http::StatusCode::OK,
        )),
        Err(val) => Ok(warp::reply::with_status(
            warp::reply::json(&val),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // panic if db init fail
    let conn = init_db().unwrap();

    conn.execute(
        "INSERT INTO reports (version, topic, content, time_recived) VALUES (?1, ?2, ?3, ?4)",
        params!["v1", "test", "some content", "1222222"],
    );

    let v1 = warp::path("v1").and(warp::path("report").and_then(get_report));

    warp::serve(v1).run(([0, 0, 0, 0], 3030)).await;
}

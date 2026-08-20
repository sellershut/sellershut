use super::*;
use serde_json::json;

#[track_caller]
fn check_max_connections(input: u32, expected: u32) {
    let actual = MaxConnections(input);
    assert_eq!(expected, actual.0);
}

#[test]
fn max_connections_default() {
    check_max_connections(MaxConnections::default().0, 100);
}

#[track_caller]
fn check_database_url(input: &str, expected: &str) {
    let config: DatabaseConfig = serde_json::from_value(json!({
        "url": input,
    }))
    .unwrap();

    match config {
        DatabaseConfig::Url { url } => {
            assert_eq!(expected, url.as_str());
        }
        DatabaseConfig::Connection { .. } => {
            panic!("expected Url variant");
        }
    }
}

#[test]
fn database_config_url() {
    check_database_url(
        "postgres://postgres:password@localhost:5432/sellershut",
        "postgres://postgres:password@localhost:5432/sellershut",
    );
}

#[track_caller]
fn check_database_connection(username: &str, password: &str, host: &str, db_name: &str) {
    let config: DatabaseConfig = serde_json::from_value(json!({
        "username": username,
        "password": password,
        "host": host,
        "db_name": db_name,
    }))
    .unwrap();

    match config {
        DatabaseConfig::Connection {
            username: actual_username,
            password: actual_password,
            host: actual_host,
            db_name: actual_db_name,
        } => {
            assert_eq!(username, actual_username);
            assert_eq!(password, actual_password.expose());
            assert_eq!(host, actual_host);
            assert_eq!(db_name, actual_db_name);
        }
        DatabaseConfig::Url { .. } => {
            panic!("expected Connection variant");
        }
    }
}

#[test]
fn database_config_connection() {
    check_database_connection("postgres", "secret", "localhost", "sellershut");
}

#[track_caller]
fn check_config_url(url: &str, max_connections: u32) {
    let config: Config = serde_json::from_value(json!({
        "url": url,
        "max_connections": max_connections,
    }))
    .unwrap();

    match config.database {
        DatabaseConfig::Url { url: actual_url } => {
            assert_eq!(url, actual_url.as_str());
        }
        DatabaseConfig::Connection { .. } => {
            panic!("expected Url variant");
        }
    }

    assert_eq!(max_connections, config.max_connections.0);
}

#[test]
fn config_url() {
    check_config_url(
        "postgres://postgres:password@localhost:5432/sellershut",
        100,
    );
    check_config_url("postgres://user:secret@db.example.com:5433/mydb", 25);
}

#[track_caller]
fn check_config_connection(
    username: &str,
    password: &str,
    host: &str,
    db_name: &str,
    max_connections: u32,
) {
    let config: Config = serde_json::from_value(json!({
        "username": username,
        "password": password,
        "host": host,
        "db_name": db_name,
        "max_connections": max_connections,
    }))
    .unwrap();

    match config.database {
        DatabaseConfig::Connection {
            username: actual_username,
            password: actual_password,
            host: actual_host,
            db_name: actual_db_name,
        } => {
            assert_eq!(username, actual_username);
            assert_eq!(password, actual_password.expose());
            assert_eq!(host, actual_host);
            assert_eq!(db_name, actual_db_name);
        }
        DatabaseConfig::Url { .. } => {
            panic!("expected Connection variant");
        }
    }

    assert_eq!(max_connections, config.max_connections.0);
}

#[test]
fn config_connection() {
    check_config_connection("postgres", "secret", "localhost", "sellershut", 100);
    check_config_connection("app", "another-secret", "db.example.com", "production", 25);
}

#[track_caller]
fn check_config_default(expected_url: &str, expected_max_connections: u32) {
    let config = Config::default();

    match config.database {
        DatabaseConfig::Url { url } => {
            assert_eq!(expected_url, url.as_str());
        }
        DatabaseConfig::Connection { .. } => {
            panic!("expected Url variant");
        }
    }

    assert_eq!(expected_max_connections, config.max_connections.0);
}

#[test]
fn config_default() {
    check_config_default(
        "postgres://postgres:password@localhost:5432/sellershut",
        100,
    );
}

#[tokio::test]
#[ignore = "requires a live database"]
async fn connect_with_url() {
    let config = Config {
        database: DatabaseConfig::Url {
            url: Url::parse("postgres://postgres:password@localhost:5432/postgres").unwrap(),
        },
        max_connections: MaxConnections::default(),
    };

    config.connect().await.unwrap();
}

#[tokio::test]
#[ignore = "requires a live database"]
async fn connect_with_connection() {
    let config = Config {
        database: DatabaseConfig::Connection {
            username: "postgres".into(),
            password: String::from("password").into(),
            host: "localhost".into(),
            db_name: "postgres".into(),
        },
        max_connections: MaxConnections::default(),
    };

    config.connect().await.unwrap();
}

#[tokio::test]
#[ignore = "slow"]
async fn connect_url_error() {
    let config = Config {
        database: DatabaseConfig::Url {
            url: Url::parse("postgres://invalid:invalid@localhost:1/invalid").unwrap(),
        },
        max_connections: MaxConnections::default(),
    };

    assert!(config.connect().await.is_err());
}

#[tokio::test]
#[ignore = "slow"]
async fn connect_connection_error() {
    let config = Config {
        database: DatabaseConfig::Connection {
            username: "invalid".into(),
            password: String::from("invalid").into(),
            host: "localhost".into(),
            db_name: "invalid".into(),
        },
        max_connections: MaxConnections::default(),
    };

    assert!(config.connect().await.is_err());
}

use super::*;
use sqlx::{Encode, Postgres, postgres::PgArgumentBuffer};

#[test]
fn expose_returns_original_secret() {
    let secret = RedactedSecret::from("super-secret".to_owned());

    assert_eq!(secret.expose(), "super-secret");
}

#[test]
fn from_string_preserves_secret() {
    let value = "my-secret".to_owned();
    let secret = RedactedSecret::from(value.clone());

    assert_eq!(secret.expose(), value);
}

#[cfg(feature = "serde")]
#[test]
fn serialize_redacts_secret() {
    let secret = RedactedSecret::from("super-secret".to_owned());

    let serialized = serde_json::to_string(&secret).unwrap();

    assert_eq!(serialized, r#""""#);
    assert!(!serialized.contains("super-secret"));
}

#[test]
fn postgres_type_is_compatible_with_text() {
    let type_info = <String as Type<Postgres>>::type_info();

    assert!(RedactedSecret::compatible(&type_info));
}

#[test]
fn encode_contains_secret_value() {
    let secret = RedactedSecret::from("super-secret".to_owned());
    let mut buf = PgArgumentBuffer::default();

    let result = secret.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    assert!(!buf.is_empty());
}

#[test]
fn encode_empty_secret_succeeds() {
    let secret = RedactedSecret::from(String::new());
    let mut buf = PgArgumentBuffer::default();

    let result = secret.encode_by_ref(&mut buf);

    assert!(result.is_ok());
}

tonic::include_proto!("common.pagination");

#[cfg(feature = "rpc-server-categories")]
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use cursor::Index;

/// Pagination cursor
#[derive(Debug, PartialEq, Eq)]
#[cfg(feature = "rpc-server-categories")]
pub struct CursorBuilder {
    id: String,
    dt: String,
}

#[cfg(feature = "rpc-server-categories")]
impl CursorBuilder {
    /// Create cursor
    pub fn new(id: &str, dt: &str) -> Self {
        Self {
            id: id.to_string(),
            dt: dt.to_string(),
        }
    }
    /// decode a cursor
    pub fn decode(
        params: &cursor::cursor_value::CursorType,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cursor = match params {
            cursor::cursor_value::CursorType::After(cursor) => cursor,
            cursor::cursor_value::CursorType::Before(cursor) => cursor,
        };

        let bytes = BASE64_URL_SAFE_NO_PAD.decode(cursor)?;

        let decoded = String::from_utf8(bytes)?;

        let mut tokens = decoded.split('|');
        if let (Some(dt), Some(id)) = (tokens.next(), tokens.next()) {
            Ok(Self {
                id: id.to_string(),
                dt: dt.parse()?,
            })
        } else {
            Err("missing tokens".into())
        }
    }

    /// get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// get date time
    pub fn dt(&self) -> &str {
        &self.dt
    }

    /// encode a cursor
    pub fn encode(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(format!("{}|{}", self.dt, self.id))
    }

    /// Gets pagination direction
    pub fn is_paginating_from_left(pagination: &Cursor) -> bool {
        let index = pagination.index.expect("index should exist");

        match &pagination.cursor_value {
            Some(value) => match &value.cursor_type {
                Some(cursor::cursor_value::CursorType::After(_)) => {
                    matches!(index, Index::First(_))
                }
                Some(cursor::cursor_value::CursorType::Before(_)) => {
                    matches!(index, Index::First(_))
                }
                None => matches!(index, Index::First(_)),
            },
            None => matches!(index, Index::First(_)),
        }
    }

    /// Checks if cursor is unavailable
    pub fn is_cursor_unavailable(pagination: &Cursor) -> bool {
        if let Some(value) = pagination.cursor_value.as_ref() {
            match value.cursor_type.as_ref() {
                Some(val) => match val {
                    cursor::cursor_value::CursorType::After(_) => false,
                    cursor::cursor_value::CursorType::Before(_) => false,
                },
                None => true,
            }
        } else {
            true
        }
    }
}

/// Gets maximum query results from pagination data
pub fn query_count(max: i32, pagination: &Index) -> i32 {
    let user_param = match pagination {
        Index::First(value) => value,
        Index::Last(value) => value,
    };
    if *user_param > max {
        max
    } else {
        *user_param
    }
}

#[cfg(test)]
mod tests {
    use super::{cursor::cursor_value::CursorType, CursorBuilder};

    #[test]
    fn test_cursor() {
        let cursor =
            CursorBuilder::new("9ckyrhcx6jun6n_7a8adq", "2023-01-11T11:32:07.853915+00:00");

        let encode = cursor.encode();

        let cursor_type = CursorType::After(encode);
        let decode = CursorBuilder::decode(&cursor_type).unwrap();

        assert_eq!(decode, cursor);
    }
}

#[cfg(feature = "users")]
pub mod users;

#[cfg(all(feature = "base", any(feature = "users")))]
pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

        #[cfg(feature = "time")]
        pub mod utils {

            use crate::google::protobuf::Timestamp;
            use time::{Error, OffsetDateTime};

            pub fn to_offset_datetime(timestamp: Timestamp) -> Result<OffsetDateTime, Error> {
                let seconds = timestamp.seconds;
                let nanos = timestamp.nanos as i64;
                let nanoseconds = nanos % 1_000_000_000;
                let d = OffsetDateTime::from_unix_timestamp(seconds)?
                    + time::Duration::nanoseconds(nanoseconds);
                Ok(d)
            }

            pub fn to_timestamp(dt: OffsetDateTime) -> Timestamp {
                Timestamp {
                    seconds: dt.unix_timestamp(),
                    nanos: dt.nanosecond() as i32,
                }
            }
        }
    }
}

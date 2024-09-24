#[cfg(feature = "users")]
pub mod users;

#[cfg(all(feature = "base", any(feature = "users")))]
pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

        #[cfg(feature = "time")]
        pub mod utils {
            use crate::google::protobuf::Timestamp;
            use time::OffsetDateTime;
            impl From<OffsetDateTime> for Timestamp {
                fn from(dt: OffsetDateTime) -> Self {
                    Timestamp {
                        seconds: dt.unix_timestamp(),
                        nanos: dt.nanosecond() as i32,
                    }
                }
            }

            impl TryFrom<Timestamp> for OffsetDateTime {
                type Error = time::Error;

                fn try_from(timestamp: Timestamp) -> Result<Self, Self::Error> {
                    let seconds = timestamp.seconds;
                    let nanos = timestamp.nanos as i64;
                    let nanoseconds = nanos % 1_000_000_000;
                    let d = OffsetDateTime::from_unix_timestamp(seconds)?
                        + time::Duration::nanoseconds(nanoseconds);
                    Ok(d)
                }
            }
        }
    }
}

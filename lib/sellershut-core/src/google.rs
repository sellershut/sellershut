pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

    #[cfg(feature = "time")]
    impl From<time::OffsetDateTime> for Timestamp {
        fn from(dt: time::OffsetDateTime) -> Self {
            Timestamp {
                seconds: dt.unix_timestamp(),
                nanos: dt.nanosecond() as i32,
            }
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<Timestamp> for time::OffsetDateTime {
        type Error = time::Error;

        fn try_from(timestamp: Timestamp) -> Result<Self, Self::Error> {
            let seconds = timestamp.seconds;
            let nanos = timestamp.nanos as i64;
            let nanoseconds = nanos % 1_000_000_000;
            let d = time::OffsetDateTime::from_unix_timestamp(seconds)?
                + time::Duration::nanoseconds(nanoseconds);
            Ok(d)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use time::{Duration, OffsetDateTime};

        #[test]
        fn test_offsetdatetime_to_timestamp() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();

            assert_eq!(timestamp.seconds, now.unix_timestamp());
            assert_eq!(timestamp.nanos, now.nanosecond() as i32);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();
            let dt: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt, now);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = 123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_negative_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = -123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_invalid_seconds() {
            let timestamp = Timestamp {
                seconds: i64::MIN,
                nanos: 0,
            };
            let result: Result<OffsetDateTime, time::Error> = timestamp.try_into();
            assert!(result.is_err());
        }
    }
}

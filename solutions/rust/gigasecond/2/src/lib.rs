use time::{PrimitiveDateTime as DateTime, UtcDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let unix_time = start.as_utc().unix_timestamp();
    let after_unix_time = unix_time + 1000000000;
    let utc_after_time = UtcDateTime::from_unix_timestamp(after_unix_time).unwrap();
    DateTime::new(
        utc_after_time.date(),
        utc_after_time.time()
    )
}

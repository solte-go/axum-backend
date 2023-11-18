mod dir_entry_froms;

mod error;

pub use self::error::{Error, Result};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime{
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> Result<String>{
    time.format(&Rfc3339).map_err(|_| Error::FailToConvertTime)
}

pub fn now_utc_plus_sec_str(sec: f64) -> Result<String>{
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time).map_err(|_| Error::FailToConvertTime)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339)
    .map_err(|_| Error::FailToParseDate(moment.to_string()))
} 

pub fn b64u_encode(content: &str) -> String {
    base64_url::encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<String> {
    let decoded_string = base64_url::decode(b64u)
    .ok()
    .and_then(|r| String::from_utf8(r).ok())
    .ok_or(Error::FailToB64uDecode)?;

    Ok(decoded_string)
}
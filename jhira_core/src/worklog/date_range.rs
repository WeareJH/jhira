use std::str::FromStr;
use failure::Error;
use chrono::{Date, Utc};

#[derive(Debug)]
pub struct DateRange {
    pub dates: Vec<Date<Utc>>,
}

#[derive(Fail, Debug)]
pub enum DateRangeError {
    #[fail(display = "nah nah nah {}", _0)]
    Nope(String)
}

impl FromStr for DateRange {
    type Err = DateRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateRange { dates: vec![] })
    }
}

#[test]
fn test_from_str() {
    let res = "1d".parse::<DateRange>();
    println!("{:#?}", res);
}

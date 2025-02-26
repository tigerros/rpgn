use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroU8;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Date {
    pub year: Option<u16>,
    pub month: Option<NonZeroU8>,
    pub day: Option<NonZeroU8>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DateValueError {
    YearGreaterThan9999,
    MonthGreaterThan12,
    DayGreaterThan31,
}

impl Date {
    /// # Errors
    ///
    /// - `year` is greater than 9999.
    /// - `month` is greater than 12.
    /// - `day` is greater than 31.
    pub fn new(year: Option<u16>, month: Option<NonZeroU8>, day: Option<NonZeroU8>) -> Result<Self, DateValueError> {
        if year.is_some_and(|y| y > 9999) {
            return Err(DateValueError::YearGreaterThan9999);
        }

        if let Some(month) = month {
            if month.get() > 12 {
                return Err(DateValueError::MonthGreaterThan12);
            }
        }

        if let Some(day) = day {
            if day.get() > 31 {
                return Err(DateValueError::DayGreaterThan31);
            }
        }

        Ok(Self { year, month, day })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(year) = self.year {
            f.write_str(&format!("{year:0>4}"))?;
        } else {
            f.write_str("????")?;
        }

        f.write_char('.')?;

        if let Some(month) = self.month {
            f.write_str(&format!("{month:0>2}"))?;
        } else {
            f.write_str("??")?;
        }

        f.write_char('.')?;

        if let Some(day) = self.day {
            f.write_str(&format!("{day:0>2}"))
        } else {
            f.write_str("??")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateParseError {
    MissingYear,
    MissingMonth,
    MissingDay,
    ValueError(DateValueError)
}

impl FromStr for Date {
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');

        let Some(year) = split.next() else {
            return Err(Self::Err::MissingYear);
        };

        let year = year.parse::<u16>().ok();

        let Some(month) = split.next() else {
            return Err(Self::Err::MissingMonth);
        };

        let month = month.parse::<NonZeroU8>().ok();

        let Some(day) = split.next() else {
            return Err(Self::Err::MissingDay);
        };

        let day = day.parse::<NonZeroU8>().ok();

        Self::new(year, month, day).map_err(Self::Err::ValueError)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use test_case::test_case;
    use proptest::proptest;

    #[test_case(Date { year: Some(2024), month: Some(unsafe { NonZeroU8::new_unchecked(2) }), day: Some(unsafe { NonZeroU8::new_unchecked(1) }) }, "2024.02.01")]
    #[test_case(Date { year: Some(15), month: Some(unsafe { NonZeroU8::new_unchecked(12) }), day: None }, "0015.12.??")]
    #[test_case(Date { year: Some(100), month: None, day: Some(unsafe { NonZeroU8::new_unchecked(11) }) }, "0100.??.11")]
    #[test_case(Date { year: None, month: Some(unsafe { NonZeroU8::new_unchecked(2) }), day: Some(unsafe { NonZeroU8::new_unchecked(1) }) }, "????.02.01")]
    #[test_case(Date { year: None, month: None, day: Some(unsafe { NonZeroU8::new_unchecked(1) }) }, "????.??.01")]
    #[test_case(Date { year: None, month: None, day: None }, "????.??.??")]
    #[test_case(Date { year: Some(1), month: None, day: None }, "0001.??.??")]
    #[test_case(Date { year: None, month: Some(unsafe { NonZeroU8::new_unchecked(5) }), day: None }, "????.05.??")]
    fn to_string_from_string(date: Date, date_str: &str) {
        assert_eq!(date.to_string(), date_str);
        assert_eq!(Date::from_str(date_str).unwrap(), date);
    }

    proptest! {
        #[test]
        fn invalid_year(year in 10000..u16::MAX) {
            assert_eq!(Date::new(Some(year), None, None), Err(DateValueError::YearGreaterThan9999));
        }

        #[test]
        fn invalid_month(month in 13..u8::MAX) {
            // SAFETY: The range 13..u8::MAX does not contain 0.
            // Also, it is a test who cares.
            let month = unsafe { NonZeroU8::new_unchecked(month) };
            assert_eq!(Date::new(None, Some(month), None), Err(DateValueError::MonthGreaterThan12));
        }
        
        #[test]
        fn invalid_day(day in 32..u8::MAX) {
            // SAFETY: The range 32..u8::MAX does not contain 0.
            // Also, it is a test who cares.
            let day = unsafe { NonZeroU8::new_unchecked(day) };
            assert_eq!(Date::new(None, None, Some(day)), Err(DateValueError::DayGreaterThan31));
        }
    }
}
use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroU8;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Date {
    year: Option<u16>,
    month: Option<NonZeroU8>,
    day: Option<NonZeroU8>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueError {
    YearGreaterThan9999,
    MonthGreaterThan12,
    DayGreaterThan31,
}

impl Date {
    /// # Errors
    /// See [`ValueError`].
    pub fn new(year: Option<u16>, month: Option<NonZeroU8>, day: Option<NonZeroU8>) -> Result<Self, ValueError> {
        if year.is_some_and(|y| y > 9999) {
            return Err(ValueError::YearGreaterThan9999);
        }

        if let Some(month) = month {
            if month.get() > 12 {
                return Err(ValueError::MonthGreaterThan12);
            }
        }

        if let Some(day) = day {
            if day.get() > 31 {
                return Err(ValueError::DayGreaterThan31);
            }
        }

        Ok(Self { year, month, day })
    }

    pub const fn year(&self) -> Option<u16> {
        self.year
    }

    pub const fn month(&self) -> Option<NonZeroU8> {
        self.month
    }

    pub const fn day(&self) -> Option<NonZeroU8> {
        self.day
    }

    /// # Errors
    /// Only variant that occurs is [`ValueError::YearGreaterThan9999`].
    pub fn set_year(&mut self, year: u16) -> Result<(), ValueError> {
        if year > 9999 {
            Err(ValueError::YearGreaterThan9999)
        } else {
            self.year = Some(year);
            Ok(())
        }
    }

    /// # Errors
    /// Only variant that occurs is [`ValueError::MonthGreaterThan12`].
    pub fn set_month(&mut self, month: NonZeroU8) -> Result<(), ValueError> {
        if month.get() > 12 {
            Err(ValueError::MonthGreaterThan12)
        } else {
            self.month = Some(month);
            Ok(())
        }
    }

    /// # Errors
    /// Only variant that occurs is [`ValueError::DayGreaterThan31`].
    pub fn set_day(&mut self, day: NonZeroU8) -> Result<(), ValueError> {
        if day.get() > 31 {
            Err(ValueError::DayGreaterThan31)
        } else {
            self.day = Some(day);
            Ok(())
        }
    }
}

#[cfg(feature = "time")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntoTimeDateError {
    NoYear,
    NoMonth,
    NoDay,
    TimeError(time::error::ComponentRange)
}

#[cfg(feature = "time")]
impl TryFrom<Date> for time::Date {
    type Error = IntoTimeDateError;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
        let Some(year) = date.year else { return Err(IntoTimeDateError::NoYear); };
        let Some(month) = date.month else { return Err(IntoTimeDateError::NoMonth); };
        let Some(day) = date.day else { return Err(IntoTimeDateError::NoDay); };

        // CLIPPY: We make sure the month is not zero and not greater than 12.
        #[allow(clippy::unwrap_used)]
        Self::from_calendar_date(i32::from(year), time::Month::try_from(month.get()).unwrap(), day.get()).map_err(IntoTimeDateError::TimeError)
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
pub enum ParseError {
    NoYear,
    NoMonth,
    NoDay,
    ValueError(ValueError)
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');

        let Some(year) = split.next() else {
            return Err(Self::Err::NoYear);
        };

        let year = year.parse::<u16>().ok();

        let Some(month) = split.next() else {
            return Err(Self::Err::NoMonth);
        };

        let month = month.parse::<NonZeroU8>().ok();

        let Some(day) = split.next() else {
            return Err(Self::Err::NoDay);
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

    #[test_case(Ok(::time::Date::from_calendar_date(2000, ::time::Month::October, 15).unwrap()), Date { year: Some(2000), month: Some(unsafe { NonZeroU8::new_unchecked(10) }), day: Some(unsafe { NonZeroU8::new_unchecked(15) }) })]
    fn time(correct: Result<::time::Date, IntoTimeDateError>, test: Date) {
        assert_eq!(correct, ::time::Date::try_from(test));
    }

    proptest! {
        #[test]
        fn invalid_year(year in 10000..u16::MAX) {
            assert_eq!(Date::new(Some(year), None, None), Err(ValueError::YearGreaterThan9999));
        }

        #[test]
        fn invalid_month(month in 13..u8::MAX) {
            // SAFETY: The range 13..u8::MAX does not contain 0.
            // Also, it is a test who cares.
            let month = unsafe { NonZeroU8::new_unchecked(month) };
            assert_eq!(Date::new(None, Some(month), None), Err(ValueError::MonthGreaterThan12));
        }
        
        #[test]
        fn invalid_day(day in 32..u8::MAX) {
            // SAFETY: The range 32..u8::MAX does not contain 0.
            // Also, it is a test who cares.
            let day = unsafe { NonZeroU8::new_unchecked(day) };
            assert_eq!(Date::new(None, None, Some(day)), Err(ValueError::DayGreaterThan31));
        }
    }
}
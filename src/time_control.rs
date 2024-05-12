use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::TimeControlField;

// TODO

#[derive(Debug)]
pub struct TimeControl {
    pub descriptors: Vec<TimeControlField>
}

impl Display for TimeControl {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for TimeControl {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    // #[test_case(TimeControl { descriptors: vec![TimeControlField::MovesPerSeconds { moves, }] })]
    // fn to_string_from_string(time_control: TimeControl, time_control_str: &str) {
    //     assert_eq!(time_control.to_string(), time_control_str);
    //     assert_eq!(TimeControl::from_str(time_control_str).unwrap(), time_control);
    // }
}
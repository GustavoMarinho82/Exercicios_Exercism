use std::fmt;

//i32 to avoid some data type conversions
pub const MINUTES_IN_HOUR: i32 = 60;
pub const MINUTES_IN_DAY: i32 = 1440;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: u8,
    minute: u8
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time_in_minutes: i32 = (hours*MINUTES_IN_HOUR) + minutes;
        let time_reduced: i32 = time_in_minutes.rem_euclid(MINUTES_IN_DAY); //x.rem_euclid(y) = ((x % y) + y) % y
        
        return Clock {
            hour: (time_reduced / MINUTES_IN_HOUR) as u8,
            minute: (time_reduced % MINUTES_IN_HOUR) as u8
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time_in_minutes: i32 = ((self.hour as i32)*MINUTES_IN_HOUR) + (self.minute as i32) + minutes;

        return Clock::new(0, time_in_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
    }
}
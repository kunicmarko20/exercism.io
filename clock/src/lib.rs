use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_A_HOUR: i32 = 60;

impl Clock {
    pub fn new(mut hour: i32 , mut minute: i32) -> Clock {
        while minute < 0 { 
            hour -= 1;
            minute += MINUTES_IN_A_HOUR 
        }

        hour += minute / MINUTES_IN_A_HOUR;

        while hour < 0 {
            hour += HOURS_IN_A_DAY
        }

        Clock { hours: hour % HOURS_IN_A_DAY, minutes: minute % MINUTES_IN_A_HOUR }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_A_HOUR: i32 = 60;

impl Clock {
    pub fn new(hour: i32 , minute: i32) -> Clock {
        Clock {
            hours: Clock::calculate_hours(hour, minute),
            minutes: Clock::calculate_minutes(minute)
        }
    }

    fn calculate_minutes(minutes: i32) -> i32 {
        let minutes_remainder = minutes % MINUTES_IN_A_HOUR;

        if minutes_remainder == 0 {
            return 0;
        }

        if minutes_remainder < 0 {
            return MINUTES_IN_A_HOUR + minutes_remainder;
        }
        
        minutes_remainder
    }

    fn calculate_hours(mut hours: i32, minutes: i32) -> i32 {
        if minutes % MINUTES_IN_A_HOUR < 0 {
            hours -= 1;
        }

        hours += minutes / MINUTES_IN_A_HOUR;

        let hours_remainder = hours % HOURS_IN_A_DAY;

        if hours_remainder < 0 {
            return HOURS_IN_A_DAY + hours_remainder;
        }

        hours_remainder
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

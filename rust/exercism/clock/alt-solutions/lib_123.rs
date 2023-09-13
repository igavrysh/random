use std::fmt::{self, format};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {

    const MINS_PER_DAY: i32 = 24 * 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Clock::align(hours, minutes);
        Clock { hours: h, minutes: m }
    }

    fn align(hours: i32, minutes: i32) -> (i32, i32) {
        let mut m_total = hours * 60 + minutes;
        if m_total < 0 {
            m_total = m_total + (-1 * (m_total / Clock::MINS_PER_DAY) + 1) * Clock::MINS_PER_DAY;
        }

        let h_rem = m_total / 60;
        let m = m_total % 60;
        let h = (h_rem) % 24;
        (h, m)
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl From<Clock> for String {
    fn from(c: Clock) -> String {
        format!("{:02}:{:02}", c.hours, c.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

/*
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
*/

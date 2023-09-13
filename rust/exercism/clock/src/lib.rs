use std::fmt::{self, format};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MINS_PER_HOUR: i32 = 60;
    const HRS_PER_DAY: i32 = 24;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Clock::align(hours, minutes);
        Clock { hours: h, minutes: m }
    }

    fn align(hours: i32, minutes: i32) -> (i32, i32) {
        let m_total = hours * Clock::MINS_PER_HOUR + minutes;
        let hrs = m_total.div_euclid(Clock::MINS_PER_HOUR);
        let m = m_total.rem_euclid(Clock::MINS_PER_HOUR);
        let h = hrs.rem_euclid(Clock::HRS_PER_DAY);
        (h, m)
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let (h, m) = Clock::align(self.hours, self.minutes + minutes);
        self.minutes = m;
        self.hours = h;

        self
    }

    // Not working!
    /*  
    pub fn add_minutes(&mut self, minutes: i32) -> &Self {
        (self.hours, self.minutes) = Clock::align(self.hours, self.minutes + minutes);
        self
    }
    */

    // Why not the code below?
    /* 
    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let (h, m) = Clock::align(self.hours, self.minutes + minutes);
        self.minutes = m;
        self.hours = h;
        self
    */

    // Code below is also not working 
    /*
    pub fn add_minutes(&mut self, minutes: i32) -> &Self {
        let (h, m) = Clock::align(self.hours, self.minutes + minutes);
        self.minutes = m;
        self.hours = h;

        self
    }
    */

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

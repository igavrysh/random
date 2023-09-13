#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}
const MINUTES_IN_A_DAY: i32 = 24 * 60;
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Self::wrap(hours, minutes);
        Clock{hours: h, minutes: m}
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        (self.hours, self.minutes) = Self::wrap(self.hours as i32, self.minutes as i32 + minutes);
        self
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    fn wrap(hours: i32, minutes: i32) -> (u8, u8) {
        let oneday = ((hours * 60 + minutes) % MINUTES_IN_A_DAY + MINUTES_IN_A_DAY) % MINUTES_IN_A_DAY;
        ((oneday / 60) as u8, (oneday % 60) as u8)
    }
}

use std::fmt::Formatter;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minute_from_hours = hours * 60;
        let time_minute = minutes + minute_from_hours;
        let time_minute_parsed = Self::rec_time_num(time_minute);
        let mut hours = time_minute_parsed / 60;
        if hours == 24 { hours = 0 };
        let minutes = time_minute_parsed % 60;
        println!("time_minute:{time_minute};hours:{hours};minutes:{minutes};");
        if time_minute < 0 {
            let remain_time = Self::rec_time_num(1440 - hours * 60 - minutes);
            Clock {
                hours: { remain_time / 60 },
                minutes: { remain_time % 60 },
            }
        } else {
            Clock { hours, minutes }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }

    fn rec_time_num(time_num: i32) -> i32 {
        let mut ret: i32 = time_num.abs();
        if ret > 1440 {
            ret -= 1440;
            Self::rec_time_num(ret)
        } else {
            ret
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
use std::str::FromStr;

pub struct Timestamp {
    pub time_milliseconds: i64,
}

impl Timestamp {
    pub fn from_milliseconds(ms: i64) -> Timestamp {
        Timestamp {
            time_milliseconds: ms,
        }
    }

    pub fn to_string(&self) -> String {
        let mut total = self.time_milliseconds;
        let milliseconds = total % 1000;
        total = total - milliseconds;
        if total == 0 {
            return format!("00:00:00,{:03}", milliseconds);
        }
        let seconds = (total % (60 * 1000)) / 1000;
        total = total - seconds * 1000;
        if total == 0 {
            return format!("00:00:{:02},{:03}", seconds, milliseconds);
        }
        let minuts = (total % (60 * 60 * 1000)) / (60 * 1000);
        total = total - minuts * (60 * 1000);
        if total == 0 {
            return format!("00:{:02}:{:02},{:03}", minuts, seconds, milliseconds);
        }
        let hours = (total % (60 * 60 * 60 * 1000)) / (60 * 60 * 1000);
        format!(
            "{:02}:{:02}:{:02},{:03}",
            hours, minuts, seconds, milliseconds
        )
    }

    pub fn shift(&mut self, ms: i64) {
        self.time_milliseconds = self.time_milliseconds + ms;
    }
}

impl FromStr for Timestamp {
    // parse timestamp from HH:mm:ss,SSS format
    fn from_str(s: &str) -> Result<Timestamp, ()> {
        let parts: Vec<&str> = s.split([':', ',']).collect();
        let hours = parts[0].parse::<i64>().unwrap();
        let minuts = parts[1].parse::<i64>().unwrap() + hours * 60;
        let seconds = parts[2].parse::<i64>().unwrap() + minuts * 60;
        let milliseconds = parts[3].parse::<i64>().unwrap() + seconds * 1000;
        Result::Ok(Timestamp {
            time_milliseconds: milliseconds,
        })
    }

    type Err = ();
}

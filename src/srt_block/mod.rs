use crate::timestamp::Timestamp;

/// Present block of SRT subtitle file.
pub struct SRTBlock {
    pub index: u32,
    pub begin_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub text: String,
}

impl SRTBlock {
    pub fn from_str(block: &str) -> SRTBlock {
        let block_parts: Vec<&str> = block.split("\r\n").collect();
        let index: u32 = block_parts[0].parse().unwrap();
        let timestamps_raw: Vec<&str> = block_parts[1].split(" --> ").collect();
        let timestamp_begin: Timestamp = timestamps_raw[0].parse().unwrap();
        let timestamp_end: Timestamp = timestamps_raw[1].parse().unwrap();
        let text = block_parts[2..].join("\r\n");
        SRTBlock {
            index,
            begin_timestamp: timestamp_begin,
            end_timestamp: timestamp_end,
            text,
        }
    }

    pub fn to_str(&self) -> String {
        //println!("{}", self.text);
        return format!(
            "{}\r\n{} -->  {}\r\n{}\r\n",
            self.index,
            self.begin_timestamp.to_string(),
            self.end_timestamp.to_string(),
            self.text.trim()
        );
    }

    /// Shift begin timestamp of srt on `ms`.
    pub fn shift_begin_timestamp(&mut self, ms: i64) {
        self.begin_timestamp.shift(ms);
    }

    /// Shift end timestamp of srt on `ms`.
    pub fn shift_end_timestamp(&mut self, ms: i64) {
        self.end_timestamp.shift(ms);
    }

    /// Shift both timestamp of srt on `ms`.
    pub fn shift_timestamps(&mut self, ms: i64) {
        self.begin_timestamp.shift(ms);
        self.end_timestamp.shift(ms);
    }
}

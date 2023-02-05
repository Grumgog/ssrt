#[cfg(test)]
mod srt_block_test {
    use crate::srt_block::SRTBlock;

    #[test]
    fn srt_block_test() {
        let test1 = SRTBlock::from_str(
            "1\r\n00:00:00,000 --> 01:00:00,000\r\nНовый фильм\r\nпредставляет\r\n\r\n",
        );
        assert_eq!(test1.begin_timestamp.time_milliseconds, 0);
        assert_eq!(test1.end_timestamp.time_milliseconds, 1000 * 60 * 60)
    }
}

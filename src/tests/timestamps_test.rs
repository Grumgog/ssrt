#[cfg(test)]
mod timestamps_test {
    use crate::timestamp::Timestamp;

    #[test]
    fn from_str_correct_parse() {
        let test1 = "00:00:05,300".parse::<Timestamp>().unwrap();
        let test2 = "00:45:41,954".parse::<Timestamp>().unwrap();
        let test3 = "00:38:20,132".parse::<Timestamp>().unwrap();
        let test4 = "01:38:20,132".parse::<Timestamp>().unwrap();
        assert_eq!(5300, test1.time_milliseconds);
        assert_eq!(2741954, test2.time_milliseconds);
        assert_eq!(2300132, test3.time_milliseconds);
        assert_eq!(5900132, test4.time_milliseconds);
    }

    #[test]
    fn to_str_correct_convert() {
        let test1 = Timestamp::from_milliseconds(1500);
        assert_eq!(test1.to_string(), "00:00:01,500");
        let test2 = Timestamp::from_milliseconds(75000);
        assert_eq!(test2.to_string(), "00:01:15,000");
        let test3 = Timestamp::from_milliseconds(5900132);
        assert_eq!(test3.to_string(), "01:38:20,132");
    }
}

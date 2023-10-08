use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, TimeZone,
    Utc,
};

use crate::error::Result;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum DateFormat {
    YYYYMMDDHHMMSS,
    YYYYMMDD,
    CN_DATE_FORMAT,
    YYYYMMDD_,
    YYYYMM,
    HHMMSS,
}

pub fn naive_date_time_to_string(date_time: NaiveDateTime) -> String {
    naive_date_time_to_string_by_format(date_time, DateFormat::YYYYMMDDHHMMSS)
}

pub fn naive_date_time_to_string_by_format(date_time: NaiveDateTime, format: DateFormat) -> String {
    date_time.format(format_to_str(format)).to_string()
}

/// 获取今日开始时间:时间戳格式(毫秒)
#[allow(dead_code)]
pub fn get_today_start_end_time_millis() -> (i64, i64) {
    let dt = Local::now();
    let (today_start, today_end) = get_today_start_end_time(dt);
    (today_start.timestamp(), today_end.timestamp())
}

/// 获取今日开始时间:字符格式
#[allow(dead_code)]
pub fn get_today_start_end_time_str() -> (String, String) {
    let dt = Local::now();
    let (today_start, today_end) = get_today_start_end_time(dt);
    let today_start_str = today_start
        .format(format_to_str(DateFormat::YYYYMMDDHHMMSS))
        .to_string();
    let today_end_str = today_end
        .format(format_to_str(DateFormat::YYYYMMDDHHMMSS))
        .to_string();
    (today_start_str, today_end_str)
}

fn get_today_start_end_time<Tz: TimeZone>(dt: DateTime<Tz>) -> (NaiveDateTime, NaiveDateTime) {
    let today_date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
    let today_start_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let today_end_time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let today_start = NaiveDateTime::new(today_date, today_start_time);
    let today_end = NaiveDateTime::new(today_date, today_end_time);
    (today_start, today_end)
}

/// 获取当前的时间,返回String类型
#[allow(dead_code)]
pub fn get_now_time_str() -> String {
    let dt = Utc::now();
    date_to_str(dt, DateFormat::YYYYMMDDHHMMSS)
}

/// 获取当前的时间,返回String类型
#[allow(dead_code)]
pub fn get_now_time_str_yyyy_mm_dd() -> String {
    let dt = Utc::now();
    date_to_str(dt, DateFormat::YYYYMMDD)
}

#[allow(dead_code)]
pub fn get_now_time_str_yyyymmdd() -> String {
    let dt = Utc::now();
    date_to_str(dt, DateFormat::YYYYMMDD_)
}

/// 获取当前时间,返回i64类型
#[allow(dead_code)]
pub fn get_now_time() -> i64 {
    let dt = Utc::now();
    dt.timestamp()
}

/// 获取当前时间,返回i64类型
#[allow(dead_code)]
pub fn get_now_time_millis() -> i64 {
    let dt = Utc::now();
    dt.timestamp_millis()
}

pub fn get_current_time() -> Option<DateTime<Utc>> {
    // let eight_hours = FixedOffset::east_opt(8 * 60 * 60).unwrap(); // 东八区时差为 +8 小时
    // println!("{}", eight_hours);
    // let local_time = eight_hours.from_utc_datetime(&Utc::now().naive_utc()); // 将当前的 UTC 时间转换为东八区时间
    // println!("{}", local_time);
    // Some(local_time.with_timezone(&Utc))
    Some(Utc::now())
}

///根据时间戳生成DateTime
pub fn from_timestamp_millis(timestamp: u64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_millis(timestamp as i64).unwrap(),
        Utc,
    )
}

///根据时间戳生成DateTime
pub fn from_timestamp_millis_opt(timestamp: Option<u64>) -> Option<DateTime<Utc>> {
    if timestamp.is_none() {
        return None;
    }
    Some(DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_millis(timestamp.unwrap() as i64).unwrap(),
        Utc,
    ))
}

///根据时间戳生成DateTime
pub fn mills_convert_datetime(timestamp: Option<u64>) -> Option<DateTime<Utc>> {
    timestamp.and_then(|ts| Utc.timestamp_millis_opt(ts as i64).single())
}

pub fn get_current_time_str() -> String {
    let dt = get_current_time().unwrap();
    date_to_str(dt, DateFormat::YYYYMMDDHHMMSS)
}

pub fn get_current_date() -> String {
    let dt = get_current_time().unwrap();
    date_to_str(dt, DateFormat::YYYYMMDD)
}

pub fn get_current_month() -> String {
    let dt = get_current_time().unwrap();
    date_to_str(dt, DateFormat::YYYYMM)
}

pub fn get_current_month_begin() -> String {
    let dt = get_current_time().unwrap();
    let str = date_to_str(dt, DateFormat::YYYYMM);
    format!("{}-01 00:00:00", str)
}

pub fn get_current_month_end() -> String {
    let dt = get_current_time().unwrap();
    let str = date_to_str(dt, DateFormat::YYYYMM);
    format!("{}-31 23:59:59", str)
}

/// 时间类型转字符类型,返回String类型
/// example:
///     se chrono::Utc;
///
///     let dt = Utc::now();
///     let dt_str = date_to_str(dt, DateFormat::YYYYMMDD);
///     println!("{}", dt_str);
#[allow(dead_code)]
pub fn date_to_str(dt: DateTime<Utc>, format: DateFormat) -> String {
    // dt.format(format_to_str(format)).to_string()
    date_to_str_beijing(dt, format)
}

pub fn date_to_str_beijing(dt: DateTime<Utc>, format: DateFormat) -> String {
    dt.with_timezone(&Local)
        .format(format_to_str(format))
        .to_string()
}

pub fn str_to_date(s: &str, format: DateFormat) -> Result<NaiveDateTime, ParseError> {
    let dt = NaiveDateTime::parse_from_str(s, format_to_str(format))?;
    Ok(dt)
}

pub fn get_date_start(dt: DateTime<Utc>) -> DateTime<Utc> {
    let date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
    let start_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date_start = NaiveDateTime::new(date, start_time);
    DateTime::<Utc>::from_utc(date_start, Utc)
}

pub fn get_date_start_from_str(s: &str) -> DateTime<Utc> {
    let dt = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    get_date_start(DateTime::from_utc(dt, Utc))
}

pub fn get_date_start_from_str_return_str(s: &str) -> String {
    let dt = get_date_start_from_str(s);
    dt.format(format_to_str(DateFormat::YYYYMMDDHHMMSS)).to_string()
}

pub fn get_date_end_from_str_return_str(s: &str) -> String {
    let dt = get_date_end_from_str(s);
    dt.format(format_to_str(DateFormat::YYYYMMDDHHMMSS)).to_string()
}

pub fn get_date_end(dt: DateTime<Utc>) -> DateTime<Utc> {
    let date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
    let end_time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let date_end = NaiveDateTime::new(date, end_time);
    DateTime::<Utc>::from_utc(date_end, Utc)
}

pub fn get_date_end_from_str(s: &str) -> DateTime<Utc> {
    let dt = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    get_date_end(DateTime::from_utc(dt, Utc))
}

fn format_to_str(format: DateFormat) -> &'static str {
    match format {
        DateFormat::YYYYMMDD => "%Y-%m-%d",
        DateFormat::CN_DATE_FORMAT => "%Y年%m月%d日",
        DateFormat::YYYYMMDD_ => "%Y%m%d",
        DateFormat::HHMMSS => "%H:%M:%S",
        DateFormat::YYYYMM => "%Y-%m",
        _ => "%Y-%m-%d %H:%M:%S",
    }
}

pub fn plus_days(days: i64) -> DateTime<Utc> {
    let today = get_current_time().unwrap();
    today + Duration::days(days)
}

pub fn plus_months(months: i64) -> DateTime<Utc> {
    let mut date = get_current_time().unwrap();
    let curr_day = date.day();
    date = date.with_day(1).unwrap();
    let mut month = date.month() as i32 + months as i32;
    while month > 12 {
        date = date.with_year(date.year() + 1).unwrap();
        month -= 12;
    }
    set_days(date, curr_day, month)
}

pub fn sub_months(months: i64) -> DateTime<Utc> {
    let mut date = get_current_time().unwrap();
    let curr_day = date.day();
    date = date.with_day(1).unwrap();
    let mut month = date.month() as i32 - months as i32;
    if month <= 0 {
        date = date.with_year(date.year() - 1).unwrap();
        month += 12;
    }
    set_days(date, curr_day, month)
}

fn set_days(mut date: DateTime<Utc>, curr_day: u32, month: i32) -> DateTime<Utc> {
    date = date.with_month(month as u32).unwrap();
    let days = month_days(&date_to_str(date, DateFormat::YYYYMMDD));
    if days < curr_day {
        date = date.with_day(days).unwrap();
    } else {
        date = date.with_day(curr_day).unwrap();
    }
    date
}

pub fn plus_hours(hours: i64) -> DateTime<Utc> {
    let today = get_current_time().unwrap();
    today + Duration::hours(hours)
}

pub fn plus_years(years: i64) -> DateTime<Utc> {
    let today = get_current_time().unwrap();
    today + Duration::days(years * 365)
}

pub fn month_days(time_str: &str) -> u32 {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let end_of_month = NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    end_of_month.day()
}

pub fn get_start_time_of_month(time_str: &str) -> String {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let start_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    start_of_month.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_start_time_of_month_date_time(time_str: &str) -> DateTime<Utc> {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let start_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    DateTime::from_utc(start_of_month, Utc)
}

pub fn get_end_time_of_month_date_time(time_str: &str) -> DateTime<Utc> {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let end_of_month = NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    DateTime::from_utc(end_of_month, Utc)
}

pub fn get_end_time_of_month(time_str: &str) -> String {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let end_of_month = NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    end_of_month.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_date(s: &str) -> DateTime<Utc> {
    let dt = NaiveDate::parse_from_str(s, format_to_str(DateFormat::YYYYMMDD))
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    DateTime::from_utc(dt, Utc)
}

pub fn format_date_time(s: &str) -> DateTime<Utc> {
    let dt = NaiveDateTime::parse_from_str(s, format_to_str(DateFormat::YYYYMMDDHHMMSS)).unwrap();
    DateTime::from_utc(dt, Utc)
}

#[cfg(test)]
mod date_utils_tests {
    use std::cmp::Ordering;

    use chrono::Utc;

    use super::*;

    #[test]
    fn test_plus_years() {
        for i in 0..100 {
            let date = plus_years(i);
            println!("i = {},date:{}", i, date);
        }
    }

    #[test]
    fn test_plus_days() {
        let date = plus_days(300);
        println!("date:{}", date);
    }

    // test plus_months
    #[test]
    fn test_plus_months() {
        for i in 0..100 {
            let date = plus_months(i);
            println!("i = {},date:{}", i, date);
        }
        // let date = plus_months(20);
        // println!("date:{}", date);
    }

    // test sub_months
    #[test]
    fn test_sub_months() {
        for i in 0..10 {
            let date = sub_months(i);
            println!("i = {},date:{}", i, date);
        }
    }

    // test format_date_time
    #[test]
    fn test_format_date_time() {
        let date = format_date_time(&get_date_end_from_str_return_str("2023-04-19"));
        println!("date:{}", date);

        let now = get_current_time().unwrap();
        println!("now:{}", now);
        println!("now > date ? {:?}", now.cmp(&date) >= Ordering::Equal);
        println!("now > date ? {:?}", now.cmp(&date) <= Ordering::Equal);
    }

    // test get_date_start_from_str_return_str
    #[test]
    fn test_get_date_start_from_str_return_str() {
        let date = get_date_start_from_str_return_str("2021-01-01");
        println!("date:{}", date);

        let date = get_date_end_from_str_return_str("2023-04-20");
        println!("date:{}", date);
    }

    // test format date
    #[test]
    fn test_format_date() {
        let date = format_date("2021-01-01");
        println!("date:{}", date);
    }

    // 测试 get_end_time_of_month
    #[test]
    fn test_get_end_time_of_month() {
        let date = get_end_time_of_month("2023-02-01");
        println!("date:{}", date);
    }

    // 测试 get_start_time_of_month
    #[test]
    fn test_get_start_time_of_month() {
        let date = get_start_time_of_month("2023-03-11");
        println!("date:{}", date);
    }

    // 测试 month_days
    #[tokio::test]
    async fn test_month_days() {
        let days = month_days("2023-02-11");
        println!("days:{}", days);
    }

    // 测试 get_current_time
    #[tokio::test]
    async fn test_get_current_time() {
        let time = get_current_time();
        println!("time:{}", time.unwrap());
        println!("time:{}", Utc::now());
    }

    // 测试 get_date_start_from_str
    #[tokio::test]
    async fn test_date_start_from_str() {
        let today_start = get_date_start_from_str("2020-01-01");
        println!("today_start:{}", today_start);
    }
    // 测试 get_date_end_from_str
    #[tokio::test]
    async fn test_date_end_from_str() {
        let today_end: DateTime<Utc> = get_date_end_from_str("2020-01-01");
        println!("today_end:{}", today_end);
    }

    // 测试 get_date_end
    #[tokio::test]
    async fn test_date_end() {
        let dt = Utc::now();
        let today_end = get_date_end(dt);
        println!("today_end:{}", today_end);
    }
    // 测试 get_date_start
    #[tokio::test]
    async fn test_date_start() {
        let dt = Utc::now();
        let today_start = get_date_start(dt);
        println!("today_start:{}", today_start);
    }

    // 测试 get_now_time_str
    #[tokio::test]
    async fn test_get_now_time_str() {
        let dt_str = get_now_time_str();
        println!("{}", dt_str);
    }

    // 测试 date_to_str_beijing
    #[tokio::test]
    async fn test_date_to_str_beijing() {
        let dt = Utc::now();
        let dt_str = date_to_str_beijing(dt, DateFormat::YYYYMMDDHHMMSS);
        println!("{}", dt_str);
    }

    #[tokio::test]
    async fn tests() -> Result<(), Box<dyn std::error::Error>> {
        let dt = Utc::now();
        println!("get_now_time = {}", get_now_time());
        println!("get_now_time_millis = {}", get_now_time_millis());
        println!(
            "date_to_str_YYYYMMDDHHMMSS = {}",
            date_to_str(dt, DateFormat::YYYYMMDDHHMMSS)
        );
        println!(
            "date_to_str_YYYYMMDD = {}",
            date_to_str(dt, DateFormat::YYYYMMDD)
        );
        println!(
            "date_to_str_HHMMSS = {}",
            date_to_str(dt, DateFormat::HHMMSS)
        );
        println!(
            "get_today_start_end_time_millis = {:?}",
            get_today_start_end_time_millis()
        );
        println!(
            "get_today_start_end_time_str = {:?}",
            get_today_start_end_time_str()
        );
        println!(
            "str_to_date_YYYYMMDDHHMMSS = {:?}",
            str_to_date("2023-03-27 00:00:00", DateFormat::YYYYMMDDHHMMSS)
        );
        println!(
            "from_timestamp = {:?}",
            date_to_str(
                from_timestamp_millis(1615887113456),
                DateFormat::YYYYMMDDHHMMSS
            )
        );
        println!(
            "convert_timestamp = {:?}",
            date_to_str(
                mills_convert_datetime(Some(1615887113456)).unwrap(),
                DateFormat::YYYYMMDDHHMMSS
            )
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_month() {
        println!("get_current_month = {}", get_current_month());
        println!("get_current_month_begin = {}", get_current_month_begin());
        println!("get_current_month_end = {}", get_current_month_end());
    }
}

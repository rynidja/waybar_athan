use chrono::{Duration, Local, NaiveDateTime};
use islam::salah::PrayerTimes;

pub fn next_prayer(prayers: PrayerTimes) -> (String, NaiveDateTime, Duration) {
    let prayer = prayers.next().unwrap();

    let (h, m) = prayers.time_remaining().unwrap();
    let rem_time = Duration::minutes((m + h * 60) as i64);

    let time = if prayers.ishaa < Local::now().naive_local() {
        prayers.fajr_tomorrow
    } else {
        prayers.time(prayer)
    };
    println!(
        "next: {:?} at {} -{}:{:02}",
        prayer,
        time,
        h,
        m
    );

    (prayer.name().unwrap(), time, rem_time)
}

pub fn current_prayer(prayers: PrayerTimes) -> (String, NaiveDateTime, Duration) {
    let prayer = prayers.current().unwrap();

    let (h, m) = prayers.time_passed().unwrap();
    let past_time = Duration::minutes((m + h * 60) as i64);

    let time = if Local::now().naive_local() < prayers.fajr {
        prayers.ishaa_yesterday
    } else {
        prayers.time(prayer)
    };

    println!(
        "curr: {:?} at {} +{}:{:02}",
        prayer,
        time,
        h,
        m
    );

    (prayer.name().unwrap(), time, past_time)
}
// use std::ops::Add;

// use chrono::{Days, Duration, Local};


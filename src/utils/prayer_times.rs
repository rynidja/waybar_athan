use chrono::{Duration, Local};
use islam::salah::{PrayerTimes, Prayer};


pub fn next_prayer(prayers: PrayerTimes) -> (Prayer, Duration) {
    let prayer = prayers.next().unwrap();
    let rem_time = prayers.time(prayer).signed_duration_since(Local::now().naive_local());

    (prayer, rem_time)
}

pub fn current_prayer(prayers: PrayerTimes) -> (Prayer, Duration) {
    let prayer = prayers.current().unwrap();
    let prayer_time = prayers.time(prayer);
    let past_time = Local::now().naive_local().signed_duration_since(prayer_time);

    (prayer, past_time)
}

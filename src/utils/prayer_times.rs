use salah::prelude::*;

pub fn next_prayer(prayers: PrayerTimes) -> (Prayer, Duration) {
    let prayer = prayers.next();
    let rem_time = prayers.time(prayer).signed_duration_since(Utc::now());

    (prayer, rem_time)
}

pub fn current_prayer(prayers: PrayerTimes) -> (Prayer, Duration) {
    let prayer = prayers.current();
    let prayer_time = prayers.time(prayer);
    let past_time = Utc::now().signed_duration_since(prayer_time);

    (prayer, past_time)
}

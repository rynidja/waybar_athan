use chrono::{Days, Duration, Local, NaiveDateTime};
use islam::salah::{Prayer, PrayerSchedule, PrayerTimes};

pub fn next_prayer(prayers: PrayerTimes) -> (String, NaiveDateTime, Duration) {
    let prayer = prayers.next();
    let prayer_time = prayers.time(prayer);

    let (h, m) = prayers.time_remaining();
    let rem_time = Duration::minutes((m + h * 60) as i64);

    // println!("next: {:?} at {} -{}:{:02}", prayer, prayer_time, h, m);

    (prayer.name().unwrap(), prayer_time, rem_time)
}

pub fn current_prayer(prayers: PrayerTimes) -> (String, NaiveDateTime, Duration) {
    let prayer = prayers.current();

    let prayer_time = if prayers.next() == Prayer::Fajr {
        PrayerSchedule::new(prayers.location)
            .unwrap()
            .on(prayers.time.date() - Days::new(1))
            .unwrap()
            .with_config(prayers.config)
            .calculate()
            .unwrap()
            .time(Prayer::Ishaa)
    } else {
        prayers.time(prayers.current())
    };

    let past_time = Local::now().naive_local() - prayer_time;

    // let (h, m) = (past_time.num_hours(), past_time.num_minutes() % 60);
    // println!("curr: {:?} at {} +{}:{:02}", prayer, prayer_time, h, m);

    (prayer.name().unwrap(), prayer_time, past_time)
}

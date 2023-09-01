use chrono::Duration;
use islam::salah::PrayerTimes;

use crate::utils::prayer_times::*;

enum TimeStatus {
    Remaining,
    Before, 
    Athan,
    Past,
}

impl From<TimeStatus> for String {
    fn from(class: TimeStatus) -> Self {
        match class {
            TimeStatus::Remaining => "remaining".into(),
            TimeStatus::Before => "before".into(),
            TimeStatus::Athan => "athan".into(),
            TimeStatus::Past => "past".into(),
        }
    }
}

fn format_time(duration: Duration) -> String {
    let hours = duration.num_hours() as u32;
    let minutes = (duration.num_minutes() % 60) as u32;

    format!("{}:{:02}", hours, minutes)
}

pub fn gen_output(prayers: PrayerTimes, i3blocks_style: bool) -> String {
    let text: String;
    let tooltip: String;
    let class: TimeStatus;

    let (curr_name, curr_time, past) = current_prayer(prayers);
    let (next_name, next_time, rem) = next_prayer(prayers);

    if rem < (past * 2) {
        if rem.num_minutes() == 0 {
            text = next_name;
            tooltip = format!("Time for {}", text);
            class = TimeStatus::Athan;
        } else {
            text = format!("{} -{}", next_name, format_time(rem));
            tooltip = format!("{} at {}", next_name, next_time.format("%H:%M"));
            if rem.num_minutes() <= 10 {
                class = TimeStatus::Before;
            } else {
                class = TimeStatus::Remaining;
            }
        }
    } else if past.num_minutes() == 0 {
        text = curr_name;
        tooltip = format!("Time for {}", text);
        class = TimeStatus::Athan;
    } else {
        text = format!("{} +{}", curr_name, format_time(past));
        tooltip = format!("{} at {}", curr_name, curr_time.format("%H:%M"));
        class = TimeStatus::Past;
    }

    if i3blocks_style {
        format!("{}\n{}\n{}", text, tooltip, String::from(class))
    } else {
        format!(
            "{{\"text\": \"{}\", \"alt\": \"\", \"tooltip\": \"{}\", \"class\": \"{}\"}}",
            text,
            tooltip,
            String::from(class)
        )
    }
}

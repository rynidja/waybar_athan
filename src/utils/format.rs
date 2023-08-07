use crate::utils::prayer_times::*;
use salah::prelude::*;

enum TimeStatus {
    Remaining,
    Before, // wudu ...
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

    format!(
        "{}:{}{}",
        hours,
        if minutes <= 9 { "0" } else { "" },
        minutes
    )
}

pub fn gen_output(prayers: PrayerTimes, i3blocks_style: bool) -> String {
    let text: String;
    let tooltip: String;
    let class: TimeStatus;

    let (curr, past) = current_prayer(prayers);
    let (next, rem) = next_prayer(prayers);

    if rem < (past * 2) {
        if rem.num_minutes() == 0 {
            text = next.name();
            tooltip = format!("Time for {}", text);
            class = TimeStatus::Athan;
        } else {
            text = format!("{} -{}", next.name(), format_time(rem));
            tooltip = format!("{} at {}", next.name(), prayers.time(next).with_timezone(&Local).format("%H:%M"));
            if rem.num_minutes() <= 10 {
                class = TimeStatus::Before;
            } else {
                class = TimeStatus::Remaining;
            }
        }
    } else if past.num_minutes() == 0 {
        text = curr.name();
        tooltip = format!("Time for {}", text);
        class = TimeStatus::Athan;
    } else {
        text = format!("{} +{}", curr.name(), format_time(past));
        tooltip = format!("{} at {}", curr.name(), prayers.time(curr).with_timezone(&Local).format("%H:%M"));
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

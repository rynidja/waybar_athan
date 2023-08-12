use clap::{Parser, ValueEnum};
use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule, PrayerTimes};
use chrono::Local;

// that's it for now :(
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    latitude: f32,

    #[arg(short = 'L', long)]
    longitude: f32,

    /// Prayer time calculion method
    #[arg(short, long, value_enum, default_value_t = MethodArg::Mwl)]
    method: MethodArg,

    #[arg(short = 'M', long, value_enum, default_value_t = MadhabArg::Shafi)]
    madhab: MadhabArg,

    /// Print the output in i3blocks style, otherwise json 
    #[arg(short = 'v', long)]
    i3blocks: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MadhabArg {
    /// Shafi, Maliki, Hanbali
    Shafi,
    /// Hanafi
    Hanafi,
}

impl From<MadhabArg> for Madhab {
    fn from(madhab: MadhabArg) -> Self {
        match madhab {
            MadhabArg::Shafi => Madhab::Shafi,
            MadhabArg::Hanafi => Madhab::Hanafi,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum MethodArg {
    /// MuslimWorldLeague
    Mwl,
    /// Egyptian
    Egy,
    /// Karachi
    Kara,
    /// UmmAlQura
    Uaq,
    /// NorthAmerica
    Na,
    /// Singapore
    Sg,
    /// French Muslims
    Fr,
    /// Spiritual Administration of Muslims of Russia
    Rus,
    /// Fixed Ishaa Time Interval, 90min
    Fi,
}

impl From<MethodArg> for Method {
    fn from(method: MethodArg) -> Self {
        match method {
            MethodArg::Mwl => Method::MuslimWorldLeague,
            MethodArg::Egy => Method::Egyptian,
            MethodArg::Kara => Method::Karachi,
            MethodArg::Uaq => Method::UmmAlQura,
            MethodArg::Na => Method::NorthAmerica,
            MethodArg::Sg => Method::Singapore,
            MethodArg::Fr => Method::French,
            MethodArg::Rus => Method::Russia,
            MethodArg::Fi => Method::FixedInterval,
        }
    }
}

pub fn parse() -> (PrayerTimes, bool) {
    let args = Args::parse();

    let method: Method = args.method.into();
    let madhab: Madhab = args.madhab.into();
    let location = Location::new(args.latitude, args.longitude);
    let today = Local::now().date_naive();
    let params = Config::new().with(method, madhab);

    (
        PrayerSchedule::new(location)
            .unwrap()
            .on(today)
            .with_config(params)
            .calculate()
            .unwrap(),
        args.i3blocks,
    )
}

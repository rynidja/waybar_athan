use clap::{Parser, ValueEnum};

use salah::prelude::*;

// that's it for now :(
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    latitude: f64,

    #[arg(short = 'L', long)]
    longitude: f64,

    /// Prayer time calculion method
    #[arg(short, long, value_enum, default_value_t = MethodArg::Mwl)]
    method: MethodArg,

    #[arg(short = 'M', long, value_enum, default_value_t = MadhabArg::Shafi)]
    madhab: MadhabArg,

    /// Print the output in separate lines format, otherwise waybar json format
    #[arg(short, long)]
    standalone: bool,
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
    /// Dubai
    Du,
    /// MoonsightingCommittee
    Mc,
    /// NorthAmerica
    Na,
    /// Kuwait
    Kw,
    /// Qatar
    Qa,
    /// Singapore
    Sg,
    /// Tehran
    T,
    /// Turkey
    Tr,
}

impl From<MethodArg> for Method {
    fn from(method: MethodArg) -> Self {
        match method {
            MethodArg::Mwl => Method::MuslimWorldLeague,
            MethodArg::Egy => Method::Egyptian,
            MethodArg::Kara => Method::Karachi,
            MethodArg::Uaq => Method::UmmAlQura,
            MethodArg::Du => Method::Dubai,
            MethodArg::Mc => Method::MoonsightingCommittee,
            MethodArg::Na => Method::NorthAmerica,
            MethodArg::Kw => Method::Kuwait,
            MethodArg::Qa => Method::Qatar,
            MethodArg::Sg => Method::Singapore,
            MethodArg::T => Method::Tehran,
            MethodArg::Tr => Method::Turkey,
        }
    }
}

pub fn parse() -> (PrayerTimes, bool) {
    let args = Args::parse();

    let method: Method = args.method.into();
    let madhab: Madhab = args.madhab.into();
    let city = Coordinates::new(args.latitude, args.longitude);
    let today = Utc::now().date();
    let params = Configuration::with(method, madhab);

    (
        PrayerSchedule::new()
            .on(today)
            .for_location(city)
            .with_configuration(params)
            .calculate()
            .expect("Err: somthing is wrong in the config"),
        args.standalone,
    )
}

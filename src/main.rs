use utils::format::gen_output;
mod utils;

fn main() {
    let (prayers, i3blocks_style) = utils::args::parse();

    println!("{}", gen_output(prayers, i3blocks_style));
}

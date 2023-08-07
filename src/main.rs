/* TODO:
*   notification
* */

use utils::format::gen_output;
mod utils;

fn main() {
    let (prayers, standalone) = utils::args::parse();

    println!("{}", gen_output(prayers, standalone));
}

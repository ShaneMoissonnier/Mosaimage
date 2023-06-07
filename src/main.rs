mod buffer;
mod color;
mod constants;
mod mosaic;
mod parser;

use clap::Parser;
use mosaic::generate_mosaic;

fn main() {
    generate_mosaic(parser::Arguments::parse());
}

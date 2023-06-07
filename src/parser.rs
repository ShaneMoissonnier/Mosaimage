use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
pub struct Arguments {
    #[arg(short, long)]
    pub input_image_path: String,

    #[arg(short, long)]
    pub source_image_path: String,

    #[arg(short, long)]
    pub output_image_path: String,
}

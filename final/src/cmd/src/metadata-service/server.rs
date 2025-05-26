use clap::command;
use clap::Parser;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG : &str = "config/placement-center.toml";

#[derive(Parser,Debug)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value=String::form(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf:String
}


fn main() {
    println!("Hello.")
}

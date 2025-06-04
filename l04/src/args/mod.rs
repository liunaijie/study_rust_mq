use clap::Parser;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";

#[derive(Parser, Debug)]
#[command(next_line_help = true)]
pub struct ArgsParams {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    pub conf: String,
}

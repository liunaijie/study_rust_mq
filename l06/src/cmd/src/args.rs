use clap::Parser;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";
pub const DEFAULT_LOG_CONFIG: &str = "config/log4rs.yaml";
pub const DEFAULT_LOG_PATH: &str = "/logs";

#[derive(Parser, Debug)]
#[command(next_line_help = true)]
pub struct ArgsParams {
    // /// Name of the person to greet
    #[arg(default_value_t = String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    pub conf: String,
    #[arg(default_value_t = String::from(DEFAULT_LOG_CONFIG))]
    pub log_config: String,
    #[arg(default_value_t = String::from(DEFAULT_LOG_PATH))]
    pub log_path: String,

}

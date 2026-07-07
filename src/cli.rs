use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "proximity-server",
    version,
    about = "Receive inference results from DeepStream"
)]
pub struct Opts {
    /// Unix Domain Socket path
    #[arg(
        long,
        short = 's',
        value_name = "path",
        default_value = "/tmp/ds.sock",
        help = "Unix socket path"
    )]
    pub socket: String,

    /// Configuration file
    #[arg(
        long,
        short = 'c',
        value_name = "file",
        default_value = "config.toml",
        help = "Configuration file"
    )]
    pub config: String,
}

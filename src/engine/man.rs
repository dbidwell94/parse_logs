use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Devin Bidwell <dbidwell94@biddydev.com>", name = "FFTB (Fast Fail2Ban)", version, about, long_about = None)]
#[command(help_template = "{name} -- {author} \n {usage-heading} {usage} \n {all-args} {tab}")]
pub struct Args {
    #[cfg_attr(debug_assertions, arg(short, long, default_value = "./config.yaml"))]
    #[cfg_attr(
        not(debug_assertions),
        arg(short, long, default_value = "/etc/ff2b/config.yaml")
    )]
    /// Location of the config path used to tell ff2b where log files are
    /// and how to parse the logs
    pub config_path: String,
    #[arg(short, long, default_value = "false")]
    /// Performs a dry startup in order to test the parsing of the requested configuration.
    /// May be used in conjunction with `-c`.
    pub test_config: bool,
}

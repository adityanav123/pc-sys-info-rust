use clap::Parser;
use env_logger::Env;
use log::LevelFilter;
use log::{info, trace};
use pc_info::PcInfo;
use sysinfo::System;

mod pc_info;

#[derive(Parser, Debug)]
struct Args {
    /// Show debug logs (Debug + INFO)
    #[arg(long)]
    verbose: bool,

    /// Show trace logs (everything)
    #[arg(long)]
    debug: bool,

    /// GET PC Information
    #[arg(long)]
    pcinfo: bool,

    /// No Logging Info
    #[arg(long)]
    quiet: bool,
}

fn init_logger(args: &Args) {
    let default_level = if args.debug {
        LevelFilter::Trace
    } else if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(default_level.as_str())).init()
}

fn main() {
    let mut system = System::new_all();
    let cli = Args::parse();
    if !cli.quiet {
        init_logger(&cli);
    }

    if cli.pcinfo {
        trace!("Fetching PC Info");
        let mut pc = PcInfo::new();
        pc.fetch_data(&mut system);
        info!("PC info collected");
        println!("{}", pc.to_json());
    }
}

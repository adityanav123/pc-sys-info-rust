use clap::Parser;
use env_logger::Env;
use log::LevelFilter;
use log::{info, trace};
use pc_info::PcInfo;
use sysinfo::System;

mod pc_info;
mod utils;

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

    /// Pretty-print JSON output
    #[arg(long)]
    pretty: bool,

    /// Run a program (no shell). Example: --exec python -- --version
    #[arg(long)]
    exec: Option<String>,

    /// Everything after `--` is passed to the executed program
    #[arg(last = true)]
    exec_args: Vec<String>,

    /// Run a shell command (bash -lc). Example: --shell "dmesg | tail -n 20"
    #[arg(long)]
    shell: Option<String>,
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

fn print_json<T: serde::Serialize>(value: &T, pretty: bool) {
    if pretty {
        println!("{}", serde_json::to_string_pretty(value).unwrap());
    } else {
        println!("{}", serde_json::to_string(value).unwrap());
    }
}

fn main() -> std::io::Result<()> {
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

    // Shell mode
    if let Some(cmdline) = cli.shell.as_deref() {
        trace!("Executing Shell: {}", cmdline);
        match utils::execute(utils::ExecMode::Shell {
            cmd: cmdline.to_string(),
        }) {
            Ok(res) => print_json(&res, cli.pretty),
            Err(e) => eprintln!("Failed to execute shell: {}", e),
        }
    }

    // Exec mode (no shell)
    if let Some(program) = cli.exec.as_deref() {
        trace!("Executing Exec: {} {:?}", program, cli.exec_args);
        match utils::execute(utils::ExecMode::Exec {
            program: program.to_string(),
            args: cli.exec_args.clone(),
        }) {
            Ok(res) => print_json(&res, cli.pretty),
            Err(e) => eprintln!("Failed to execute exec: {}", e),
        }
    }

    Ok(())
}

//! Aura CLI — Sentinel-first entry. Default: enforce + deny-all.
//!
//! No model, tool, network, or file work is offered here yet. Status and an
//! explicit boot-continue attempt (which fails under deny-all) prove the gate.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use aura_runtime::{BootSupervisor, DecisionLog, SentinelMode};

fn main() -> ExitCode {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        args.push("status".into());
    }

    let mode = parse_mode(&args);
    let data_dir = data_dir();
    let log_path = data_dir.join("decisions.jsonl");

    let log = match DecisionLog::open(&log_path) {
        Ok(l) => Arc::new(l),
        Err(e) => {
            eprintln!("FATAL: decision ledger failed: {e}");
            eprintln!("Aura refuses to start without a seal path (Forever Law).");
            return ExitCode::from(2);
        }
    };

    let mut boot = BootSupervisor::start(mode, log);

    match args[0].as_str() {
        "status" => {
            let status = boot.status();
            println!("{}", serde_json::to_string_pretty(&status).unwrap());
            ExitCode::SUCCESS
        }
        "boot-continue" => match boot.try_continue_boot() {
            Ok(status) => {
                println!("{}", serde_json::to_string_pretty(&status).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("boot-continue refused: {e}");
                let status = boot.status();
                println!("{}", serde_json::to_string_pretty(&status).unwrap());
                ExitCode::from(1)
            }
        },
        "help" | "-h" | "--help" => {
            print_help();
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn parse_mode(args: &[String]) -> SentinelMode {
    // Default enforce — there is no gate before the Sentinel.
    let mut mode = SentinelMode::Enforce;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--sentinel-mode" {
            if let Some(v) = args.get(i + 1) {
                if let Some(parsed) = SentinelMode::parse(v) {
                    mode = parsed;
                    if matches!(mode, SentinelMode::Shadow) {
                        eprintln!(
                            "WARNING: --sentinel-mode shadow is a logged opt-down; effects stay blocked"
                        );
                    }
                } else {
                    eprintln!("invalid --sentinel-mode {v}; using enforce");
                }
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    mode
}

fn data_dir() -> PathBuf {
    if let Ok(p) = env::var("AURA_DATA_DIR") {
        return PathBuf::from(p);
    }
    PathBuf::from("data")
}

fn print_help() {
    eprintln!(
        "aura — Sentinel-first runtime (L0)\n\
         \n\
         Usage:\n\
           aura status [--sentinel-mode enforce|shadow]\n\
           aura boot-continue [--sentinel-mode enforce|shadow]\n\
         \n\
         Default mode: enforce + deny-all policy.\n\
         boot-continue maps to Core action effect.execute / aura://boot/continue.\n\
         Under deny-all it must fail closed and leave phase=initializing.\n\
         \n\
         Law: There is no gate before the Sentinel.\n"
    );
}

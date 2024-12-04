use clap::{error::ErrorKind, CommandFactory, Parser};
use env_logger::Env;
use log::LevelFilter;
use options::{Args, Format};
use saldo::SaldoLexicon;

mod options;

fn main() {
    let args = Args::parse();
    dbg!(&args);

    configure_logging(args.verbose);
    let eval = matches!(args.format, Format::Eval);
    if eval && args.eval_lemmas.is_none() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "evalLemmas not specified, required when --format=eval",
        )
        .exit();
    }
    if eval && args.eval_key.is_none() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "evalKey not specified, required when --format=eval",
        )
        .exit();
    }
    if eval && args.for_lemma.is_none() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "forLemma not specified, required when --format=eval",
        )
        .exit();
    }

    let saldo = args
        .saldo
        .as_ref()
        .map(|saldo_file| SaldoLexicon::new(saldo_file));
    // let wsd = WSDApplication::make(saldo, args.app_name);
}

fn configure_logging(level: u8) {
    let log_level = match level {
        3 => LevelFilter::Trace,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Debug,
    };
    env_logger::builder().filter_level(log_level).init();
}

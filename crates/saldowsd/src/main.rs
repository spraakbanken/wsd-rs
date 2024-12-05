use log::LevelFilter;
use options::Args;
use saldo::SaldoLexicon;

mod options;

fn main() -> miette::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Args::parse(&argv).map_err(|err| {
        usage();
        err
    })?;

    configure_logging(args.verbose);

    // let wsd = WSDApplication::make(saldo, args.app_name);
    let saldo = match &args.saldo {
        None => None,
        Some(saldo_file) => Some(SaldoLexicon::new(saldo_file)?),
    };
    Ok(())
}

fn usage() {
    eprintln!("Usage: saldowsd -appName=APP_NAME [-saldo=SALDO]");
    eprintln!("");
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

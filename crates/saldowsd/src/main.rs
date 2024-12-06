use std::io;

use log::LevelFilter;
use miette::IntoDiagnostic;
use options::Args;
use saldo::SaldoLexicon;
use wsd_application::{
    make_wsd_application,
    wsd_application::{evaluate, read_sentences},
};

mod options;

fn main() -> miette::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = Args::parse(&argv).map_err(|err| {
        usage();
        err
    })?;

    configure_logging(args.verbose);

    let saldo = match &args.saldo {
        None => None,
        Some(saldo_file) => Some(SaldoLexicon::new(saldo_file)?),
    };

    let wsd = make_wsd_application(saldo.as_ref(), &args.app_name, &argv)?;

    if args.eval {
        evaluate(wsd, &args.eval_lemmas.unwrap(), &args.eval_key.unwrap());
        return Ok(());
    }

    // let mut ratios = None;
    if !args.for_lemma.is_none() {
        todo!("ratios = Some(HashMap::new())");
    }
    let mut total_sentences = 0;
    let mut next_print = 100000;

    let mut stdin = io::stdin().lock();

    loop {
        let text = read_sentences(
            &mut stdin,
            saldo.as_ref(),
            args.sbxml,
            args.batch_size,
            args.split_mwes,
            args.split_compounds,
        )
        .into_diagnostic()?;
        if text.len() == 0 {
            break;
        }
        total_sentences += text.len();
        if total_sentences > next_print {
            log::info!("{}", next_print);
            next_print += 100000;
        }

        if !args.for_lemma.is_none() {
            todo!("forLemma not supported yet");
        } else {
            let result = wsd.disambiguate_text(text);
            for p in result {
                for i in 0..p.0.len() {
                    print!("{}\t", &p.0[i]);
                    match &p.1[i] {
                        None => println!("_"),
                        Some(scores) => println!("{}", join_to_string(scores)),
                    }
                }
                println!();
            }

            if total_sentences >= args.max_sen {
                break;
            }
        }
    }

    if !args.for_lemma.is_none() {
        todo!("printRatios(ratios)");
    }

    // TODO split into chunks and use thread pool
    Ok(())
}

fn usage() {
    eprintln!("Usage: saldowsd -appName=APP_NAME [-saldo=SALDO]");
    eprintln!("");
}

fn join_to_string(vs: &[f32]) -> String {
    if vs.is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(vs.len() * 2);
    out.push_str(&vs[0].to_string());
    for v in &vs[1..] {
        out.push_str(&format!("|{}", v));
    }
    out
}
fn configure_logging(level: u8) {
    let log_level = match level {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    env_logger::builder().filter_level(log_level).init();
}

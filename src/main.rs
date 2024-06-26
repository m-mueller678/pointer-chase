use crate::pointer_cycle::PointerCycle;
use clap::Parser;
use minstant::Instant;
use std::mem::size_of;
use std::process::exit;

mod pointer_cycle;

fn main() {
    let cli = Cli::parse();
    if cli.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
        return;
    }
    let pointer_count = cli.size / size_of::<*const u8>();
    let rounded_size = pointer_count * size_of::<*const u8>();
    assert!(pointer_count > 0);
    if cli.print_header {
        println!("size,time,latency",);
    }
    if cli.exit == Some(0) {
        return;
    }

    let cycle = PointerCycle::build(pointer_count);
    let start = Instant::now();
    let mut ref_time = start;
    let mut iterations: u64 = 0;
    let mut print_count = 0;
    cycle.walk_loop(
        #[inline(always)]
        || {
            iterations += 1;
            if iterations % (1 << 14) == 0 {
                let now = Instant::now();
                let duration = now.duration_since(ref_time);
                if now.duration_since(start).as_millis() >= (cli.time * (print_count + 1)) as u128 {
                    let time_per_pointer = duration.as_nanos() as f64 / iterations as f64;
                    print_count += 1;
                    println!(
                        "{rounded_size},{},{time_per_pointer}",
                        now.duration_since(start).as_secs_f64()
                    );
                    if Some(print_count) == cli.exit {
                        exit(0);
                    } else {
                        ref_time = now;
                        iterations = 0;
                    }
                }
            }
        },
    );
}

/// Generates a random cycle of pointers and walks it in a loop, recording the time per pointer access.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Size of the pointer cycle in bytes. Will be rounded down to next multiple of pointer width.
    #[arg(short, long)]
    size: usize,
    /// Duration in milliseconds
    #[arg(short, long, default_value = "1000")]
    time: u64,
    /// Exit after printing set number of measurements
    #[arg(short, long)]
    exit: Option<u64>,
    /// Print header
    #[arg(long)]
    print_header: bool,
    #[arg(long, hide = true)]
    markdown_help: bool,
}

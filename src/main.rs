use crate::pointer_cycle::PointerCycle;
use clap::Parser;
use minstant::Instant;
use std::mem::size_of;

mod pointer_cycle;

fn main() {
    let x = Cli::parse();
    let pointer_count = x.size / size_of::<*const u8>();
    let rounded_size = pointer_count * size_of::<*const u8>();
    assert!(pointer_count > 0);
    if x.print_header {
        if x.print_size {
            print!("size,");
        }
        println!("time,latency",);
    }
    if x.exit == Some(0) {
        return;
    }

    let cycle = PointerCycle::build(pointer_count);
    let start = Instant::now();
    let mut ref_time = start;
    let mut iterations = 0;
    let mut print_count = 0;
    loop {
        cycle.walk();
        iterations += 1;
        let now = Instant::now();
        let duration = now.duration_since(ref_time);
        if duration.as_millis() >= x.time as u128 {
            let time_per_pointer = duration.as_nanos() / (iterations * pointer_count) as u128;
            print_count += 1;
            if x.print_size {
                print!("{rounded_size},");
            }
            println!(
                "{},{time_per_pointer}",
                now.duration_since(start).as_secs_f64()
            );
            if Some(print_count) == x.exit {
                break;
            } else {
                ref_time = now;
                iterations = 0;
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Size of the pointer cycle in bytes. Will be rounded down to next multiple of pointer width.
    #[arg(short, long)]
    size: usize,
    /// Duration in milliseconds
    #[arg(short, long, default_value = "1000")]
    time: u64,
    /// Exit after printing N measurements
    #[arg(short, long)]
    exit: Option<usize>,
    /// Print size in addition to recorded timings.
    #[arg(long)]
    print_size: bool,
    /// Print header
    #[arg(long)]
    print_header: bool,
}

use clap::Parser;

fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    use rand::distr::Alphanumeric;

    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn do_sleep(seconds: u64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_secs(seconds));
}

#[derive(Parser, Debug)]
/// scorn is a CLI tool to generate <REPEAT> random strings of length <LENGTH> and sleeps for <SLEEP> seconds between outputs.
///
/// The program is designed to be run from the command line and takes the following arguments:
/// 
/// `--length`: The length of the random string to generate.
/// 
/// `--sleep`: The number of seconds to sleep between generating random strings.
///
/// `--repeat`: The number of times to repeat the process of generating a random string and sleeping.
///
/// The program uses the `clap` crate for command line argument parsing.
/// The program uses the `rand` crate to generate random strings and the `std::thread` module to sleep.
struct Args {
    /// number of random characters to generate
    #[clap(short, long, default_value = "32")]
    length: usize,
    /// number of seconds to sleep
    #[clap(short, long, default_value = "10")]
    sleep: u64,
    /// number of times to repeat
    #[clap(short, long, default_value = "1000000000")]
    repeat: u64,
}

fn main() {
    let args = Args::parse();
    let length = args.length;
    let sleep = args.sleep;
    let repeat = args.repeat;
    for _ in 0..repeat {
        let random_string = generate_random_string(length);
        println!("{random_string}");
        do_sleep(sleep);
    }
}

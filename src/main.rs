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
/// sorn is a CLI tool to generate <REPEAT> random strings each consisting of <COMPONENTS> random strings 
/// each of length <LENGTH> and separated by `-`. Between the output of two random strings, 
/// the program sleeps for <SLEEP> seconds. The program will print the output in double 
/// quotes unless the --raw option is set. Usually, we use this program to generate unique error numbers.
/// 
struct Args {
    /// number of random characters to generate
    #[clap(short, long, default_value = "6")]
    length: usize,
    /// number of seconds to sleep
    #[clap(short, long, default_value = "1")]
    sleep: u64,
    /// number of times to repeat
    #[clap(short, long, default_value = "1000000000")]
    repeat: u64,
    /// unless option --raw is set, we will print the random string in double quotes
    /// Note that there is not short version since -r defines the repeat option
    #[clap(long, default_value = "false")]
    raw: bool,
    /// number of components to generate.
    #[clap(short, long, default_value = "3")]
    components: u64,
}

fn main() {
    let args = Args::parse();
    let delimiter = if args.raw { "" } else { "\"" };
    for i in 0..args.repeat {
        let mut separator = "".to_string();
        print!("{delimiter}"); // Print in double quotes unless --raw is set
        for _ in 0..args.components {
            print!(
                "{separator}{random_string}",
                random_string = generate_random_string(args.length)
            );
            separator = "-".to_string();
        }
        println!("{delimiter}");
        if i != args.repeat - 1 {
            // Sleep only if not the last iteration
            // This is to avoid sleeping after the last output
            // and to ensure that we exit after the last output
            // without any delay.
            do_sleep(args.sleep);
        }
    }
}

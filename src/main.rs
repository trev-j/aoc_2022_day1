use std::env;
use std::process;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let calorie_list_contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut max_calories: usize = 0;
    let mut max_three: Vec<usize> = vec![0,0,0];
    let mut current_calories: usize = 0;

    for line in calorie_list_contents.lines() {
        if line == "" {
            for i in (0..3).rev() {
                // Shift max values in result array
                if max_three[i] == 0 || max_three[i] < current_calories {
                    for j in 0..i {
                        max_three[j] = max_three[j + 1];
                    }
                    max_three[i] = current_calories;
                    break;
                }
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }

    for calories in max_three {
        max_calories += calories;
    }

    println!("Max calories held: {}", max_calories);
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path: file_path})
    }
}
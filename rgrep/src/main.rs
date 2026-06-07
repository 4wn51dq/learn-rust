use regex::Regex;
use std::env;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()< 3 {
        println!("Usage: rgrep [OPTION] <pattern> <path>");
        eprintln!("Options: -v -c -l");
        std::process::exit(1);
    }

    let mut invert = false;
    let mut count_only = false;
    let mut files_only = false;
    let mut pattern_index = 1;

    for i in 1..args.len() { //1.. and not 0.. because args[0] is the program name.
        match args[i].as_str() {
            "-v" => invert = true,
            "-c" => count_only = true,
            "-l" => files_only = true,
            _ => {
                pattern_index = i;
                break;
            }
        }
    }
    if pattern_index+1>= args.len(){
        eprintln!("Error: missing pattern or path");
        std::process::exit(1);
    }

    let pattern = &args[pattern_index];
    let path = &args[pattern_index+1];

    let regex = match Regex::new(pattern) { // take the pattern string and compile it into a regex object
        Ok(r) => r, //r is the regex object
        Err(e) => {
            eprintln!("Unmatched pattern: {}", e);
            std::process::exit(1);
        }
    }; 
    // Regex::new returns a Result<Regex, Error>, this is a enum with two variants: Ok(value) meaning 
    // success and Err(error) meaning failure.

    for entry in WalkDir::new(path).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(_)=> continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let file_path = entry.path();

        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
    
        let matches = search(&contents, &regex, invert);
    
        if files_only {
            if !matches.is_empty() {
                println!("{}", file_path.display());
            }
            continue;
            // file_path.display() converts the Path into something printable.
            // Path doesn't implement Display directly but .display() returns a wrapper that does.
        }
        if count_only {
            println!("{}: {}", file_path.display(), matches.len());
            continue;
        }
        for i in 0..matches.len() {
            let line_num = matches[i].0;
            let line = &matches[i].1;
            println!("{}: {}: {}", file_path.display(), line_num, line);
        }
    }
}

fn search(contents: &str, regex: &Regex, invert: bool) -> Vec<(usize, String)> {
    let mut results: Vec<(usize, String)> = Vec::new();
    let mut line_number = 1;

    for line in contents.lines() {
        let is_match = regex.is_match(line);

        let should_include = if invert {!is_match} else {is_match};

        if should_include {
            results.push((line_number, line.to_string()));
        }

        line_number+=1;
    }
    results
}

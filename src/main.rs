use std::fs;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn format_result(display: &std::path::Display, index:usize, line: &str) -> String {
    // ANSI escape code
    let green_color = "\x1b[32m";
    let yellow_color = "\x1b[33m";
    let reset_color = "\x1b[0m";

    format!("{}{}{} {}(line {}){}: {}", green_color, display, reset_color, yellow_color, index + 1, reset_color, line.trim())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: tsea <search word>");
        return;
    }

    let query = &args[1];
    let current_dir = Path::new(".");
    
    match fs::read_dir(current_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |e| e == "txt") {
                        let results = search_in_file(&path, &query);
                        for line in results {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
        Err(_) => {
            println!("Error: An error read while loading the directory");
        }
    }
}

fn search_in_file(path: &Path, query: &str) -> Vec<String> {
    let display = path.display();
    let mut results = Vec::new();
    
    let file = match fs::File::open(&path) {
        Err(_) => return results,
        Ok(file) => file,
    };
    
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(query) {
            results.push(format_result(&display, index, &line));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_search_in_file() {
        let test_file_path = Path::new("temp_test_file.txt");
        
        // Create test file
        let mut file = fs::File::create(&test_file_path).unwrap();
        writeln!(file, "Hello\nThis is a test\nAnother test line").unwrap();

        let results = search_in_file(&test_file_path, "test");

        // Clean
        fs::remove_file(&test_file_path).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.contains(&"temp_test_file.txt (line 2): This is a test".to_string()));
        assert!(results.contains(&"temp_test_file.txt (line 3): Another test line".to_string()));
    }
}

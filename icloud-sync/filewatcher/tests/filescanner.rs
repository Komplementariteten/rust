#[cfg(test)]
mod tests {
    use filewatcher::filescanner::{scan, scan_ordered};

    #[test]
    fn filescanner_gets_all_relative_files() {
        let r = scan("..", true).unwrap();
        for file in r {
            println!("File: {}", file.path.to_str().unwrap());
        }
    }
    #[test]
    fn filescanner_gets_all_idrive_files() {
        let r = scan_ordered("/Users/me/Library/Mobile Documents/com~apple~CloudDocs/Dokumente", true).unwrap();
        println!("{}", &r.len());
        for file in r {
            println!("File: {}", file.path.to_str().unwrap());
        }
    }
}
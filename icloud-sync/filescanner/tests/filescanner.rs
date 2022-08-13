#[cfg(test)]
mod tests {
    use filescanner::scan;

    #[test]
    fn filescanner_gets_all_relative_files() {
        let r = scan("..").unwrap();
        for file in r {
            println!("File: {}", file.path.to_str().unwrap());
        }
    }
    #[test]
    fn filescanner_gets_all_idrive_files() {
        let r = scan("/Users/me/Library/Mobile Documents/com~apple~CloudDocs/Dokumente").unwrap();
        println!("{}", &r.len());
        for file in r {
            println!("File: {}", file.path.to_str().unwrap());
        }
    }
}
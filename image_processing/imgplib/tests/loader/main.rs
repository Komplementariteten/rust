#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn load_img_file_not_exists_reports_error() {
        use imgplib::loader::*;
        let result = load_img("note existing image");
        assert!(result.is_err());
    }

    #[test]
    fn load_img_file_works() {
        use imgplib::loader::*;
        let result = load_img("tests/loader/test.jpeg");
        assert!(result.is_ok());
    }
}
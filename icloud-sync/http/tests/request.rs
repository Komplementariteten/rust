#[cfg(test)]
mod tests {
    use http::request::Request;

    #[test]
    fn get_gets() {
        "http://www.google.de".gets().expect("Get Failed").json().bytes();
    }
}
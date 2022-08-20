#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::io::{BufReader, BufWriter, Read};
    use std::str::from_utf8;
    use time::OffsetDateTime;
    use http::protocol::{BaseHttpRouting, HttpResponse, HttpVerb, read_header, respond};

    struct TestRouting {
    }

    impl BaseHttpRouting for TestRouting {
        fn get(&self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 1,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }
        }

        fn post<R: Read>(&self, resource: String, aditional_header: HashMap<String, String>, stream: &mut R) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 2,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }
        }

        fn head(&self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 3,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }
        }

        fn put<R: Read>(&self, resource: String, aditional_header: HashMap<String, String>, stream: &mut R) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 4,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }

        }

        fn delete(&self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 5,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }

        }

        fn options(&self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 6,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![]
            }
        }
    }

    #[test]
    fn protocol_does_route() {
        let http = "GET /hello.txt HTTP/1.1
User-Agent: curl/7.64.1
Host: www.example.com
Accept-Language: en, mi

<body></body>".as_bytes();
        let mut br = BufReader::new(http);
        let mut bw = BufWriter::new(Vec::new());
        let routing = TestRouting{};
        let res = respond(br, bw.borrow_mut(), routing);
        assert!(res.is_ok());
        let utf8 = from_utf8(bw.get_ref());
        assert!(utf8.is_ok());
        let utf8_str = utf8.unwrap();
        let first = utf8_str.lines().next();
        assert!(first.is_some());
        assert_eq!(first.unwrap() ,"HTTP/1.1 1 ");
    }

    #[test]
    fn protocol_finds_get_header() {
        let http = "GET /hello.txt HTTP/1.1
User-Agent: curl/7.64.1
Host: www.example.com
Accept-Language: en, mi

<body></body>".as_bytes();
        let mut br = BufReader::new(http);
        let res = read_header(&mut br);
        assert!(res.is_ok());
        let header = res.unwrap();
        assert_eq!(header.verb, HttpVerb::Get);
    }

    #[test]
    fn protocol_finds_post_header() {
        let http = "POST /hello.txt
User-Agent: curl/7.64.1
Host: www.example.com
Accept-Language: en, mi

<body></body>".as_bytes();
        let mut br = BufReader::new(http);
        let res = read_header(&mut br);
        assert!(res.is_ok());
        let header = res.unwrap();
        assert_eq!(header.verb, HttpVerb::Post);
    }

    #[test]
    fn protocol_response_can_into(){
        let r = HttpResponse{
            date: OffsetDateTime::now_utc(),
            status: 400,
            msg: "Bad Request".to_string(),
            is_compressed: false,
            header: Default::default(),
            content: vec![]
        };
        let bytes: Vec<u8> = r.into();
        assert!(!bytes.is_empty());
        let rq_bytes = from_utf8(&bytes);
        assert!(rq_bytes.is_ok());
        let rq_text = rq_bytes.unwrap();
        assert!(rq_text.lines().count() > 1);
        let intro = rq_text.lines().next();
        assert!(intro.is_some());
        let first = intro.unwrap();
        assert!(first.contains("HTTP/1.1 400"));
        assert!(rq_text.lines().next().is_some());
    }
}
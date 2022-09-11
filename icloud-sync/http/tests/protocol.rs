#[cfg(test)]
mod tests {
    use http::response::{read_header, BaseHttpRouting, HttpResponse, HttpVerb};
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::io::{BufReader, BufWriter, Read};
    use std::str::from_utf8;
    use time::OffsetDateTime;
    use http::request::HttpPath;

    struct TestRouting {}

    impl BaseHttpRouting for TestRouting {
        fn get(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 1,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }

        fn post(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
            stream: Vec<u8>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 2,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }

        fn head(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 3,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }

        fn put(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
            stream: Vec<u8>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 4,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }

        fn delete(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 5,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }

        fn options(
            &mut self,
            resource: String,
            aditional_header: HashMap<String, String>,
        ) -> HttpResponse {
            HttpResponse {
                header: Default::default(),
                status: 6,
                msg: "".to_string(),
                date: OffsetDateTime::now_utc(),
                is_compressed: false,
                content: vec![],
            }
        }
    }

    #[test]
    fn test_https_vaild_with_port() {
        let http_path = HttpPath::try_from("https://localhost:77");
        assert!(http_path.is_ok());
        let p = http_path.unwrap();
        assert!(p.ssl);
        assert_eq!(p.con, "localhost:77");
        assert_eq!(p.resource, "");
    }

    #[test]
    fn test_http_ip_vaild_without_port() {
        let http_path = HttpPath::try_from("http://1.1.1.1");
        assert!(http_path.is_ok());
        let p = http_path.unwrap();
        assert_eq!(p.ssl, false);
        assert_eq!(p.con, "1.1.1.1:80");
        assert_eq!(p.resource, "");
    }

    #[test]
    fn test_http_ip_vaild_with_port_and_path() {
        let http_path = HttpPath::try_from("http://1.1.1.1:77/hallo");
        assert!(http_path.is_ok());
        let p = http_path.unwrap();
        assert_eq!(p.ssl, false);
        assert_eq!(p.con, "1.1.1.1:77");
        assert_eq!(p.resource, "/hallo");
    }

    #[test]
    fn protocol_finds_get_header() {
        let http = "GET /hello.txt HTTP/1.1
User-Agent: curl/7.64.1
Host: www.example.com
Accept-Language: en, mi

<body></body>"
            .as_bytes();
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

<body></body>"
            .as_bytes();
        let mut br = BufReader::new(http);
        let res = read_header(&mut br);
        assert!(res.is_ok());
        let header = res.unwrap();
        assert_eq!(header.verb, HttpVerb::Post);
    }

    #[test]
    fn protocol_response_can_into() {
        let r = HttpResponse {
            date: OffsetDateTime::now_utc(),
            status: 400,
            msg: "Bad Request".to_string(),
            is_compressed: false,
            header: Default::default(),
            content: vec![],
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

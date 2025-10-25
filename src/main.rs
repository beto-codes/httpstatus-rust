use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Cell, Color, Table};
use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn get_status_codes() -> BTreeMap<u16, &'static str> {
    let mut map = BTreeMap::<u16, &'static str>::new();

    // 1xx Informational
    map.insert(100, "Continue");
    map.insert(101, "Switching Protocols");
    map.insert(102, "Processing");
    map.insert(103, "Early Hints");

    // 2xx Success
    map.insert(200, "OK");
    map.insert(201, "Created");
    map.insert(202, "Accepted");
    map.insert(203, "Non-Authoritative Information");
    map.insert(204, "No Content");
    map.insert(205, "Reset Content");
    map.insert(206, "Partial Content");
    map.insert(207, "Multi-Status");
    map.insert(208, "Already Reported");
    map.insert(226, "IM Used");

    // 3xx Redirection
    map.insert(300, "Multiple Choices");
    map.insert(301, "Moved Permanently");
    map.insert(302, "Found");
    map.insert(303, "See Other");
    map.insert(304, "Not Modified");
    map.insert(305, "Use Proxy");
    map.insert(306, "Switch Proxy");
    map.insert(307, "Temporary Redirect");
    map.insert(308, "Permanent Redirect");

    // 4xx Client Error
    map.insert(400, "Bad Request");
    map.insert(401, "Unauthorized");
    map.insert(402, "Payment Required");
    map.insert(403, "Forbidden");
    map.insert(404, "Not Found");
    map.insert(405, "Method Not Allowed");
    map.insert(406, "Not Acceptable");
    map.insert(407, "Proxy Authentication Required");
    map.insert(408, "Request Timeout");
    map.insert(409, "Conflict");
    map.insert(410, "Gone");
    map.insert(411, "Length Required");
    map.insert(412, "Precondition Failed");
    map.insert(413, "Payload Too Large");
    map.insert(414, "URI Too Long");
    map.insert(415, "Unsupported Media Type");
    map.insert(416, "Range Not Satisfiable");
    map.insert(417, "Expectation Failed");
    map.insert(418, "I'm a teapot");
    map.insert(421, "Misdirected Request");
    map.insert(422, "Unprocessable Entity");
    map.insert(423, "Locked");
    map.insert(424, "Failed Dependency");
    map.insert(425, "Too Early");
    map.insert(426, "Upgrade Required");
    map.insert(428, "Precondition Required");
    map.insert(429, "Too Many Requests");
    map.insert(431, "Request Header Fields Too Large");
    map.insert(451, "Unavailable For Legal Reasons");

    // 5xx Server Error
    map.insert(500, "Internal Server Error");
    map.insert(501, "Not Implemented");
    map.insert(502, "Bad Gateway");
    map.insert(503, "Service Unavailable");
    map.insert(504, "Gateway Timeout");
    map.insert(505, "HTTP Version Not Supported");
    map.insert(506, "Variant Also Negotiates");
    map.insert(507, "Insufficient Storage");
    map.insert(508, "Loop Detected");
    map.insert(510, "Not Extended");
    map.insert(511, "Network Authentication Required");

    map
}

fn print_json(status_codes: &BTreeMap<u16, &'static str>) {
    let json = match serde_json::to_string(status_codes) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to serialize JSON: {}", e);
            return;
        }
    };

    let child = Command::new("jq").arg(".").stdin(Stdio::piped()).spawn();
    match child {
        Ok(mut process) => {
            if let Some(mut stdin) = process.stdin.take() {
                let _ = stdin.write_all(json.as_bytes());
            }
            let _ = process.wait();
        }
        Err(_) => {
            println!(
                "{}",
                serde_json::to_string_pretty(status_codes).unwrap_or(json)
            );
        }
    }
}

fn print_table(status_codes: &BTreeMap<u16, &'static str>) {
    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.set_header(vec![
        Cell::new("Code").fg(Color::Cyan),
        Cell::new("Description").fg(Color::Yellow),
    ]);

    for (&code, &description) in status_codes {
        table.add_row(vec![
            Cell::new(code.to_string()).fg(Color::Red),
            Cell::new(description).fg(Color::Green),
        ]);
    }

    println!("{}", table);
}

fn main() {
    let status_codes = get_status_codes();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--json" || args[1] == "-j") {
        print_json(&status_codes);
    } else {
        print_table(&status_codes);
    }
}

#[cfg(test)]
mod tests {
    use crate::get_status_codes;

    #[test]
    fn test_status_codes_count() {
        let status_codes = get_status_codes();
        assert_eq!(status_codes.len(), 63, "Should have 63 HTTP status codes")
    }

    #[test]
    fn test_status_codes_order() {
        let status_codes = get_status_codes();
        let codes: Vec<u16> = status_codes.keys().copied().collect();
        for i in 1..codes.len() {
            assert!(
                codes[i - 1] < codes[i],
                "Codes should be in ascending order"
            )
        }
    }

    #[test]
    fn test_common_status_codes() {
        let status_codes = get_status_codes();

        assert_eq!(status_codes.get(&200), Some(&"OK"));
        assert_eq!(status_codes.get(&302), Some(&"Found"));
        assert_eq!(status_codes.get(&404), Some(&"Not Found"));
        assert_eq!(status_codes.get(&418), Some(&"I'm a teapot"));
    }

    #[test]
    fn test_status_code_ranges() {
        let status_codes = get_status_codes();
        let count_1xx = status_codes
            .keys()
            .filter(|&&c| (100..200).contains(&c))
            .count();
        let count_2xx = status_codes
            .keys()
            .filter(|&&c| (200..300).contains(&c))
            .count();
        let count_3xx = status_codes
            .keys()
            .filter(|&&c| (300..400).contains(&c))
            .count();
        let count_4xx = status_codes
            .keys()
            .filter(|&&c| (400..500).contains(&c))
            .count();
        let count_5xx = status_codes
            .keys()
            .filter(|&&c| (500..600).contains(&c))
            .count();

        assert_eq!(count_1xx, 4, "Should have 4 informational codes");
        assert_eq!(count_2xx, 10, "Should have 10 success codes");
        assert_eq!(count_3xx, 9, "Should have 9 redirection codes");
        assert_eq!(count_4xx, 29, "Should have 29 client error codes");
        assert_eq!(count_5xx, 11, "Should have 11 client error codes");
    }

    #[test]
    fn test_non_existent_status_code() {
        let status_codes = get_status_codes();
        assert!(!status_codes.contains_key(&999));
        assert!(!status_codes.contains_key(&666));
    }

    #[test]
    fn test_all_values_are_non_empty() {
        let status_codes = get_status_codes();
        for (code, description) in &status_codes {
            assert!(
                !description.is_empty(),
                "Code {} has an empty description",
                code
            )
        }
    }

    #[test]
    fn test_json_output_is_valid() {
        let status_codes = get_status_codes();
        let json = serde_json::to_string(&status_codes);
        assert!(json.is_ok(), "Should serialize to valid JSON");

        let json_str = json.unwrap();
        assert!(json_str.starts_with('{'));
        assert!(json_str.ends_with('}'));
        assert!(json_str.contains("\"100\""));
        assert!(json_str.contains("\"500\""));
    }
}

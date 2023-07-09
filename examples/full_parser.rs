use uaparser::{Parser, UserAgentParserBuilder};

fn main() {
    let parser = UserAgentParserBuilder::new()
        .build_from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    println!("{:?}", parser.parse("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"))
}

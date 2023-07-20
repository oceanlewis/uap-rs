use uaparser::{Parser, UserAgentParser};

fn main() {
    let parser = UserAgentParser::builder()
        .with_unicode_support(false)
        .with_device(false)
        .with_os(true)
        .with_user_agent(false)
        .build_from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    println!("{:?}", parser.parse("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"))
}

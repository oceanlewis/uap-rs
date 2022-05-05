use std::{fs::File, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_derive::Deserialize;
use uaparser::{Parser, UserAgentParser};

#[derive(Deserialize, Debug)]
struct TestCase {
    user_agent_string: String,
}

#[derive(Deserialize, Debug)]
struct TestCases {
    test_cases: Vec<TestCase>,
}

fn bench_os(c: &mut Criterion) {
    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = File::open("./src/core/tests/test_os.yaml").unwrap();
    let test_cases: TestCases = serde_yaml::from_reader(file).unwrap();

    c.bench_function("parse_os", |b| {
        b.iter(|| {
            for case in &test_cases.test_cases {
                black_box(parser.parse_os(&case.user_agent_string));
            }
        })
    });
}

fn bench_device(c: &mut Criterion) {
    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = File::open("./src/core/tests/test_device.yaml").unwrap();
    let test_cases: TestCases = serde_yaml::from_reader(file).unwrap();

    c.bench_function("parse_device", |b| {
        b.iter(|| {
            for case in &test_cases.test_cases {
                black_box(parser.parse_device(&case.user_agent_string));
            }
        })
    });
}

fn bench_ua(c: &mut Criterion) {
    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = std::fs::File::open("./src/core/tests/test_ua.yaml").unwrap();
    let test_cases: TestCases = serde_yaml::from_reader(file).unwrap();

    c.bench_function("parse_user_agent", |b| {
        b.iter(|| {
            for case in &test_cases.test_cases {
                black_box(parser.parse_user_agent(&case.user_agent_string));
            }
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(25))
        .measurement_time(Duration::from_secs(180))
        // degree of noise to ignore in measurements, here 1%
        .noise_threshold(0.01)
        // likelihood of noise registering as difference, here 5%
        .significance_level(0.05)
        // likelihood of capturing the true runtime, here 95%
        .confidence_level(0.95)
        // total number of bootstrap resamples, higher is less noisy but slower
        .nresamples(10_000)
        // total samples to collect within the set measurement time
        .sample_size(100);
    targets = bench_device, bench_os, bench_ua
);
criterion_main!(benches);

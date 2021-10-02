pub mod spec {
    extern crate serde;
    extern crate serde_json;

    use std::fs;
    use crate::util::util::execute_command;
    use serde::{Deserialize, Serialize};
    use similar::{ChangeTag, TextDiff};
    use std::io::{BufWriter, Write};
    use std::fs::File;

    enum TestStatus {
        Pass,
        Fail,
        New,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Spec {
        describe: String,
        command: String,
        results: String,
    }

    impl Spec {
        pub fn execute(&mut self) -> TestStatus {
            print!("\x1b[38;5;3mTEST\x1b[m {}", self.command);
            let actual = execute_command(self.command.as_str());
            let expected = match fs::read_to_string(&self.results) {
                Ok(results) => results,
                Err(_) => {
                    // only save snapshot
                    self.save_spec(actual);
                    println!("\r\x1b[38;5;221mNEW\x1b[m  {}", self.command);
                    return TestStatus::New;
                }
            };
            let diff = TextDiff::from_lines(
                expected.as_str(),
                actual.as_str(),
            );

            if diff.ratio() < 1.0 {
                println!("\r\x1b[38;5;196mFAIL\x1b[m {}", self.command);
                println!("++++++++diff++++++++");
                for change in diff.iter_all_changes() {
                    let sign = match change.tag() {
                        ChangeTag::Delete => format!("\x1b[38;5;27m-{}\x1b[m", change),
                        ChangeTag::Insert => format!("\x1b[38;5;196m+{}\x1b[m", change),
                        ChangeTag::Equal => format!("\x1b[38;5;241m {}\x1b[m", change),
                    };
                    print!("{}", sign);
                }
                println!("++++++++diff++++++++");
                TestStatus::Fail
            } else {
                println!("\r\x1b[38;5;42mPASS\x1b[m {}", self.command);
                TestStatus::Pass
            }
        }

        pub fn save_spec(&mut self, results: String) {
            let dir = ".spec/";
            match fs::create_dir_all(dir) {
                Ok(()) => {},
                Err(_) => {
                    panic!("Cannot create directory .spec");
                }
            };
            let filename = format!(".spec/{}_{}.snapshot", self.describe, self.command);
            let mut file = match File::create(&filename) {
                Ok(f) => f,
                Err(_) => {
                    panic!("Cannot create {}", filename);
                }
            };
            let mut writer = BufWriter::new(file);
            match writer.write(&results.as_bytes()) {
                Ok(_) => {
                    self.results = filename;
                },
                Err(_) => {
                    panic!("Cannot write text to file: {}", filename);
                }
            };
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SpecFile {
        name: String,
        specs: Vec<Spec>,
    }

    pub struct Summary {
        pub total: i64,
        pub success: i64,
        pub fail: i64,
        pub new: i64,
    }

    impl SpecFile {
        pub fn new(s: &str) -> SpecFile {
            let spec: SpecFile = match serde_json::from_str(s) {
                Ok(str) => str,
                Err(_) => {
                    panic!("Cannot read file: {}. You should add shellio.spec.json or specify shellio spec file path.", s);
                }
            };
            spec
        }

        pub fn execute(&mut self) -> Summary {
            let mut summary = Summary{
                total: self.specs.len() as i64,
                success: 0,
                fail: 0,
                new: 0,
            };
            for item in &mut self.specs {
                match item.execute() {
                    TestStatus::Pass => {
                        summary.success += 1;
                    },
                    TestStatus::Fail => {
                        summary.fail += 1
                    },
                    TestStatus::New => {
                        summary.new += 1;
                    },
                };
            }
            summary
        }
    }
}

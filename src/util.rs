pub mod util {
    use std::process::Command;
    use std::str::from_utf8;
    pub fn execute_command(command: &str) -> String {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", command])
                .output()
                .expect(&*format!("failed to execute command: {}", command))
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect(&*format!("failed to execute command: {}", command))
        };
        from_utf8(&*output.stdout).unwrap().to_string()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_execute_command() {
            if cfg!(target_os = "windows") {
                assert_eq!(execute_command("echo test"), "test\r\n");
            } else {
                assert_eq!(execute_command("echo test"), "test\n");
            }
        }
    }
}

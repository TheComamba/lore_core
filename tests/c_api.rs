#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_c_api() {
        let output = Command::new("python3")
            .arg("tests/c_api.py")
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success(),
            "Python script did not finish successfully"
        );
    }
}

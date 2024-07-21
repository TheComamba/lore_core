#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_c_api() {
        let output = Command::new("python3")
            .arg("tests/c_api.py")
            .output()
            .expect("Failed to execute command. Is python3 installed?");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        assert!(
            output.status.success(),
            "Python script did not finish successfully"
        );
    }
}

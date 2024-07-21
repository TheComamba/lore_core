#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_c_api() {
        let mut command = Command::new("python3");
        command.arg("tests/c_api.py");

        #[cfg(debug_assertions)]
        {
            println!("Calling python script in debug mode.");
        }

        #[cfg(not(debug_assertions))]
        {
            println!("Calling python script in release mode.");
            command.arg("--release");
        }

        let output = command
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

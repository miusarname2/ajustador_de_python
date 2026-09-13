pub fn review_python_installation() {
    println!("Reviewing Python installation...");
    #[cfg(target_os = "windows")]
    {
        // Windows-specific logic
        use std::process::Command;
        println!("Running on Windows. Checking Python installation...");
        let mut command = Command::new("py");
        command.args(["--version"]);
        let output = command.output().expect("Failed to execute command");
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Python is installed. Version: {}", version.trim());
        } else {
            println!("Python is not installed or not found in PATH.");
        }
        // Add your logic to review Python installation on Windows here
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Linux-specific logic
        use std::process::Command;
        println!("Running on Linux. Checking Python installation...");
        let mut command = Command::new("bash");
        command.args(&["-c", "python3 --version"]);
        let output = command.output().expect("Failed to execute command");
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("Python is installed. Version: {}", version.trim());
        } else {
            println!("Python is not installed or not found in PATH.");
        }
        // Add your logic to review Python installation on Linux here
    }
    // Add your logic to review Python installation here
}

pub fn review_pyenv_installation() {
    println!("Reviewing Python virtual environment installation...");
    #[cfg(target_os = "windows")]
    {
        // Windows-specific logic
        use std::process::Command;
        println!("Running on Windows. Checking Python virtual environment installation...");
        let mut command = Command::new("cmd");
        command.args(&["/C", "py -m venv --help"]);
        let output = match command.output() {
            Ok(output) => output,
            Err(error) => {
                println!("Could not execute the Python launcher: {error}");
                return;
            }
        };
        if output.status.success() {
            println!("Python virtual environments are available.");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Could not use Python to create a virtual environment.");
            println!("Python launcher error: {}", error.trim());
        }
    }
}

pub fn install_python_version(version: &str) -> bool {
    println!("Installing Python version {}...", version);
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = match Command::new("py")
            .args(["install", version])
            .output()
        {
            Ok(output) => output,
            Err(error) => {
                println!("Could not execute the Python launcher: {error}");
                return false;
            }
        };
        if output.status.success() {
            println!("Python version {} installed successfully.", version);
            true
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("The installed Python launcher cannot install versions automatically.");
            println!("Python launcher error: {}", error.trim());
            println!("Trying winget to install Python {}...", version);

            match Command::new("winget")
                .args([
                    "install",
                    "--id",
                    &format!("Python.Python.{}", version),
                    "--exact",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ])
                .status()
            {
                Ok(status) if status.success() => {
                    println!("Python version {} installed successfully with winget.", version);
                    true
                }
                Ok(_) => {
                    println!("Could not install Python version {} with winget.", version);
                    false
                }
                Err(error) => {
                    println!("winget is not available: {error}");
                    false
                }
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        let mut command = Command::new("bash");
        command.args(&["-c", &format!("pyenv install {}", version)]);
        let output = match command.output() {
            Ok(output) => output,
            Err(error) => {
                println!("Could not execute the pyenv command: {error}");
                return false;
            }
        };
        if output.status.success() {
            println!("Python version {} installed successfully.", version);
            true
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Could not install Python version {}.", version);
            println!("pyenv error: {}", error.trim());
            false
        }
    }
}

pub fn create_virtual_environment(venv_name: &str, python_version: &str) {
    println!("Creating virtual environment '{}' with Python version '{}'...", venv_name, python_version);
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = match Command::new("py")
            .args([&format!("-{python_version}"), "-m", "venv", venv_name])
            .output()
        {
            Ok(output) => output,
            Err(error) => {
                println!("Could not execute the Python launcher: {error}");
                return;
            }
        };
        if output.status.success() {
            println!("Virtual environment '{}' created successfully.", venv_name);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("The Python launcher could not find Python {}.", python_version);
            println!("Python launcher error: {}", error.trim());
            println!("Trying the Python executable available in PATH...");

            let version_output = match Command::new("python").arg("--version").output() {
                Ok(output) => output,
                Err(error) => {
                    println!("Could not execute Python from PATH: {error}");
                    return;
                }
            };
            let detected_version = format!(
                "{}{}",
                String::from_utf8_lossy(&version_output.stdout),
                String::from_utf8_lossy(&version_output.stderr)
            );

            if detected_version.contains(&format!("Python {python_version}")) {
                let fallback_output = Command::new("python")
                    .args(["-m", "venv", venv_name])
                    .output();

                match fallback_output {
                    Ok(output) if output.status.success() => {
                        println!("Virtual environment '{}' created successfully with Python {}.", venv_name, python_version);
                    }
                    Ok(output) => {
                        let error = String::from_utf8_lossy(&output.stderr);
                        println!("Could not create virtual environment '{}'.", venv_name);
                        println!("Python error: {}", error.trim());
                    }
                    Err(error) => println!("Could not execute Python from PATH: {error}"),
                }
            } else {
                println!("Python from PATH is not version {}.", python_version);
                println!("Detected version: {}", detected_version.trim());
                println!("Could not create virtual environment '{}'.", venv_name);
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Linux-specific logic
        use std::process::Command;
        let mut command = Command::new("bash");
        command.args(&["-c", &format!("python{} -m venv {}", python_version, venv_name)]);
        let output = match command.output() {
            Ok(output) => output,
            Err(error) => {
                println!("Could not execute the Python command: {error}");
                return;
            }
        };
        if output.status.success() {
            println!("Virtual environment '{}' created successfully.", venv_name);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Could not create virtual environment '{}'.", venv_name);
            println!("Python command error: {}", error.trim());
        }
    }
}
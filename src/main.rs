use colored::Colorize;

mod utilidades;

fn main() {
    println!("Welcome to this automatizator to install and configure a environment for a Python project!");
    println!("This program will help you to install and configure a virtual environment for your Python project and adding the review requirements.");
    println!("Are you ready to start? (y/n)");

    let  mut proceed_confirmation = String::new();
    std::io::stdin().read_line(&mut proceed_confirmation).unwrap();

    if proceed_confirmation.trim() == "y" {
        println!("Starting the setup process...");
        utilidades::review_python_installation();
        println!("Ok! Now, let's review and configure the Python virtual environment.");
        utilidades::review_pyenv_installation();
        println!("Setup process completed successfully!");
        println!("You can now use the virtual environment for your Python project.");
        println!("{}", "If you already have the Python version you want, you can select it when creating the virtual environment. If it is not installed, the program will try to install it.".yellow());
        println!("\n\nDo you want to create a virtual environment for your project now? (y/n)");
        let mut create_venv_confirmation = String::new();
        std::io::stdin().read_line(&mut create_venv_confirmation).unwrap();
        if create_venv_confirmation.trim() == "y" {
            println!("Ok! What is the name of the virtual environment you want to create? (For example, 'myenv')");
            let mut venv_name = String::new();
            std::io::stdin().read_line(&mut venv_name).unwrap();
            println!("Perfect! Now What is the version of Python you want to use for the virtual environment? (For example, '3.11')");
            let mut python_version = String::new();
            std::io::stdin().read_line(&mut python_version).unwrap();
            println!("Finally, just to confirm, do you have that version? Or would you like us to install it for you? (y/n)");
            let mut install_version_confirmation = String::new();
            std::io::stdin().read_line(&mut install_version_confirmation).unwrap();
            let installed = if install_version_confirmation.trim() == "y" {
                utilidades::install_python_version(python_version.trim())
            } else {
                true
            };
            if !installed {
                println!("The requested Python version was not installed.");
                println!("The virtual environment was not created.");
                return;
            }
            if install_version_confirmation.trim() == "y" {
                println!("{}", "Perfect! Installed the requested Python version.".green().bold());
            } else {
                println!("Using the Python version already installed on the computer.");
            }
            println!("Perfect! Creating virtual environment '{}' with Python version '{}'...", venv_name.trim(), python_version.trim());
            println!("Creating virtual environment...");
            utilidades::create_virtual_environment(venv_name.trim(), python_version.trim());
        } else {
            println!("You can create the virtual environment later using the command 'py -m venv <env_name>'");
        }
    } else {
        println!("Setup process aborted.");
        return;
    }
}

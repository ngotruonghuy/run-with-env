use std::env;
use std::process::Command;

fn main() {
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Warning: Could not load .env file: {}", e);
    } else {
        println!("Successfully loaded .env file (if present).");
    }
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!(
            "Usage: {} <command> [args...]",
            env::current_exe().unwrap().display()
        );
        eprintln!(
            "Example: {} npm run dev",
            env::current_exe().unwrap().display()
        );
        eprintln!(
            "Example: {} echo $MY_VAR",
            env::current_exe().unwrap().display()
        );
        std::process::exit(1); // Exit with an error code
    }

    let command_name = &args[0];
    let command_args = &args[1..];

    println!(
        "Executing command: '{}' with arguments: {:?}",
        command_name, command_args
    );

    let mut child = match Command::new(command_name).args(command_args).spawn() {
        Ok(child) => child,
        Err(e) => {
            #[cfg(windows)]
            {
                // Try with .cmd extension if not found and on Windows
                let cmd_name = format!("{}.cmd", command_name);
                match Command::new(&cmd_name).args(command_args).spawn() {
                    Ok(child) => child,
                    Err(_) => {
                        eprintln!("Failed to execute command '{}': {}\nAlso tried '{}'.\nIs it installed and in your PATH?", command_name, e, cmd_name);
                        std::process::exit(127);
                    }
                }
            }
            #[cfg(not(windows))]
            {
                eprintln!(
                    "Failed to execute command '{}': {}\nIs it installed and in your PATH?",
                    command_name, e
                );
                std::process::exit(127);
            }
        }
    };

    let status = child.wait().expect("Failed to wait on child process");

    if status.success() {
        println!("\nCommand executed successfully.");
    } else {
        eprintln!("\nCommand failed with exit status: {:?}", status.code());
        std::process::exit(status.code().unwrap_or(1)); // Exit with the child's exit code, or 1 if none
    }
}

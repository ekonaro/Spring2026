use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),         
    Display(String),        
    Create(String, String), 
    Remove(String),       
    Pwd,                  
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let result = Command::new("ls").arg(&path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("\nDirectory contents:");
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!(
                            "Failed to list directory: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Error executing ls: {}", e),
            }
        }

        FileOperation::Display(path) => {
            let result = Command::new("cat").arg(&path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("\nFile contents:");
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!(
                            "Failed to display file: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Error executing cat: {}", e),
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            let result = Command::new("sh").arg("-c").arg(&command).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("File '{}' created successfully.", path);
                    } else {
                        eprintln!(
                            "Failed to create file: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Error executing create command: {}", e),
            }
        }

        FileOperation::Remove(path) => {
            let result = Command::new("rm").arg(&path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("File '{}' removed successfully.", path);
                    } else {
                        eprintln!(
                            "Failed to remove file: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Error executing rm: {}", e),
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd").output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("Current working directory: ");
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!(
                            "Failed to get working directory: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!("Error executing pwd: {}", e),
            }
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "1" => {
                let path = read_input("Enter directory path: ");
                Some(FileOperation::List(path))
            }
            "2" => {
                let path = read_input("Enter file path: ");
                Some(FileOperation::Display(path))
            }
            "3" => {
                let path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                Some(FileOperation::Create(path, content))
            }
            "4" => {
                let path = read_input("Enter file path: ");
                Some(FileOperation::Remove(path))
            }
            "5" => Some(FileOperation::Pwd),
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please enter a number from 0 to 5.");
                None
            }
        };

        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}
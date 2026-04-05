use crate::memory::logs::{log_message, read_log};

mod memory;
mod types;
fn main() -> std::io::Result<()> {
    println!("AI Assistant starting...");

    let dirs = vec![
        "memory/logs",
        "memory/daily",
        "memory/entities",
        "memory/system",
        "memory/tasks"
    ];

    for dir in dirs {
        std::fs::create_dir_all(dir)?;
    }

    let mut input = String::new();

    while input.trim() != "exit" {
        input.clear();
        println!("Enter message (type 'exit' to exit):");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let ai_message = String::from("I heard you say: ") + input.trim();
        println!("{}", ai_message);

        log_message(input.trim(), "user", "user_message")?;
        log_message(&ai_message, "assistant", "assistant_message")?;
    }

    let events = read_log(None)?;
    println!("Loaded {} events from today.", events.len());

    Ok(())
}

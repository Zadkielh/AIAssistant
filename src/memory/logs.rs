use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::types::LogEvent;

pub fn read_log(path: Option<&DateTime<Utc>>) -> std::io::Result<Vec<LogEvent>> {
    let default = Utc::now();
    let path = format!("memory/logs/{}.jsonl", path.unwrap_or(&default).format("%Y-%m-%d").to_string());

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let parsed = serde_json::from_str::<LogEvent>(&line);

        let event = match parsed {
            Ok(event) => event,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse JSON: {} | line: {}", e, &line),
                ));
            }
        };

        events.push(event);
    }

    Ok(events)
}

pub fn log_message(msg: &str, sender: &str, event_type: &str) -> std::io::Result<()> {

    let event = LogEvent{
        timestamp: Utc::now(),
        event_type: event_type.to_string(),
        message: msg.to_string(),
        owner: sender.to_string()
    };
    append_event(event)?;

    Ok(())
    
}

pub fn append_event(event: LogEvent) -> std::io::Result<()> {

    // Open File
    let mut log = File::options()
        .append(true)
        .create(true)
        .open(format!("memory/logs/{}.jsonl", Utc::now().format("%Y-%m-%d").to_string()))?;

    // Convert event to JSON
    let event_json = serde_json::to_string(&event)
        .map_err(|e| std::io::Error::other(e))?;

    // Append line
    writeln!(log, "{}", event_json)?;

    Ok(())

}
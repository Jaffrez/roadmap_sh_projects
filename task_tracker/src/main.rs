use task::{TaskStatus, TaskVec};
use task_tracker::parse_args;
use tui::run_tui;

mod task;
mod tui;

fn main() {
    let args = parse_args();
    let path = match args.get_one::<String>("file") {
        Some(path) => path.to_string(),
        None => "tasks.db3".to_string(),
    };

    let mut tasks = TaskVec::from(&path);

    match args.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let desc = sub_matches.get_one::<String>("description").unwrap();
            tasks.add(name, desc);
        }
        Some(("del", sub_matches)) => {
            let id = sub_matches.get_one::<u64>("id").unwrap();
            tasks.del(*id);
        }
        Some(("update", sub_matches)) => {
            let id = sub_matches.get_one::<u64>("id").unwrap();
            let name = sub_matches.get_one::<String>("name");
            let desc = sub_matches.get_one::<String>("description");
            match tasks.update(*id, name, desc) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Some(("mark", sub_matches)) => {
            let id = sub_matches.get_one::<u64>("id").unwrap();
            let status = sub_matches.get_one::<String>("state").unwrap();
            let status = TaskStatus::from_str(&status).unwrap();
            match tasks.mark(*id, status) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Some(("list", sub_matches)) => {
            let status = sub_matches.get_one::<String>("state").unwrap();
            let status = TaskStatus::from_str(&status);
            let result = tasks.list_by_status(status);
            for task in result {
                println!(
                    "({}) {}\n    Status: {}\n    Created on: {}\n    Updated on: {}",
                    task.id(),
                    task.name(),
                    task.status().to_string(),
                    task.created(),
                    task.updated()
                );
            }
        }
        Some(("tui", _)) => run_tui(&mut tasks, &path),
        _ => unreachable!(),
    }
    tasks.to(&path);
}

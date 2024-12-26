use clap::{value_parser, Arg, ArgMatches, Command};

mod task;
mod tui;

pub fn parse_args() -> ArgMatches {
    let add_command = Command::new("add")
        .about("Add a task to the list")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("description").required(true));
    let del_command = Command::new("del")
        .about("Remove a task from the list")
        .arg(
            Arg::new("id")
                .required(true)
                .value_parser(value_parser!(u64)),
        );
    let update_command = Command::new("update")
        .about("Update a task in the list")
        .arg(
            Arg::new("id")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        // * name和description参数至少需要一个
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .required_unless_present("description"),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .required_unless_present("name"),
        );
    let mark_command = Command::new("mark")
        .about("Mark a task")
        .arg(
            Arg::new("id")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(Arg::new("state").required(true).value_parser([
            "todo",
            "skip",
            "in_progress",
            "done",
        ]));
    let list_command = Command::new("list")
        .about("List tasks by status")
        .arg(Arg::new("state").value_parser(["todo", "skip", "in_progress", "done"]));
    let tui_command = Command::new("tui").about("Start TUI");

    Command::new("task-cli")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("path")
                .help("Customize the database path."),
        )
        .subcommand(add_command)
        .subcommand(del_command)
        .subcommand(update_command)
        .subcommand(mark_command)
        .subcommand(list_command)
        .subcommand(tui_command)
        .get_matches()
}

#[cfg(test)]
mod test;

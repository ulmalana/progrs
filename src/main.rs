use clap::{arg, command, value_parser, Command};

mod base;
mod sub;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new todo work")
                .arg(arg!([TODO_NAME])), // .value_parser(value_parser!(String))),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit your todo work based on their ID")
                .arg(arg!([TODO_ID]).value_parser(value_parser!(u32))),
        )
        .subcommand(Command::new("list").about("Show all your todo works"))
        .subcommand(
            Command::new("show")
                .about("Show one of your todo works based on ID")
                .arg(arg!([TODO_ID]).value_parser(value_parser!(u32))),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete your todo work based on ID (DELETING YOUR TODO IS NOT RECOMMENDED)")
                .arg(arg!([TODO_ID]).value_parser(value_parser!(u32))),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let todo_name = sub_matches.get_one::<String>("TODO_NAME");
            match todo_name {
                Some(name) => sub::add::add(name),
                None => println!("No todo name."),
            }
        }
        Some(("edit", edit_matches)) => {
            let todo_id = edit_matches.get_one::<u32>("TODO_ID");
            match todo_id {
                Some(id) => sub::edit::edit(id),
                None => println!("No ID supplied"),
            }
        }
        Some(("list", _)) => sub::list::list(),
        Some(("show", show_match)) => {
            let todo_id = show_match.get_one::<u32>("TODO_ID");
            match todo_id {
                Some(id) => sub::list::show(id),
                None => println!("No ID supplied"),
            }
        }
        Some(("delete", delete_match)) => {
            let todo_id = delete_match.get_one::<u32>("TODO_ID");
            match todo_id {
                Some(id) => sub::delete::delete(id),
                None => println!("No ID supplied for deletion"),
            }
        }
        _ => unreachable!("still not implemented"),
    }
}

use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new todo work")
                .arg(arg!([TODO_NAME])),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit your todo work based on their ID")
                .arg(arg!([TODO_ID])),
        )
        .subcommand(Command::new("list").about("Show all your todo works"))
        .subcommand(
            Command::new("delete")
                .about("Delete your todo work based on ID (DELETING YOUR TODO IS NOT RECOMMENDED)")
                .arg(arg!([TODO_ID])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'progrs add' was used, TODO_NAME is: {:?}",
            sub_matches.get_one::<String>("TODO_NAME")
        ),
        Some(("edit", edit_matches)) => println!(
            "'progrs edit' was used, TODO_ID is : {:?}",
            edit_matches.get_one::<String>("TODO_ID")
        ),
        Some(("list", _)) => println!("Here are your todo list: ...."),
        Some(("delete", delete_match)) => println!(
            "'progrs delete' was used, TODO_ID is: {:?}",
            delete_match.get_one::<String>("TODO_ID")
        ),
        _ => unreachable!("still not implemented"),
    }
}

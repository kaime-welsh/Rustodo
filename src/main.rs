mod todo;

fn help() {
    println!(
        "\
A simple todo management app in the terminal.

Usage: todo [SUBCOMMAND] [VALUE]

Subcommands:
    add,    -a, --add    '<DESCRIPTION>'       add a new todo item
    edit,   -e, --edit   <ID> '<DESCRIPTION>'  edit an existing todo item
    finish, -f, --finish <ID>                  mark an item as finished
    remove, -r, --remove <ID>                  remove an item
    clean,  -c, --clean                        clean the list of finished items
    help,   -h, --help                         print help"
    );
}

fn main() {
    let mut list = todo::TodoList::new();
    list.read();

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        list.print();
    } else {
        match args[1].as_str() {
            "add" | "-a" | "--add" => list.add(&args[2].as_str()),
            "finish" | "-f" | "--finish" => list.finish(args[2].parse().unwrap()),
            "edit" | "-e" | "--edit" => list.edit(args[2].parse().unwrap(), args[3].as_str()),
            "remove" | "-r" | "--remove" => list.remove(args[2].parse().unwrap()),
            "clean" | "-c" | "--clean" => list.clean(),
            "help" | "-h" | "--help" => help(),
            _ => {
                println!("\x1b[31;1mInvalid command!\x1b[0m");
                help();
            }
        }
    }
}

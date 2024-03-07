use std::fs;

use argh::FromArgs;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement};
use serde_derive::{Deserialize, Serialize};

// TODO:
// Add arguments and parsing

#[derive(FromArgs)]
/// create and manage todo lists from the terminal
struct Args {
    #[argh(option, short = 'a')]
    /// add new todo
    add: Option<String>,

    #[argh(option, short = 'f')]
    /// mark a todo as complete
    finish: Option<usize>,

    #[argh(switch, short = 'c')]
    /// clean finished todo's from list
    clean: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    description: String,
    status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        if !std::path::Path::new("list.json").exists() {
            fs::write("list.json", b"").expect("failed to create new file");
        }
        Self { items: Vec::new() }
    }

    fn add(&mut self, desc: &str) {
        self.items.push(TodoItem {
            description: desc.to_string(),
            status: false,
        });
    }

    fn finish(&mut self, index: usize) {
        self.items[index].status = true;
    }

    fn clean(&mut self) {
        self.items.retain_mut(|i| !i.status);
    }

    fn write(&self) {
        let j = serde_json::to_string_pretty(self).unwrap();
        fs::write("list.json", j.as_bytes()).unwrap();
    }

    fn read(&mut self) {
        let data = fs::read_to_string("list.json").unwrap();
        if !data.is_empty() {
            self.items = serde_json::from_str::<TodoList>(&data).unwrap().items;
        }
    }

    fn print(&self) {
        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec!["#", "☐", "Description"]);

        for (index, item) in self.items.iter().enumerate() {
            let idx = Cell::new(index.to_string()).fg(Color::Yellow);
            let description = match item.status {
                true => Cell::new(item.description.to_string())
                    .add_attribute(comfy_table::Attribute::CrossedOut),
                false => Cell::new(item.description.to_string()),
            };
            let status = match item.status {
                true => Cell::new("☑").fg(Color::Green),
                false => Cell::new("☐"),
            };

            table.add_row(vec![idx, status, description]);
        }

        if self.items.is_empty() {
            table.add_row(vec!["", "", "No more items to do!"]);
        }

        println!("{table}");
    }
}

fn main() {
    let mut list = TodoList::new();
    list.read();

    let args: Args = argh::from_env();

    if args.clean {
        list.clean();
    } else if args.add.is_some() {
        list.add(&args.add.unwrap());
    } else if args.finish.is_some() {
        list.finish(args.finish.unwrap());
    }

    list.write();
    // println!("{}", serde_json::to_string_pretty(&list).unwrap());
    list.print();
}

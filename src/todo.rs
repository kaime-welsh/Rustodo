use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    description: String,
    status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        if !std::path::Path::new("list.json").exists() {
            fs::write("list.json", b"").expect("failed to create new file");
        }
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, desc: &str) {
        self.items.push(TodoItem {
            description: desc.to_string(),
            status: false,
        });
        self.write();
        self.print();
    }

    pub fn edit(&mut self, index: usize, desc: &str) {
        self.items[index].description = desc.to_string();
        self.write();
        self.print();
    }

    pub fn finish(&mut self, index: usize) {
        self.items[index].status = true;
        self.write();
        self.print();
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
        self.write();
        self.print();
    }

    pub fn clean(&mut self) {
        self.items.retain_mut(|i| !i.status);
        self.write();
        self.print();
    }

    pub fn write(&self) {
        let j = serde_json::to_string_pretty(self).unwrap();
        fs::write("list.json", j.as_bytes()).unwrap();
    }

    pub fn read(&mut self) {
        let data = fs::read_to_string("list.json").unwrap();
        if !data.is_empty() {
            self.items = serde_json::from_str::<TodoList>(&data).unwrap().items;
        }
    }

    pub fn print(&self) {
        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .set_width(80)
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

use terminal_size::{terminal_size, Height, Width};

mod todo;
mod file;
mod ui;

use todo::*;
use file::*;
use ui::*;

fn main() {
    let terminal = terminal_size();
    let width: u8;
    let height: u8;

    if let Some((Width(w), Height(h))) = terminal {
        width = w.try_into().unwrap_or(80);
        height = h.try_into().unwrap_or(24);
    } else {
        println!("Terminal error.");
        width = 60;
        height = 15;
    }

    let mut my_list = List::new();
    
    my_list.add("System update", "Don't forget to update.");
    my_list.add("Learn Rust", "Make specific project to imporve.");
    my_list.add("Merge code", "Make a file with whole codebase.");
    my_list.add("Nested", "This task will be deleted.");

    my_list.change_title(0, "[NOW] Update system");
    my_list.change_description(1, "Learn the ownership and borrowing.");

    my_list.done(1);
    my_list.done(2);

    my_list.remove(3);

    let folder = FolderType::new("my_showcase_tasks");
    
    folder.add_file("ok_file", "json", "{\"status\": \"success\"}");
    folder.add_file("temp_file", "txt", "We testing remove file function.");

    if let Some(read_data) = folder.read_file("ok_file") {
    }

    folder.remove_file("temp_file");
    print!("\x1B[2J\x1B[1;1H");

    let main_ui = UI::new(width, height, 0, 0);
    let cloned_ui = UI::clone(&main_ui);

    let frame = Frame::new(
        "=", "|", 
        "green", "green", 
        "black", "black", "black"
    );

    let dummy_list = List::new(); 
    let widget = ListWidget::new(
        dummy_list, 
        "Showcase Liste", 
        0, 
        "", 
        "", 
        false
    );
    widget.draw(cloned_ui, frame, my_list, "white", "green");

    print!("\x1B[{};{}H\n", height + 4, 0);
    println!("This was the showcase. This main function made by ai but other files not.");
}

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
        println!("Terminal boyutu hesaplanamadı, varsayılan değerler kullanılıyor.");
        width = 60;
        height = 15;
    }

    let mut my_list = List::new();
    
    my_list.add("Sistemi Guncelle", "Paketleri guncellemeyi unutma.");
    my_list.add("Rust Calis", "Ownership kurallarini tekrar et.");
    my_list.add("Kod Birlestir", "Modulleri tek dosyada topla.");
    my_list.add("Gereksiz Gorev", "Bu gorev birazdan silinecek.");

    my_list.change_title(0, "[GUNCEL] Sistemi Guncelle");
    my_list.change_description(1, "Ownership ve Borrowing kurallarini tekrar et.");

    my_list.done(1);
    my_list.done(2);

    my_list.remove(3);

    let folder = FolderType::new("my_showcase_tasks");
    
    folder.add_file("kalici_dosya", "json", "{\"status\": \"basarili\"}");
    folder.add_file("gecici_dosya", "txt", "Bu dosya remove_file fonksiyonunu test etmek icin.");

    if let Some(read_data) = folder.read_file("kalici_dosya") {
    }

    folder.remove_file("gecici_dosya");
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
    println!("Showcase tamamlandi! Tum modul fonksiyonlari basariyla calistirildi.");
}

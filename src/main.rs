use terminal_size::{terminal_size, Height, Width};

mod todo;
mod file;
mod ui;

use todo::*;
use file::*;
use ui::*;

fn main() {
    // ==========================================
    // 0. TERMİNAL BOYUTLARINI HESAPLAMA
    // ==========================================
    let terminal = terminal_size();
    let width: u8;
    let height: u8;

    if let Some((Width(w), Height(h))) = terminal {
        // Ekrandan taşmaması için değerleri güvenli bir şekilde küçültüyoruz (padding)
        width = w.try_into().unwrap_or(80);
        height = h.try_into().unwrap_or(24);
    } else {
        println!("Terminal boyutu hesaplanamadı, varsayılan değerler kullanılıyor.");
        width = 60;
        height = 15;
    }

    // ==========================================
    // 1. TODO MODÜLÜ SHOWCASE (Tüm fonksiyonlar)
    // ==========================================
    let mut my_list = List::new();
    
    // add(): Görev ekleme
    my_list.add("Sistemi Guncelle", "Paketleri guncellemeyi unutma.");
    my_list.add("Rust Calis", "Ownership kurallarini tekrar et.");
    my_list.add("Kod Birlestir", "Modulleri tek dosyada topla.");
    my_list.add("Gereksiz Gorev", "Bu gorev birazdan silinecek.");

    // change_title() & change_description(): Düzenleme işlemleri
    my_list.change_title(0, "[GUNCEL] Sistemi Guncelle");
    my_list.change_description(1, "Ownership ve Borrowing kurallarini tekrar et.");

    // done(): Görevi tamamlandı olarak işaretleme
    my_list.done(1); // 2. görevi tamamla
    my_list.done(2); // 3. görevi tamamla

    // remove(): Görev silme (ID'si 3 olan 'Gereksiz Gorev'i siliyoruz)
    my_list.remove(3);

    // list(): Kendi yazdığın listeleme fonksiyonu (Ham çıktı olarak UI'dan bağımsız basar)
    // println!("--- TODO LIST HAM CIKTISI ---");
    // my_list.list(); 

    // ==========================================
    // 2. FILE MODÜLÜ SHOWCASE (Tüm fonksiyonlar)
    // ==========================================
    // new(): Klasör oluşturma
    let folder = FolderType::new("my_showcase_tasks");
    
    // add_file(): Dosya oluşturma (Hem kalıcı hem de silinecek bir dosya ekleyelim)
    folder.add_file("kalici_dosya", "json", "{\"status\": \"basarili\"}");
    folder.add_file("gecici_dosya", "txt", "Bu dosya remove_file fonksiyonunu test etmek icin.");

    // read_file(): Dosya okuma testi
    if let Some(read_data) = folder.read_file("kalici_dosya") {
        // Okunan veriyi konsola basabiliriz ancak UI'ı bozmaması için loglamayı geçiyoruz
        // println!("Dosya okundu, uzantisi: {}", read_data.extension);
    }

    // remove_file(): Dosya silme testi
    folder.remove_file("gecici_dosya");

    // ==========================================
    // 3. UI MODÜLÜ SHOWCASE (Tüm fonksiyonlar)
    // ==========================================
    // Arayüz çizilmeden hemen önce ekranı temizle
    print!("\x1B[2J\x1B[1;1H");

    // UI::new(): UI nesnesi oluşturma (terminal genişliğine göre dinamik)
    let main_ui = UI::new(width, height, 0, 0);

    // UI::clone(): Clone fonksiyonunu test edelim
    let cloned_ui = UI::clone(&main_ui);

    // Frame::new(): Çerçeve tasarımı
    let frame = Frame::new(
        "=", "|", 
        "green", "green", 
        "black", "black", "black"
    );

    // ListWidget::new(): Kendi struct'ının gereksinimlerine göre dummy data ile init
    let dummy_list = List::new(); 
    let widget = ListWidget::new(
        dummy_list, 
        "Showcase Liste", 
        0, 
        "", 
        "", 
        false
    );

    // draw(): Klonlanmış UI, tasarlanan frame ve işlenmiş son TODO listesini ekrana çizme
    widget.draw(cloned_ui, frame, my_list, "white", "green");

    // Terminal imlecini çizilen kutunun en altına (terminal_size'dan gelen height değerine) taşı
    print!("\x1B[{};{}H\n", height + 4, 0);
    println!("Showcase tamamlandi! Tum modul fonksiyonlari basariyla calistirildi.");
}
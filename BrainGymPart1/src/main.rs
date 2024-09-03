mod project1;
mod project2;
mod project3;
mod project4;
mod project5;

use std::io::{self, Write};

fn main() {
    loop {
        println!("Hangi projeyi çalıştırmak istersiniz?");
        println!("1 - En küçük fark");
        println!("2 - Dizi boyutunu küçült");
        println!("3 - Permütasyon hesaplama");
        println!("4 - Palindrom sayı kontrolü");
        println!("5 - Aritmetik ifade değerlendirme");
        println!("0 - Çıkış");

        print!("Seçiminiz (0-5): ");
        io::stdout().flush().unwrap(); // Konsol çıktısını hemen yazdırmak için

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();

        // Kullanıcı girdisini bir sayıya dönüştür ve geçerli bir seçenek olup olmadığını kontrol et
        match secim.trim().parse::<u32>() {
            Ok(1) => {
                println!("Proje 1 - En küçük fark:");
                project1::run();
            },
            Ok(2) => {
                println!("\nProje 2 - Dizi boyutunu küçült:");
                project2::run();
            },
            Ok(3) => {
                println!("\nProje 3 - Permütasyon hesaplama:");
                project3::run();
            },
            Ok(4) => {
                println!("\nProje 4 - Palindrom sayi kontrolü:");
                project4::run();
            },
            Ok(5) => {
                println!("\nProje 5 - Aritmetik ifade değerlendirme:");
                project5::run();
            },
            Ok(0) => {
                println!("Çıkış yapılıyor...");
                break;
            },
            _ => println!("Geçersiz seçim, lütfen tekrar deneyin."),
        }

        println!("\n-------------------------\n");
    }
}

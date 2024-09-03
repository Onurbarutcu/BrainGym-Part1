fn en_kucuk_fark(mut dizi1: Vec<i32>, mut dizi2: Vec<i32>) -> i32 {

    dizi1.sort(); //dizileri sıralıyoruz..
    dizi2.sort();

    let mut i = 0;
    let mut j = 0;
    let mut en_kucuk_deger = i32::MAX;

    // İki dizi boyunca ilerlemek için iki işaretçi kullanıyoruz
    while i < dizi1.len() && j < dizi2.len() {
        let fark = (dizi1[i] - dizi2[j]).abs();

        println!("İşlem: |{} - {}| = {}", dizi1[i], dizi2[j], fark);
        
        // En küçük farkı güncelle
        if fark < en_kucuk_deger {
            en_kucuk_deger = fark;
            println!("Güncellenmiş en küçük fark bulundu: {}", en_kucuk_deger);
        }

        // Küçük olan sayıyı ilerlet
        if dizi1[i] < dizi2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    en_kucuk_deger
}

use std::io;

pub fn run() {
    let mut dizi1 = String::new();
    let mut dizi2 = String::new();

    println!("Dizi 1 elemanlarını aralarında boşluk olacak şekilde girin (örnek: 1 2 3): ");
    io::stdin().read_line(&mut dizi1).unwrap();
    println!("Dizi 2 elemanlarını aralarında boşluk olacak şekilde girin (örnek: 4 5 6): ");
    io::stdin().read_line(&mut dizi2).unwrap();

    let dizi1: Vec<i32> = dizi1.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let dizi2: Vec<i32> = dizi2.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let sonuc = en_kucuk_fark(dizi1, dizi2);
    println!("İki dizi arasındaki en küçük fark: {}", sonuc);
}

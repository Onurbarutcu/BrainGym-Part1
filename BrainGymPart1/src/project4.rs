//4. soru palindrom sayı yani verilen sozcuk ne ise terside aynı ifadeye esit olucak..
fn palindrom(sayi: i32) -> bool {
    let mut orijinal_sayi = sayi;
    let mut ters_sayi = 0;

    //sayinin birler basamagını aıp değişkene yazıyoruz tersini kontrol etmek için..

    while orijinal_sayi != 0 {
        let kalan = orijinal_sayi % 10;
        ters_sayi = ters_sayi * 10 + kalan;
        orijinal_sayi /= 10;
    }

    if ters_sayi == sayi {  // palindrom olup olmadığının kontrolü t ,f
        true
    } else {
        false
    }
}

use std::io;

pub fn run() {
    let mut sayi = String::new();

    println!("Bir sayi giriniz: ");
    io::stdin().read_line(&mut sayi).unwrap();

    let sayi: i32 = sayi.trim().parse().unwrap();

    if palindrom(sayi) {
        println!("{} bir palindrom sayidir.", sayi);
    } else {
        println!("{} bir palindrom sayi değildir.", sayi);
    }
}

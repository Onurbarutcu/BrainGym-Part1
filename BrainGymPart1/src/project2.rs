fn dizi_boyutunu_kucult(dizi: &[i32], kucultme_miktari: usize) -> &[i32] {
    let yeni_boyut = dizi.len() - kucultme_miktari;
    &dizi[..yeni_boyut]  //slice kullanıyoruz burada bellekten bir kısmı alıyor ..
                            // değiştiremiyoruz diziyi slice ile sadece verilen diziyi kullnabliyoruz..
}
use std::io;

pub fn run() {
    let mut dizi = String::new();
    let mut kucultme_miktari = String::new();

    println!("Diziyi elemanlarını aralarında boşluk olacak şekilde girin (örnek: 1 2 3): ");
    io::stdin().read_line(&mut dizi).unwrap();
    println!("Diziden kaç eleman çıkartmak istiyorsunuz? ");
    io::stdin().read_line(&mut kucultme_miktari).unwrap();

    let dizi: Vec<i32> = dizi.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let kucultme_miktari: usize = kucultme_miktari.trim().parse().unwrap();

    let sonuc = dizi_boyutunu_kucult(&dizi, kucultme_miktari);
    println!("Sonuç: {:?}", sonuc);
}


/*pub(crate) fn run() {
    let dizi = [1, 3, 15, 11, 2];
    let kucultme_miktari = 2;
    let sonuc = dizi_boyutunu_kucult(&dizi, kucultme_miktari);
    println!("sonuc {:?}", sonuc);
}
*/
//5.soru stack ile işlem önceliği ver.. veri yapıları.

use std::collections::VecDeque;
use std::io;

fn oncelik(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,  //kalan karakterler işte önceliği 0 örn parantez..
    }
}


fn islem_uygula(a: f32, b: f32, op: char) -> f32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0.0,
    }
}


fn durum_yazdir(degerler: &VecDeque<f32>, islemler: &VecDeque<char>) {
    println!("Degerler: {:?}", degerler);
    println!("Islemler: {:?}", islemler);
    println!("-_- O_o -_- O_o -_-");
}


fn ifade_degerlendir(ifade: &str) -> f32 {
    let mut degerler: VecDeque<f32> = VecDeque::new(); //değer ve operatorleriyıgın yapısı..
    let mut islemler: VecDeque<char> = VecDeque::new();
    let mut i = 0;

    while i < ifade.len() {
        let karakter = ifade.chars().nth(i).unwrap();

        if karakter == ' ' {
            i += 1;
            continue;
        }

        if karakter.is_digit(10) {  //ifadenin bir sayı basamağı olup olamdığını kontrol ediyoz sayıyı sola kaydırıp
            let mut deger = 0.0;        // bir sonraki indekse geçiyoruz..
            while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                deger = deger * 10.0 + ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32;
                i += 1;
            }

            if i < ifade.len() && ifade.chars().nth(i).unwrap() == '.' {//burada nokta ile sayı ondalıklımı kontrol ediyoz..
                i += 1;                                             //noktayı buldugumuzda indeksi 1 arttırıyoz ondalıklı yere geçiyoz
                let mut kesir = 0.1;
                while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                    deger += ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32 * kesir;
                    kesir *= 0.1;
                    i += 1;
                }
            }
            //hesaplanan sayıları deger yıgının içine atıcaz operatorlerle kullnamak üzere.
            degerler.push_back(deger);
            println!("Sayı eklendi: {}", deger);
            durum_yazdir(&degerler, &islemler);
            continue;
        }


        if karakter == '(' {
            islemler.push_back(karakter);
            println!("Açma parantezi eklendi");
            durum_yazdir(&degerler, &islemler);
        }

        else if karakter == ')' {
            while !islemler.is_empty() && *islemler.back().unwrap() != '(' {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);
                degerler.push_back(sonuc);
                println!("Parantez içi işlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.pop_back();
            println!("Kapanma parantezi işleme alındı");
            durum_yazdir(&degerler, &islemler);
        }

        else {
            while !islemler.is_empty() && oncelik(*islemler.back().unwrap()) >= oncelik(karakter) {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);
                degerler.push_back(sonuc);
                println!("İşlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.push_back(karakter);
            println!("Operatör eklendi: {}", karakter);
            durum_yazdir(&degerler, &islemler);
        }

        i += 1;
    }

    while !islemler.is_empty() {
        let val2 = degerler.pop_back().unwrap();
        let val1 = degerler.pop_back().unwrap();
        let op = islemler.pop_back().unwrap();
        let sonuc = islem_uygula(val1, val2, op);
        degerler.push_back(sonuc);
        println!("Kalan işlem: {} {} {} = {}", val1, op, val2, sonuc);
        durum_yazdir(&degerler, &islemler);
    }

    degerler.pop_back().unwrap()
}

pub fn run() {
    let mut ifade = String::new();

    println!("bir aritmetik ifadeyi girin (örnek: 2*3+5/6*3+15): ");
    io::stdin().read_line(&mut ifade).unwrap();

    let sonuc = ifade_degerlendir(&ifade.trim());
    println!("Sonuç: {}\n", sonuc);
}

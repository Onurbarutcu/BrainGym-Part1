// permutasyon sorusu bunun için faktroriyel işlemini yapmamız lazım.
//fonksiyonumu tanımlıyorum bir dönüş tipi veriyorum.  faktoriyel hesabı yapıyroum
//permutasyon işleminde bu bu fonksiyonumu kullnarak hesaplamamı yapıyorum..
fn faktoriyel(n:i32)->i32{
    let mut sonuc= 1;
    for i in 2..=n { //range aralığı verirken bu şekilde kullanıyoruz..
        sonuc = sonuc*i;
    }
    sonuc
}

fn permutasyon(n:i32 , r:i32)->i32{
    faktoriyel(n)/faktoriyel(n-r)
}

use std::io;

pub fn run() {
    let mut n = String::new();
    let mut r = String::new();

    println!("n değerini girin: ");
    io::stdin().read_line(&mut n).unwrap();
    println!("r değerini girin: ");
    io::stdin().read_line(&mut r).unwrap();

    let n: i32 = n.trim().parse().unwrap();
    let r: i32 = r.trim().parse().unwrap();

    let sonuc = permutasyon(n, r);
    println!("P: {}", sonuc);
}
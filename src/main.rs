use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Toko {
    harga: i32,
    nama_barang: String,
    jumlah: String,
    waktu: String,
    total: i32,
    total_harga_items: i32,
}

impl Toko {
    fn new(nama_barang: String, harga_barang: i32, jumlah_barang: String, waktu: String) -> Toko {
        let total_harga_items = harga_barang * jumlah_barang.parse::<i32>().unwrap_or(0);
        Toko {
            harga: harga_barang,
            nama_barang,
            jumlah: jumlah_barang,
            waktu,
            total: 0,
            total_harga_items,
        }
    }

    fn display_hasil(
        &mut self,
        data_toko: &mut HashMap<String, HashMap<String, String>>,
    ) -> &mut Toko {
        let entry = data_toko
            .entry(self.waktu.clone())
            .or_insert(HashMap::new());
        entry.insert("nama_barang".to_string(), self.nama_barang.clone());
        entry.insert("harga".to_string(), self.harga.to_string());
        entry.insert("jumlah".to_string(), self.jumlah.clone());
        self.total += self.total_harga_items;
        self
    }

    fn tampilkan_hasil(&self, data_toko: &HashMap<String, HashMap<String, String>>) {
        println!("{:?}", data_toko);
        println!("total yang harus dibayarkan : {}", self.total);
    }

    fn deletes(&mut self, a: &str, data_toko: &mut HashMap<String, HashMap<String, String>>) {
        if a.to_uppercase() == "Y" {
            println!("ok");
        } else {
            loop {
                println!("\n\nketik done jika data sudah benar");
                let mut a = String::new();
                println!("pilih data yang akan dihapus,");
                println!("{:?}", data_toko);
                for (k, _) in data_toko.iter() {
                    println!("{}", k);
                }
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                let a = a.trim();
    
                if a == "done" {
                    self.tampilkan_hasil(data_toko);
                    break;
                }
    
                if let Some(v) = data_toko.get_mut(a) {
                    if v["jumlah"].parse::<i32>().unwrap_or(0) < 2 {
                        self.total -= v["harga"].parse::<i32>().unwrap_or(0);
                        data_toko.remove(a);
                    } else {
                        let harga: i32 = v["harga"].parse().unwrap_or(0);
                        v.insert(
                            "jumlah".to_string(),
                            (v["jumlah"].parse::<i32>().unwrap_or(0) - 1).to_string(),
                        );
                        self.total -= harga;
                        self.tampilkan_hasil(data_toko);
                        break;
                    }
                }
            }
        }
    }
}

fn opening() {
    println!("OK");
    print!("Loading");

    for _ in 0..10 {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!("\nLoading complete!");
}

fn main() {
    let mut data_toko: HashMap<String, HashMap<String, String>> = HashMap::new();
    let id: Vec<&str> = vec!["1212", "1313", "1231415", "123456"];

    println!(
        r#"
 ________ ______   __    __   ______         __    __   ______   __       __  ______
/        /      \ /  |  /  | /      \       /  |  /  | /      \ /  \     /  |/      |
$$$$$$$$/$$$$$$  |$$ | /$$/ /$$$$$$  |      $$ | /$$/ /$$$$$$  |$$  \   /$$ |$$$$$$/
   $$ | $$ |  $$ |$$ |/$$/  $$ |  $$ |      $$ |/$$/  $$ |__$$ |$$$  \ /$$$ |  $$ |
   $$ | $$ |  $$ |$$  $$<   $$ |  $$ |      $$  $$<   $$    $$ |$$$$  /$$$$ |  $$ |
   $$ | $$ |  $$ |$$$$$  \  $$ |  $$ |      $$$$$  \  $$$$$$$$ |$$ $$ $$/$$ |  $$ |
   $$ | $$ \__$$ |$$ |$$  \ $$ \__$$ |      $$ |$$  \ $$ |  $$ |$$ |$$$/ $$ | _$$ |_
   $$ | $$    $$/ $$ | $$  |$$    $$/       $$ | $$  |$$ |  $$ |$$ | $/  $$ |/ $$   |
   $$/   $$$$$$/  $$/   $$/  $$$$$$/        $$/   $$/ $$/   $$/ $$/      $$/ $$$$$$/"#
    );

    let mut id_kasir = String::new();
    println!("masukan ID terlebih dahulu :");
    io::stdin().read_line(&mut id_kasir).expect("Failed to read line");

    if id.contains(&id_kasir.trim()) {
        opening();

        loop {
            let waktu = chrono::Local::now().format("%H:%M:%S").to_string();
            let mut nama_barang = String::new();
            println!("\nmasukan nama barang :");
            io::stdin().read_line(&mut nama_barang).expect("Failed to read line");
            let nama_barang = nama_barang.trim().to_string();

            if nama_barang == "done" {
                println!("\nTERIMA KASIH!\n");
                break;
            }

            let harga_barang: i32 = {
                let mut input = String::new();
                println!("masukan harga barang :");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                input.trim().parse().unwrap_or(0)
            };

            let mut jumlah_barang = String::new();
            println!("masukan jumlah barang :");
            io::stdin().read_line(&mut jumlah_barang).expect("Failed to read line");
            let jumlah_barang = jumlah_barang.trim().to_string();

            let mut test = Toko::new(nama_barang, harga_barang, jumlah_barang, waktu.clone());
            test.display_hasil(&mut data_toko);
            test.tampilkan_hasil(&data_toko);

            print!("Apakah data sudah Benar? (y/n): ");
            io::stdout().flush().unwrap();
            let mut input_data = String::new();
            io::stdin().read_line(&mut input_data).expect("Failed to read line");
            let input_data = input_data.trim();
            test.deletes(input_data, &mut data_toko);
        }
    } else {
        println!("ID salah");
    }
}

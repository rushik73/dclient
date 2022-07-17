use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let args = std::env::args();
    let mut args = args.skip(1);
    let address = args.next().unwrap();
    let mut total = 0;
    loop {
        let size = client.get(&address).send().unwrap().bytes().unwrap().len();
        total += size;
        println!("total: {}", total);
    }
}

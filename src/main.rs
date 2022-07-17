use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    loop {
        let size = client
            .get("https://www.youtube.com")
            .send()
            .unwrap()
            .bytes()
            .unwrap()
            .len();
    }
}

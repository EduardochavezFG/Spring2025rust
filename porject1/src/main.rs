use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json;
use ureq;

// Define a trait for fetching prices
trait Pricing {
    fn get_url() -> &'static str;
    fn extract_price(&self) -> f64;

    fn fetch_price() -> std::io::Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let response = ureq::get(Self::get_url())
            .call()
            .expect("HTTP request failed");
        response.into_json()
    }

    fn save_price(&self, filename: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");
        writeln!(file, "{}", self.extract_price()).expect("Failed to write to file");
    }
}

// Structs for CoinGecko response
#[derive(Deserialize, Debug)]
struct CoinGeckoPrice {
    bitcoin: Option<CryptoPrice>,
    ethereum: Option<CryptoPrice>,
}

#[derive(Deserialize, Debug)]
struct CryptoPrice {
    usd: f64,
}

// Structs for individual assets
#[derive(Deserialize, Debug)]
struct BitcoinWrapper {
    #[serde(flatten)]
    data: CoinGeckoPrice,
}

#[derive(Deserialize, Debug)]
struct EthereumWrapper {
    #[serde(flatten)]
    data: CoinGeckoPrice,
}

// Implement Pricing trait for Bitcoin
impl Pricing for BitcoinWrapper {
    fn get_url() -> &'static str {
        "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
    }
    fn extract_price(&self) -> f64 {
        self.data.bitcoin.as_ref().map(|b| b.usd).unwrap_or(0.0)
    }
}

// Implement Pricing trait for Ethereum
impl Pricing for EthereumWrapper {
    fn get_url() -> &'static str {
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd"
    }
    fn extract_price(&self) -> f64 {
        self.data.ethereum.as_ref().map(|e| e.usd).unwrap_or(0.0)
    }
}
//there were no free wbsites that we were able to use in order ot obtain the SP500

//#[derive(Deserialize, Debug)]
//struct SP500 {
//  price: f64,
//}

//impl Pricing for SP500 {
//  fn get_url() -> &'static str {
//  "https://api.example.com/sp500" 
// fn extract_price(&self) -> f64 {
//   self.price
//  }
//}

fn main() {
    loop {
        if let Ok(bitcoin) = BitcoinWrapper::fetch_price() {
            bitcoin.save_price("bitcoin.txt");
        }
        if let Ok(ethereum) = EthereumWrapper::fetch_price() {
            ethereum.save_price("ethereum.txt");
        }
        
        thread::sleep(Duration::from_secs(10));
    }
}
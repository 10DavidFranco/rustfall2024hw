use serde::Deserialize;
use ureq;
use std::time::{Duration, Instant};


#[derive(Debug)]

struct Bitcoin{
    api_address:String,
    file_name:String,
}
//do same for others
struct Etherium{
    api_address:String,
    file_name:String,
}
struct SP500{
    api_address:String,
    file_name:String,
}

pub trait Pricing{
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

#[derive(Debug, Deserialize)]
struct Cost{
    usd:i32,
}
#[derive(Debug, Deserialize)]

struct BTC{
    bitcoin:Cost,
}
#[derive(Debug, Deserialize)]
struct SP{
    sandp:Cost,
}
#[derive(Debug, Deserialize)]
struct ETH{
    ethereum:Cost,
}

impl Pricing for Bitcoin{
    fn fetch_price(&self) -> f32{
        //match statment
        //go to api
        //go to aopi lecture
        //dog lecture
        //need ureq and serde
        //STICK ONLY WITH SERDE AND UREQ LIBRARIES
        //NOTHING ELSE
        //still need to cover smart-pointers and closures
        
        let response: BTC= match ureq::get(&self.api_address).call(){
            Ok(res)=>{
                let body = match res.into_string(){
                    Ok(d) => d,
                    Err(_)=>{
                        return 0.0;
                    }
                };

                match serde_json::from_str::<BTC>(&body) {
                    Ok(data) => data, // Successfully deserialized the JSON
                    Err(_) => {
                        println!("Failed to parse JSON response");
                        return 0.0; // Return a default value in case of error
                    }
                }
            }
            Err(_) =>{
                return 0.0;
            }
        };

        response.bitcoin.usd as f32
        

      
    }
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}


impl Pricing for SP500{
    fn fetch_price(&self) -> f32{
        //match statment
        //go to api
        //go to aopi lecture
        //dog lecture
        //need ureq and serde
        //STICK ONLY WITH SERDE AND UREQ LIBRARIES
        //NOTHING ELSE
        //still need to cover smart-pointers and closures
        
        let response: SP= match ureq::get(&self.api_address).call(){
            Ok(res)=>{
                let body = match res.into_string(){
                    Ok(d) => d,
                    Err(_)=>{
                        return 0.0;
                    }
                };

                match serde_json::from_str::<SP>(&body) {
                    Ok(data) => data, // Successfully deserialized the JSON
                    Err(_) => {
                        println!("Failed to parse JSON response");
                        return 0.0; // Return a default value in case of error
                    }
                }
            }
            Err(_) =>{
                return 0.0;
            }
        };

        response.sandp.usd as f32
        

      
    }
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}

impl Pricing for Etherium{
    fn fetch_price(&self) -> f32{
        //match statment
        //go to api
        //go to aopi lecture
        //dog lecture
        //need ureq and serde
        //STICK ONLY WITH SERDE AND UREQ LIBRARIES
        //NOTHING ELSE
        //still need to cover smart-pointers and closures
        
        let response: ETH= match ureq::get(&self.api_address).call(){
            Ok(res)=>{
                let body = match res.into_string(){
                    Ok(d) => d,
                    Err(_)=>{
                        return 0.0;
                    }
                };

                match serde_json::from_str::<ETH>(&body) {
                    Ok(data) => data, // Successfully deserialized the JSON
                    Err(_) => {
                        println!("Failed to parse JSON response");
                        return 0.0; // Return a default value in case of error
                    }
                }
            }
            Err(_) =>{
                return 0.0;
            }
        };

        response.ethereum.usd as f32
        

      
    }
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}



fn main(){

    
    //This behavior needs to be implemented in fecth price
    //Deadline is Nov.21st

    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();  
    let btc_txt = "btc_prices.json".to_string();
    let b = Bitcoin{api_address:btc_api, file_name: btc_txt,};
    
    let btc_price = b.fetch_price();
        
    //let sp_api = ;
    //let sp_txt = ;
    //let s = ;
    //let sp_price =;

    let eth_api="https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let eth_txt="eth_prices.json".to_string();
    let e = Etherium{api_address:eth_api, file_name:eth_txt};
    let eth_price= e.fetch_price();
    

    let start = Instant::now();
    while start.elapsed() < Duration::new(10,0){

   
    println!("Bitcoin price: {:?}", btc_price);
    b.save_to_file();
    println!("Etherium price: {:?}", eth_price);
    e.save_to_file();

        std::thread::sleep(Duration::from_secs(1));
    }    
}

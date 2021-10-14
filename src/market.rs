// Returns the market price of a given coin pair
// TODO: Remove this market prices simulations
// TODO: Dynamically retrieve market price for wallet coins from broker API
pub fn get_pair_price(pair: &String) -> f64 {
    let mut price: f64 = 0.;

    if pair == "BTC/USDT" {
        price = 55818.12;
    } else if pair == "SOL/USDT" {
        price = 148.05;
    } else if pair == "TRX/USDT" {
        price = 0.09453;
    } else if pair == "SHIB/USDT" {
        price = 0.00002872;
    } else if pair == "BTT/USDT" {
        price = 0.003648;
    } else if pair == "MBOX/USDT" {
        price = 4.80;
    }
    
    return price;
}

// Struct for coin type
pub struct Coin {
    pub pair: String,
    pub amount: f64
}

// Retrieves the wallet pairs and amount
// TODO: Remove this wallet amounts simulations
// TODO: Dynamically retrieve values from broker API
// TODO: Handle this: You need to Fill-up only pairs with the same comparing currencies, otherwise, the total balance would be wrong
pub fn get_coins() -> Vec<Coin> {
    let mut coins: Vec<Coin> = Vec::new();

    coins.push(Coin {pair: "BTC/USDT".to_string(), amount: 0.00572839});
    coins.push(Coin {pair: "SOL/USDT".to_string(), amount: 0.23000000});
    coins.push(Coin {pair: "TRX/USDT".to_string(), amount: 3575.00000000});
    coins.push(Coin {pair: "SHIB/USDT".to_string(), amount: 8123331.00000000});
    coins.push(Coin {pair: "BTT/USDT".to_string(), amount: 35000.00000000});
    coins.push(Coin {pair: "MBOX/USDT".to_string(), amount: 83.20000000});

    return coins;
}

// Returns the value for the given coin
pub fn get_coin_value(coin: &Coin) -> f64 {
    return coin.amount * crate::market::get_pair_price(&coin.pair);
}

// Displays the wallet information with total balance
pub fn show() {
    println!("########################## Wallet ##########################\n");

    for coin in get_coins() {
        println!("Pair: {}", coin.pair);
        println!("Amount: {}", coin.amount);
        println!("Price: {}", crate::market::get_pair_price(&coin.pair));
        println!("Value: {}\n", crate::wallet::get_coin_value(&coin));
    }

    println!("###################### Total Balance ######################\n");
    println!("{} {}\n", get_balance(), dotenv!("CURRENCY"));
}

// Returns the wallet total balance
// TODO: Need to convert from coin pair to .env currency
fn get_balance() -> f64 {    
    let mut total: f64 = 0.;

    for coin in get_coins() {
        total += crate::market::get_pair_price(&coin.pair);
    }

    return total;
}

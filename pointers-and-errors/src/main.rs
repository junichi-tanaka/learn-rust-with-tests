struct Wallet {
    balance: i64,
}

impl Wallet {
    fn deposit(&mut self, amount: i64) {
        self.balance += amount
    }

    fn balance(&self) -> i64 {
        return self.balance
    }
}

fn main() {
    let mut wallet = Wallet{balance:0};
    wallet.deposit(10);
    println!("{}", wallet.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut wallet = Wallet{balance:0};
        wallet.deposit(10);
        assert_eq!(wallet.balance(), 10);
    }
}

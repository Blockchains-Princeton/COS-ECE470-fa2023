use log::info;
use std::time;
use std::thread;

#[derive(Clone)]
pub struct TransactionGenerator {

}

impl TransactionGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(self, theta: u64) {
        thread::Builder::new()
            .name("transaction-generator".to_string())
            .spawn(move || {
                self.generate_transactions(theta);
            })
            .unwrap();
        info!("Transaction generator started");
    }

    fn generate_transactions(&self, theta: u64) {
        loop {
            unimplemented!();

            if theta != 0 {
                let interval = time::Duration::from_millis(10 * theta);
                thread::sleep(interval);
            }
        }
    }
}

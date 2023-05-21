use chrono::prelude::*;
fn main() {
    let today = Utc::today();
    let start = Utc.ymd(2023,1,1);
    let days = today.signed_duration_since(start).num_days();
    if days % 4 == 0 {
        println!("Kakteen gießen!");        
    } else {
    }
}

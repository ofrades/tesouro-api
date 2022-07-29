use chrono::Utc;

fn main() {
    let today_date = Utc::now();
    println!("{}", today_date.format("%Y-%m-%d"));
}

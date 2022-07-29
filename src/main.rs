use chrono::Utc;

fn main() {
    let today_date = Utc::now();
    println!("{}", today_date.format("%Y-%m-%d"));

    let _calendar = build_calendar(today_date.format("%Y").to_string());
}

fn build_calendar(year: String) {
    println!("{}", year)
}

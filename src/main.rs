fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let settings = settings
        .try_into::<std::collections::HashMap<String, String>>()
        .unwrap();

    let port = settings.get("port").unwrap().parse::<u16>().unwrap();
    let db = settings.get("db").unwrap().as_str();

    product_delivery::rocket(port, db).launch();
}

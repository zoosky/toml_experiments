

extern crate toml;


fn main() {
    

    use toml::Value;
    let value = "foo = 'bar'".parse::<Value>().unwrap();
    assert_eq!(value["foo"].as_str(), Some("bar"));

    println!("Hello, TOML! {}", value );
}

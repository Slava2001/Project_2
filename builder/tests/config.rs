use builder::Config;

#[test]
fn single_value() {
    let mut cfg = Config::new("./tests/config.json").unwrap();
    assert_eq!(cfg.take::<String>("str").unwrap().as_str(), "Hello");

    let mut cfg = cfg.take::<Config>("cfg").unwrap();
    assert_eq!(cfg.take::<String>("str").unwrap().as_str(), "World");

    let mut cfg = cfg.take::<Config>("include").unwrap();
    assert_eq!(cfg.take::<String>("str").unwrap().as_str(), "Included config");

    let mut cfg = cfg.take::<Vec<Config>>("cfg_arr").unwrap();
    assert_eq!(cfg[1].take::<String>("str").unwrap().as_str(), "one");

    let mut cfg = cfg[0].take::<Config>("include").unwrap();
    assert_eq!(cfg.take::<String>("str").unwrap().as_str(), "Hello");
}

pub struct Config {
  message: String,
}

impl Config {
  pub fn new(msg: String) -> Config {
    Config { message: msg }
  }
}

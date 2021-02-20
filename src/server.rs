pub struct Server {
    addr: String,
  }
  
impl Server {
  // Constructor
  pub fn new(addr: String) -> Self {
      Self { addr }
  }

  // Methods take first param referencing itself
  pub fn run(self) {
      println!("Listening on {}", self.addr);
  }
}
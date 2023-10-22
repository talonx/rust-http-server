pub struct Server {
    addr: String,
    port: u16,
}

impl Server {
    pub fn new(addr: String, port: u16) -> Self {
        Self { addr, port }
    }

    pub fn run(&self) {
        println!("Listening on {}:{}", self.addr, self.port);
    }
}

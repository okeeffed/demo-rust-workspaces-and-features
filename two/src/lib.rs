#[cfg(feature = "example")]
pub mod example {
    pub fn hello() {
        println!("Hello from the feature!");
    }
}

pub mod two {
    pub fn hello() {
        println!("Hello from two!");
    }
}

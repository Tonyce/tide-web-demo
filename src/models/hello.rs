pub struct Hello;

#[cfg_attr(test, mocktopus::macros::mockable)]
impl Hello {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello() -> String {
        String::from("hello")
    }
}

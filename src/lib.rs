mod echoer;
mod hop;

pub use echoer::*;
pub use hop::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

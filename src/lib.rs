mod error;
mod pb;
mod storage;
mod service;
pub use pb::abi::*;
pub use error::KvError;
pub use storage::Storage;
#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use crate::{CommandResponse, Storage};

pub mod command_service;
pub trait CommandService {
   
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

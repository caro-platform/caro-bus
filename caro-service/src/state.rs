use serde::Serialize;

use caro_bus_common::errors::Error as BusError;
use caro_bus_lib::{state::State as BusState, Result as BusResult};

pub struct State<T: Serialize> {
    internal: Option<BusState<T>>,
}

impl<T: Serialize + 'static> State<T> {
    pub fn new() -> Self {
        Self { internal: None }
    }

    pub fn register(&mut self, state_name: &str, initial_value: T) -> BusResult<()> {
        if self.internal.is_some() {
            return Err(Box::new(BusError::AlreadyRegistered));
        }

        match *crate::service::SERVICE_BUS.lock().unwrap() {
            Some(ref mut bus) => {
                self.internal = Some(bus.register_state(state_name, initial_value)?)
            }
            _ => panic!("Not registered"),
        }

        Ok(())
    }

    pub fn set(&mut self, value: T) {
        match self.internal {
            None => panic!("Not registered"),
            Some(ref mut internal) => internal.set(value),
        }
    }

    pub fn get(&self) -> &T {
        match self.internal {
            None => panic!("Not registered"),
            Some(ref internal) => internal.get(),
        }
    }
}
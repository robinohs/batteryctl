use eyre::Result;

use crate::features::Features;

pub trait Command {
    fn execute(&mut self, features: Features) -> Result<()>;
}

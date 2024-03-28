use crate::cli::{Example, TargetType};
use crate::{DynError};

impl Example {
    pub(crate) fn run(self) -> Result<(), DynError> {
        match self.target {
            TargetType::Msvc => self.run_msvc()?,
            TargetType::Web => self.run_web()?,
        }

        Ok(())
    }
}

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RNGProgramError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("arithmetic error")]
  ArithmeticError,


}

impl From<RNGProgramError> for ProgramError {
  fn from(e: RNGProgramError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

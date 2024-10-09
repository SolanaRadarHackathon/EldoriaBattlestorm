use crate::{error::RNGProgramError::InvalidInstruction, state::{Attack, Character}};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction {
  InitializeGame,
  JoinGame,
  Play{data:Attack},
  ClaimVic,
  ClaimVicTime,
  Upgrade{data:Character},
  Register,

}

impl RNGProgramInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    Ok(match tag {
      0 => Self::InitializeGame,
      1 => Self::JoinGame,
      2 => Self::Play{
        data:Attack::try_from_slice(&rest)?
      },
      3 => Self::ClaimVic,
      4 => Self::ClaimVicTime,
      5 => Self::Upgrade{
        data:Character::try_from_slice(&rest)?
      },
      6 => Self::Register,


      _ => return Err(InvalidInstruction.into()),
    })
  }
}


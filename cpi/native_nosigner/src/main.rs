use borsch::BorschDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

// Declare program entrypoint
entrypoint!(process_instruction);

// Define program instructions
#[derive(BorschDeserialize)]
enum ProgramInstruction {
    SolTransfer {amount: u64},
}

impl ProgramInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo].
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize instruction data
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    // Process instruction
    match instruction {
        // Parse account
        let [sender_info, receipient_info, system_program_info] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Verify the sender is a signer
        if !sender_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Create and invoke the transfer instruction
        let transfer_ix = system_instruction::transfer(
            sender_info.key,
            receipient_info.key,
            amount,
        );

        invoke(
            &transfer_ix,
            &[
                sender_info.clone(),
                receipient_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        Ok(())
    }
}
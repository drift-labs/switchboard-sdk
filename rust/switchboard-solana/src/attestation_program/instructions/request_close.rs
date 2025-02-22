use crate::prelude::*;

#[derive(Accounts)]
#[instruction(params:FunctionRequestCloseParams)]
pub struct FunctionRequestClose<'info> {
    #[account(mut)]
    pub request: AccountInfo<'info>,
    /// CHECK: Only needs to sign if request.garbage_collection_slot has not elapsed
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub escrow: AccountInfo<'info>,
    /// CHECK: we need to load_mut and remove_request
    #[account(mut)]
    pub function: AccountInfo<'info>,
    /// CHECK: allow partial funds to be sent to the claimer only if request.garbage_collection_slot has elapsed
    pub sol_dest: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_dest: AccountInfo<'info>,
    pub state: AccountInfo<'info>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(address = solana_program::system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct FunctionRequestCloseParams {}

impl InstructionData for FunctionRequestCloseParams {}

impl Discriminator for FunctionRequestCloseParams {
    const DISCRIMINATOR: [u8; 8] = [5, 221, 34, 111, 136, 82, 119, 101];
}

impl Discriminator for FunctionRequestClose<'_> {
    const DISCRIMINATOR: [u8; 8] = [5, 221, 34, 111, 136, 82, 119, 101];
}

impl<'info> FunctionRequestClose<'info> {
    pub fn get_instruction(&self, program_id: Pubkey) -> anchor_lang::Result<Instruction> {
        let accounts = self.to_account_metas(None);

        let mut data: Vec<u8> = FunctionRequestClose::discriminator().try_to_vec()?;
        let params = FunctionRequestCloseParams {};
        data.append(&mut params.try_to_vec()?);

        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
        Ok(instruction)
    }

    pub fn invoke(&self, program: AccountInfo<'info>) -> ProgramResult {
        let instruction = self.get_instruction(*program.key)?;
        let account_infos = self.to_account_infos();

        invoke(&instruction, &account_infos[..])
    }

    pub fn invoke_signed(
        &self,
        program: AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let instruction = self.get_instruction(*program.key)?;
        let account_infos = self.to_account_infos();

        invoke_signed(&instruction, &account_infos[..], signer_seeds)
    }

    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        let mut account_infos = Vec::new();
        account_infos.extend(self.request.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.escrow.to_account_infos());
        account_infos.extend(self.function.to_account_infos());
        account_infos.extend(self.sol_dest.to_account_infos());
        account_infos.extend(self.escrow_dest.to_account_infos());
        account_infos.extend(self.state.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }

    #[allow(unused_variables)]
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        account_metas.extend(self.request.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas.extend(self.escrow.to_account_metas(None));
        account_metas.extend(self.function.to_account_metas(None));
        account_metas.extend(self.sol_dest.to_account_metas(None));
        account_metas.extend(self.escrow_dest.to_account_metas(None));
        account_metas.extend(self.state.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}

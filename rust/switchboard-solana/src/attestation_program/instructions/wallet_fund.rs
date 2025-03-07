use crate::prelude::*;

#[derive(Accounts)]
#[instruction(params:WalletFundParams)]
pub struct WalletFund<'info> {
    #[account(mut)]
    pub wallet: AccountInfo<'info>, // SwitchboardWallet
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: AccountInfo<'info>, // TokenMint
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub attestation_queue: AccountInfo<'info>, // AttestationQueueAccountData
    #[account(mut)]
    pub token_wallet: AccountInfo<'info>, // TokenAccount
    #[account(mut)]
    pub funder_wallet: Option<AccountInfo<'info>>, // TokenAccount
    #[account(signer)]
    pub funder: AccountInfo<'info>, // Signer
    pub state: AccountInfo<'info>,             // AttestationProgramState
    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(address = solana_program::system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WalletFundParams {
    pub transfer_amount: Option<u64>,
    pub wrap_amount: Option<u64>,
}

impl InstructionData for WalletFundParams {}

impl Discriminator for WalletFundParams {
    const DISCRIMINATOR: [u8; 8] = [93, 170, 44, 19, 223, 172, 40, 164];
}

impl Discriminator for WalletFund<'_> {
    const DISCRIMINATOR: [u8; 8] = [93, 170, 44, 19, 223, 172, 40, 164];
}

impl<'info> WalletFund<'info> {
    pub fn get_instruction(
        &self,
        program_id: Pubkey,
        params: &WalletFundParams,
    ) -> anchor_lang::Result<Instruction> {
        let accounts = self.to_account_metas(None);

        let mut data: Vec<u8> = WalletFund::discriminator().try_to_vec()?;
        data.append(&mut params.try_to_vec()?);

        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
        Ok(instruction)
    }

    pub fn invoke(&self, program: AccountInfo<'info>, params: &WalletFundParams) -> ProgramResult {
        let instruction = self.get_instruction(*program.key, params)?;
        let account_infos = self.to_account_infos();

        invoke(&instruction, &account_infos[..])
    }

    pub fn invoke_signed(
        &self,
        program: AccountInfo<'info>,
        params: &WalletFundParams,
        signer_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let instruction = self.get_instruction(*program.key, params)?;
        let account_infos = self.to_account_infos();

        invoke_signed(&instruction, &account_infos[..], signer_seeds)
    }

    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        let mut account_infos = Vec::new();
        account_infos.extend(self.wallet.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.attestation_queue.to_account_infos());
        account_infos.extend(self.token_wallet.to_account_infos());
        account_infos.extend(self.funder_wallet.to_account_infos());
        account_infos.extend(self.funder.to_account_infos());
        account_infos.extend(self.state.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }

    #[allow(unused_variables)]
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        account_metas.extend(self.wallet.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas.extend(self.attestation_queue.to_account_metas(None));
        account_metas.extend(self.token_wallet.to_account_metas(None));
        if let Some(funder_wallet) = &self.funder_wallet {
            account_metas.extend(funder_wallet.to_account_metas(None));
        } else {
            account_metas.push(AccountMeta::new_readonly(crate::ID, false));
        }
        account_metas.extend(self.funder.to_account_metas(None));
        account_metas.extend(self.state.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}

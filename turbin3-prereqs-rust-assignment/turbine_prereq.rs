use anchor_lang::prelude::*;

declare_id!("ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa");

// Accounts
#[derive(Accounts)]
pub struct Complete<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub prereq: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    /// CHECK: Skip check
    pub signer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub prereq: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

// CPI
#[cfg(all(target_os = "solana", feature="cpi"))]
pub mod cpi {
    #![allow(unused)]
    use anchor_lang::Discriminator;
    use super::*;

    pub fn complete<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Complete<'info>>,
        github: Vec<u8>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Complete { github };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Complete::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn update<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Update<'info>>,
        github: Vec<u8>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Update { github };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Update::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }  
}

// RPC
#[cfg(all(not(target_os = "solana"), feature="cpi"))]
pub mod rpc {
    #![allow(unused)]
    use anchor_lang::prelude::*;
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct Complete {
            pub signer: Pubkey,
            pub prereq: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for Complete {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.signer,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.prereq,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
    
    #[cfg_attr(not(target_os="solana"), derive(Debug))]
    #[derive(AnchorSerialize)]
    pub struct Update {
            pub signer: Pubkey,
            pub prereq: Pubkey,
            pub system_program: Pubkey,
    }
    
        impl anchor_lang::ToAccountMetas for Update {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = vec![];
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.signer,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.prereq,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                self.system_program,
                false,
            ));
                account_metas
            }
        }
}

// I11n
#[cfg(all(target_os = "solana", feature="i11n"))]
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::{instructions::*, ID};

    // Complete
    #[derive(TryFromInstruction)]
    pub struct CompleteI11n<'info> {
        pub accounts: CompleteAccountMetas<'info>,
        pub args: Complete,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Update
    #[derive(TryFromInstruction)]
    pub struct UpdateI11n<'info> {
        pub accounts: UpdateAccountMetas<'info>,
        pub args: Update,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct CompleteAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub prereq: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateAccountMetas<'info> {
        pub signer: &'info AccountMeta,
        pub prereq: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use anchor_lang::Discriminator;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Complete {
        pub github: Vec<u8>,
    }
    
    impl anchor_lang::InstructionData for Complete {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
    

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Update {
        pub github: Vec<u8>,
    }
    
    impl anchor_lang::InstructionData for Update {
        fn data(&self) -> Vec<u8> {
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap();
            data
        }
    
        fn write_to(&self, mut data: &mut Vec<u8>) {
            data.clear();
            data.extend_from_slice(&Self::DISCRIMINATOR);
            self.serialize(&mut data).unwrap()
        }
    }
            
}

// Events
#[cfg(feature="events")]
pub mod events {
    use super::*;
    use anchor_i11n::AnchorDiscriminator;
    use anchor_lang::Discriminator;


}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

    #[account]
    pub struct SolanaCohort5Account {
        pub github: Vec<u8>,
        pub key: Pubkey,
    }  
}
        
// Defined types
#[cfg_attr(not(target_os="solana"), derive(Debug))]
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SolanaCohort5Account {
    pub github: Vec<u8>,
    pub key: Pubkey,
}
use anchor_lang::prelude::*;

declare_id!("3VCvp1YPmgXYxafYx9kvCDm2DcoufXuhd8j7P7E861Qq");

#[program]
pub mod anchor_program {
    use core::num;
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, inititial_value: u32) -> Result<()> {
        ctx.accounts.account.numb = inititial_value;
        Ok(())
    }
    pub fn add(ctx: Context<Add>, num: u32) -> Result<()> {
        ctx.accounts.account.numb = ctx.accounts.account.numb + num;
        Ok(())
    }
    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.numb = ctx.accounts.account.numb * 2;
        Ok(())
    }
}

#[account]
struct DataShape {
    pub numb: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer= signer, space =8 + 4)]
    pub account: Account<'info, DataShape>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

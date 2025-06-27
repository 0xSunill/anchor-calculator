use anchor_lang::prelude::*;

declare_id!("3VCvp1YPmgXYxafYx9kvCDm2DcoufXuhd8j7P7E861Qq");

#[program]
pub mod anchor_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       
    }
}

#[derive(Accounts)]
pub struct Initialize {}

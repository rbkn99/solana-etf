use anchor_lang::prelude::*;

declare_id!("9JNb23fdzmWDt71tWveTQFwdmebP2JDQ6nGrVS63hMdz");

pub mod solana_etf {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn get_pool_info(ctx: Context<Initialize>) -> Result<()> {
        CpiContext::new(ctx.accounts)
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct GetPoolInfo<'info> {
    #[account(mut)]
    pub pool_program: Program<'info, Pool>,
}
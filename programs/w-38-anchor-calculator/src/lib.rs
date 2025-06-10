use anchor_lang::prelude::*;

declare_id!("97gikpJ46w5KNTGp6EygCUaghE1HynQyswcogKETq93w");

#[program]
pub mod anchor_cal {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_val: u32) -> Result<()> {
        ctx.accounts.account.num = init_val;

        Ok(())
    }

    pub fn double(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;

        Ok(())
    }
    pub fn half(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num / 2;

        Ok(())
    }

    pub fn add(ctx: Context<Modify>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num + num;

        Ok(())
    }

    pub fn sub(ctx: Context<Modify>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num - num;

        Ok(())
    }

    pub fn mul(ctx: Context<Modify>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * num;

        Ok(())
    }

    pub fn div(ctx: Context<Modify>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num / num;

        Ok(())
    }
}

#[account]
pub struct DataShape {
    num: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

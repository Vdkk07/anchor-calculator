use anchor_lang::prelude::*;

declare_id!("97gikpJ46w5KNTGp6EygCUaghE1HynQyswcogKETq93w");

#[program]
pub mod anchor_cal {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_val: u32) -> Result<()> {
        ctx.accounts.account.num = init_val;

        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;

        Ok(())
    }
    pub fn half(ctx: Context<Half>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num / 2;

        Ok(())
    }

    pub fn add(ctx: Context<Add>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num + num;

        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num - num;

        Ok(())
    }

    pub fn mul(ctx: Context<Mul>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * num;

        Ok(())
    }

    pub fn div(ctx: Context<Div>, num: u32) -> Result<()> {
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
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Half<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
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
pub struct Sub<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Mul<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Div<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

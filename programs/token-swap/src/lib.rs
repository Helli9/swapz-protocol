use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Nu7m2dy8oLEqZYjtU5uBEjpAfyFJTsWKzfrQvn2gnSJ");

#[program]
pub mod token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let token_a = &ctx.accounts.token_a;
        let token_b = &ctx.accounts.token_b;
        let user_a = &ctx.accounts.user_a;
        let user_b = &ctx.accounts.user_b;
        let authority = &ctx.accounts.authority;
        let token_program = &ctx.accounts.token_program;

        // Define swap ratio (1:1 for simplicity)
        let amount_b = amount;

        // Transfer token A to user B
        let cpi_accounts = Transfer {
            from: user_a.to_account_info(),
            to: token_b.to_account_info(),
            authority: authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts),
            amount,
        )?;

        Ok(())
    }
}

struct Swap<'info> {
    #[account(mut)]
    pub token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_a: Signer<'info>,
    #[account(mut)]
    pub user_b: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

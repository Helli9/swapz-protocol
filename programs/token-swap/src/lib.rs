use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("YourProgramIDHere");

#[program]
pub mod token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount_a: u64) -> Result<()> {
        let token_a = &ctx.accounts.token_a;
        let token_b = &ctx.accounts.token_b;
        let user_a = &ctx.accounts.user_a;
        let user_b = &ctx.accounts.user_b;
        let authority = &ctx.accounts.authority;
        let token_program = &ctx.accounts.token_program;

        let amount_b = amount_a 

        // Ensure the user has enough Token A to swap
        if user_a.amount < amount_a {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Transfer Token A from user A to Token B account
        let cpi_accounts = Transfer {
            from: user_a.to_account_info(),
            to: token_b.to_account_info(),
            authority: authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts),
            amount_a,
        )?;

        // Transfer Token B from user B to user A (token swap)
        let cpi_accounts = Transfer {
            from: user_b.to_account_info(),
            to: token_a.to_account_info(),
            authority: authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts),
            amount_b,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
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

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in the user A account to perform the swap.")]
    InsufficientFunds,
}

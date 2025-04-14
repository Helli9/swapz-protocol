use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("YourProgramIDHere");

#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let token_a_from = &ctx.accounts.token_a_from;
        let token_b_to = &ctx.accounts.token_b_to;
        let token_b_from = &ctx.accounts.token_b_from;
        let token_a_to = &ctx.accounts.token_a_to;
        let swapper_a = &ctx.accounts.swapper_a;
        let swapper_b = &ctx.accounts.swapper_b;
        let token_program = &ctx.accounts.token_program;

        // Ensure swapper A has enough Token A to send
        if token_a_from.amount < amount {
            return Err(ErrorCode::InsufficientFundsA.into());
        }

        // Ensure swapper B has enough Token B to send
        if token_b_from.amount < amount {
            return Err(ErrorCode::InsufficientFundsB.into());
        }

        // Transfer Token A from swapper A to swapper B
        let cpi_accounts_a_to_b = Transfer {
            from: token_a_from.to_account_info(),
            to: token_a_to.to_account_info(), // Swapper B's Token A account
            authority: swapper_a.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts_a_to_b),
            amount,
        )?;

        // Transfer Token B from swapper B to swapper A
        let cpi_accounts_b_to_a = Transfer {
            from: token_b_from.to_account_info(),
            to: token_b_to.to_account_info(), // Swapper A's Token B account
            authority: swapper_b.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts_b_to_a),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub token_a_from: Account<'info, TokenAccount>, // Swapper A's Token A account
    #[account(mut)]
    pub token_b_to: Account<'info, TokenAccount>,   // Swapper A's Token B account
    #[account(mut)]
    pub token_b_from: Account<'info, TokenAccount>, // Swapper B's Token B account
    #[account(mut)]
    pub token_a_to: Account<'info, TokenAccount>,   // Swapper B's Token A account
    #[account(mut)]
    pub swapper_a: Signer<'info>,
    #[account(mut)]
    pub swapper_b: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in swapper A's account to perform the swap.")]
    InsufficientFundsA,
    #[msg("Insufficient funds in swapper B's account to perform the swap.")]
    InsufficientFundsB,
}
// Event to emit after a successful swap
#[event]
pub struct SwapExecuted {
    pub swapper_a: Pubkey,
    pub swapper_b: Pubkey,
    pub amount: u64,
}

emit!(SwapExecuted {
    swapper_a: ctx.accounts.swapper_a.key(),
    swapper_b: ctx.accounts.swapper_b.key(),
    amount,
});
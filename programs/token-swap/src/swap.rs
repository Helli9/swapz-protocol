use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("3oL1cksAJTYcutkNBhYXW2Q7EX8HH2LokwCU75gXWixq");

#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let sol = &ctx.accounts.token_sol;
        let jup = &ctx.accounts.token_jup;
        let swapper_a = &ctx.accounts.swapper_a;
        let swapper_b = &ctx.accounts.swapper_b;
        let token_program = &ctx.accounts.token_program;

        // Check if Swapper A has sufficient funds to perform the swap
        if sol.amount < amount {
            return Err(ErrorCode::InsufficientFundsA.into());
        }

        // Check if Swapper B has sufficient funds to perform the swap
        if jup.amount < amount {
            return Err(ErrorCode::InsufficientFundsB.into());
        }

        // First, transfer tokens from Swapper A's SOL to Swapper B's JUP account
        let cpi_accounts = Transfer {
            from: sol.to_account_info(),
            to: jup.to_account_info(),
            authority: swapper_a.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts),
            amount,
        )?;

        // Then, transfer tokens from Swapper B's JUP to Swapper A's SOL account
        let cpi_accounts = Transfer {
            from: jup.to_account_info(),
            to: sol.to_account_info(),
            authority: swapper_b.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), cpi_accounts),
            amount,
        )?;

        // Emit an event after successful swap
        emit!(SwapExecuted {
            swapper_a: ctx.accounts.swapper_a.key(),
            swapper_b: ctx.accounts.swapper_b.key(),
            amount,
        });

        Ok(())
    }
}

const SOL: &str = "So11111111111111111111111111111111111111112"; // wrapped SOL
const JUP: &str = "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"; // JUP

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub token_sol: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_jup: Account<'info, TokenAccount>,

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

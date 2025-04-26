use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("GXeBmV5kGR37ULMKTjdyRXjamQoXG55E7nPM1geUW37Q");

const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const JUP_MINT: Pubkey = pubkey!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let token_program = &ctx.accounts.token_program;

        // Check balances
        if ctx.accounts.token_sol_from.amount < amount {
            return Err(ErrorCode::InsufficientFundsA.into());
        }

        if ctx.accounts.token_jup_from.amount < amount {
            return Err(ErrorCode::InsufficientFundsB.into());
        }

        // Transfer SOL (wrapped) from Swapper A to Swapper B
        let transfer_sol = Transfer {
            from: ctx.accounts.token_sol_from.to_account_info(),
            to: ctx.accounts.token_sol_to.to_account_info(),
            authority: ctx.accounts.swapper_a.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), transfer_sol),
            amount,
        )?;

        // Transfer JUP from Swapper B to Swapper A
        let transfer_jup = Transfer {
            from: ctx.accounts.token_jup_from.to_account_info(),
            to: ctx.accounts.token_jup_to.to_account_info(),
            authority: ctx.accounts.swapper_b.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), transfer_jup),
            amount,
        )?;

        emit!(SwapExecuted {
            swapper_a: ctx.accounts.swapper_a.key(),
            swapper_b: ctx.accounts.swapper_b.key(),
            amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        mut,
        constraint = token_sol_from.mint == SOL_MINT,
        constraint = token_sol_from.owner == swapper_a.key(),
    )]
    pub token_sol_from: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_sol_to.mint == SOL_MINT,
        constraint = token_sol_to.owner == swapper_b.key(),
    )]
    pub token_sol_to: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_jup_from.mint == JUP_MINT,
        constraint = token_jup_from.owner == swapper_b.key(),
    )]
    pub token_jup_from: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_jup_to.mint == JUP_MINT,
        constraint = token_jup_to.owner == swapper_a.key(),
    )]
    pub token_jup_to: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub swapper_a: Signer<'info>,

    #[account(mut, signer)]
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

#[event]
pub struct SwapExecuted {
    pub swapper_a: Pubkey,
    pub swapper_b: Pubkey,
    pub amount: u64,
}

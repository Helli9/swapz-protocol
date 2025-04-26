use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::pubkey; // Add this import for the pubkey! macro

declare_id!("3At9UEz1bGW2ofW4twm4EBEmz6XRB22K19PubbmJGNP2");

const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const JUP: Pubkey = pubkey!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

// Define the event before using it in the program
#[event]
pub struct SwapExecuted {
    pub swapper_a: Pubkey,
    pub swapper_b: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        // Verify both parties have sufficient funds
        require!(
            ctx.accounts.token_sol_from.amount >= amount,
            ErrorCode::InsufficientFundsA
        );
        require!(
            ctx.accounts.token_jup_from.amount >= amount,
            ErrorCode::InsufficientFundsB
        );

        // Transfer SOL from A to B (signed by swapper_a)
        let transfer_sol = Transfer {
            from: ctx.accounts.token_sol_from.to_account_info(),
            to: ctx.accounts.token_sol_to.to_account_info(),
            authority: ctx.accounts.swapper_a.to_account_info(),
        };
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_sol,
            ),
            amount,
        )?;

        // Transfer JUP from B to A (signed by swapper_b)
        let transfer_jup = Transfer {
            from: ctx.accounts.token_jup_from.to_account_info(),
            to: ctx.accounts.token_jup_to.to_account_info(),
            authority: ctx.accounts.swapper_b.to_account_info(),
        };
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_jup,
            ),
            amount,
        )?;

        emit!(SwapExecuted {
            swapper_a: ctx.accounts.swapper_a.key(),
            swapper_b: ctx.accounts.swapper_b.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    // SOL accounts
    #[account(
        mut,
        constraint = token_sol_from.mint == SOL,
        constraint = token_sol_from.owner == swapper_a.key(),
    )]
    pub token_sol_from: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = token_sol_to.mint == SOL,
        constraint = token_sol_to.owner == swapper_b.key(),
    )]
    pub token_sol_to: Account<'info, TokenAccount>,

    // JUP accounts
    #[account(
        mut,
        constraint = token_jup_from.mint == JUP,
        constraint = token_jup_from.owner == swapper_b.key(),
    )]
    pub token_jup_from: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = token_jup_to.mint == JUP,
        constraint = token_jup_to.owner == swapper_a.key(),
    )]
    pub token_jup_to: Account<'info, TokenAccount>,

    // Participants must be signers
    #[account(mut)]
    pub swapper_a: Signer<'info>,
    #[account(mut)]
    pub swapper_b: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient SOL funds in swapper A's account")]
    InsufficientFundsA,
    #[msg("Insufficient JUP funds in swapper B's account")]
    InsufficientFundsB,
    #[msg("Token account owner does not match signer")]
    OwnerMismatch,
}
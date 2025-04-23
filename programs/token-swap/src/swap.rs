use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("3oL1cksAJTYcutkNBhYXW2Q7EX8HH2LokwCU75gXWixq");

#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        let token_program = &ctx.accounts.token_program;
    
        // Check balances
        if ctx.accounts.token_sol.amount < amount {
            return Err(ErrorCode::InsufficientFundsA.into());
        }
    
        if ctx.accounts.token_jup.amount < amount {
            return Err(ErrorCode::InsufficientFundsB.into());
        }
    
        // Transfer SOL (wrapped) from A to B
        let transfer_sol = Transfer {
            from: ctx.accounts.token_sol.to_account_info(),
            to: ctx.accounts.token_jup.to_account_info(),
            authority: ctx.accounts.swapper_a.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), transfer_sol),
            amount,
        )?;
    
        // Transfer JUP from B to A
        let transfer_jup = Transfer {
            from: ctx.accounts.token_jup.to_account_info(),
            to: ctx.accounts.token_sol.to_account_info(),
            authority: ctx.accounts.swapper_b.to_account_info(),
        };
        token::transfer(
            CpiContext::new(token_program.to_account_info(), transfer_jup),
            amount,
        )?;
    
        Ok(())
    }
}

//const SOL: &str = "So11111111111111111111111111111111111111112"; // wrapped SOL
//const JUP: &str = "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"; // JUP

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, associated_token::mint = SOL_MINT, associated_token::authority = swapper_a)]
pub token_sol: Account<'info, TokenAccount>,

#[account(mut, associated_token::mint = JUP_MINT, associated_token::authority = swapper_b)]
pub token_jup: Account<'info, TokenAccount>,


    #[account(signer)]
    pub swapper_a: AccountInfo<'info>,

    #[account(signer)]
    pub swapper_b: AccountInfo<'info>,

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

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, TransferParams};

declare_id!("3At9UEz1bGW2ofW4twm4EBEmz6XRB22K19PubbmJGNP2");


#[event]
pub struct BuyNuEvent {
    pub user: Pubkey,
    pub usdt_amount: u64,
    pub nu_amount: u64,
}
#[program]
pub mod direct_token_swap {
    use super::*;

    pub fn buy_nu(ctx: Context<BuyNU>, amount_nu: u64) -> Result<()> {
        // Ensure user has enough USDT
        require!(
            ctx.accounts.usdt_from.amount >= amount_nu,
            ErrorCode::InsufficientFunds
        );

        let cpi_usdt_transfer = Transfer {
            from: ctx.accounts.usdt_from.to_account_info(),
            to: ctx.accounts.usdt_pool.to_account_info(),
            authority: ctx.accounts.swapper.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_usdt_transfer,
            ),
            amount_nu,
        )?;

        let cpi_nu_transfer = Transfer {
            from: ctx.accounts.nu_pool.to_account_info(),
            to: ctx.accounts.nu_to.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(), // needs to be a signer (PDA)
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_nu_transfer,
                &[&[b"authority", &[ctx.bumps.pool_authority]]], // example seeds
            ),
            amount_nu,
        )?;

        // Emit the event
        emit!(BuyNuEvent {
        user: ctx.accounts.swapper.key(),
        usdt_amount: amount_nu,
        nu_amount: amount_nu,
         });

        Ok(())
    }
}
#[derive(Accounts)]
pub struct BuyNU<'info> {
    #[account(mut)]
    pub swapper: Signer<'info>,

    #[account(mut)]
    pub usdt_from: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub usdt_pool: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub nu_pool: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub nu_to: Box<Account<'info, TokenAccount>>,

    /// CHECK: Make sure this is a PDA signer
    pub pool_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

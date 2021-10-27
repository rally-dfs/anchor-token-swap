// #![deny(missing_docs)]
// TODO: the above causes errors with #[program]

//! An Uniswap-like program for the Solana blockchain.

use anchor_lang::prelude::*;

pub mod constraints;
pub mod curve;
pub mod error;
pub mod instruction_nonanchor;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// solana_program::declare_id!("SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8");
declare_id!("SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8");

// TODO: move this somewhere else

// TODO: #[program] does some special casing with some "instruction" mod i think, so we can't have our own instruction.rs (compiler error if we add `pub mod instruction`)

/// documentation
#[program]
mod anchor_token_swap {
    use super::*;

    pub fn deposit_single_token_type_exact_amount_in(
        ctx: Context<DepositSingleTokenTypeExactAmountInAnchor>,
        source_token_amount: u64,
        minimum_pool_token_amount: u64,
    ) -> ProgramResult {
        // TODO: maybe not the best way to do this probably, kind of defeating the purpose of
        // anchor, but lets us just use process_foo directly
        let accounts = [
            ctx.accounts.token_swap.clone(),
            ctx.accounts.swap_authority.clone(),
            ctx.accounts.user_transfer_authority.clone(),
            ctx.accounts.source_token.clone(),
            ctx.accounts.swap_token_a.clone(),
            ctx.accounts.swap_token_b.clone(),
            ctx.accounts.pool_mint.clone(),
            ctx.accounts.destination.clone(),
            ctx.accounts.token_program.clone(),
        ];

        processor::Processor::process_deposit_single_token_type_exact_amount_in(
            ctx.program_id,
            source_token_amount,
            minimum_pool_token_amount,
            &accounts,
        )
    }
}

// TODO: put this somewhere else, clean up

///   Deposit one type of tokens into the pool.  The output is a "pool" token
///   representing ownership into the pool. Input token is converted as if
///   a swap and deposit all token types were performed.
#[derive(Accounts)]
pub struct DepositSingleTokenTypeExactAmountInAnchor<'info> {
    ///   0. `[]` Token-swap
    pub token_swap: AccountInfo<'info>,
    ///   1. `[]` swap authority
    pub swap_authority: AccountInfo<'info>,
    ///   2. `[]` user transfer authority
    pub user_transfer_authority: AccountInfo<'info>,
    ///   3. `[writable]` token_(A|B) SOURCE Account, amount is transferable by user transfer authority,
    #[account(mut)]
    pub source_token: AccountInfo<'info>,
    ///   4. `[writable]` token_a Swap Account, may deposit INTO.
    #[account(mut)]
    pub swap_token_a: AccountInfo<'info>,
    ///   5. `[writable]` token_b Swap Account, may deposit INTO.
    #[account(mut)]
    pub swap_token_b: AccountInfo<'info>,
    ///   6. `[writable]` Pool MINT account, swap authority is the owner.
    #[account(mut)]
    pub pool_mint: AccountInfo<'info>,
    ///   7. `[writable]` Pool Account to deposit the generated tokens, user is the owner.
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    ///   8. '[]` Token program id
    pub token_program: AccountInfo<'info>,
}

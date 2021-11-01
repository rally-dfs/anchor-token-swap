// #![deny(missing_docs)]
// TODO: the above causes errors with #[program]

//! An Uniswap-like program for the Solana blockchain.

use crate::constraints::SWAP_CONSTRAINTS;
use crate::curve::{base::SwapCurve, fees::Fees};
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

    pub fn initialize(
        ctx: Context<InitializeAnchor>,
        fees: Fees,
        swap_curve: SwapCurve,
        // TODO: could use something like this here too not sure what's more convenient `data: instruction_nonanchor::InitializeInstructionData,`
        // (would just need to derive(AnchorDeserialize, AnchorSerialize) on it first)
    ) -> ProgramResult {
        let accounts = [
            ctx.accounts.token_swap.clone(),
            ctx.accounts.swap_authority.clone(),
            ctx.accounts.token_a.clone(),
            ctx.accounts.token_b.clone(),
            ctx.accounts.pool.clone(),
            ctx.accounts.fee.clone(),
            ctx.accounts.destination.clone(),
            ctx.accounts.token_program.clone(),
        ];
        processor::Processor::process_initialize(
            ctx.program_id,
            fees,
            swap_curve,
            &accounts,
            &SWAP_CONSTRAINTS,
        )
    }

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

///   Initializes a new swap
#[derive(Accounts)]
pub struct InitializeAnchor<'info> {
    ///   0. `[writable, signer]` New Token-swap to create.
    #[account(mut, signer)]
    pub token_swap: AccountInfo<'info>,

    ///   1. `[]` swap authority derived from `create_program_address(&[Token-swap account])`
    pub swap_authority: AccountInfo<'info>,

    ///   2. `[]` token_a Account. Must be non zero, owned by swap authority.
    pub token_a: AccountInfo<'info>,

    ///   3. `[]` token_b Account. Must be non zero, owned by swap authority.
    pub token_b: AccountInfo<'info>,

    ///   4. `[writable]` Pool Token Mint. Must be empty, owned by swap authority.
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    ///   5. `[]` Pool Token Account to deposit trading and withdraw fees.
    ///   Must be empty, not owned by swap authority
    pub fee: AccountInfo<'info>,

    ///   6. `[writable]` Pool Token Account to deposit the initial pool token
    ///   supply.  Must be empty, not owned by swap authority.
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    ///   7. '[]` Token program id
    pub token_program: AccountInfo<'info>,
}

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

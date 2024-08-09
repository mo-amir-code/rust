use anchor_lang::prelude::*;
use anchor_lang::{
    solana_program, solana_program::entrypoint::ProgramResult, solana_program::system_instruction,
};
use anchor_spl::token::{self, Token};
use std::mem::size_of;

declare_id!("6powecXbdSn7yUArKTEQGC5UmybBvYPt6EVooGg5ptEo");

const TEXT_LENGTH: usize = 255;
const MUSIC_URL_LENGTH: usize = 255;

#[program]
mod meterio {
    use super::*;

    pub fn accept_payment(ctx: Context<PayerContext>) -> ProgramResult {
        let payer_wallet = &mut ctx.accounts.payer_wallet;
        payer_wallet.wallet = ctx.accounts.authority.key();

        let tx = system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.reciever.key(),
            1000000000,
        );

        solana_program::program::invoke(
            &tx,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.reciever.to_account_info(),
            ],
        )
    }

    pub fn create_music(
        ctx: Context<CreateMusic>,
        title: String,
        music_url: String,
    ) -> ProgramResult {
        let music = &mut ctx.accounts.music;

        music.authority = ctx.accounts.authority.key();
        music.title = title;
        music.music_url = music_url;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    #[account(
        init,
        seeds = [b"payer".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PayerAccount>() + 8
    )]
    pub payer_wallet: Account<'info, PayerAccount>,

    #[account(mut)]
    pub reciever: AccountInfo<'info>,

    // This is signer who paid transaction fee
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    // Token Program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct PayerAccount {
    pub wallet: Pubkey,
}

// Create Music Context
#[derive(Accounts)]
pub struct CreateMusic<'info> {
    #[account(
        init,
        seeds = [b"music".as_ref(), randomkey.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<MusicAccount>() + TEXT_LENGTH + MUSIC_URL_LENGTH + 8
    )]
    pub music: Account<'info, MusicAccount>,

    // This is signer who paid transaction fee
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub randomkey: AccountInfo<'info>,

    pub system_program: UncheckedAccount<'info>,

    // Token Program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct MusicAccount {
    pub authority: Pubkey,
    pub title: String,
    pub music_url: String,
}
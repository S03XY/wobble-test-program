use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer_checked;
use spl_token::ui_amount_to_amount;
use std::str::FromStr;

declare_id!("8ptgXekj3hdRqdXvrW8cr55ZoMNLeyqa7Xp13XAAUMBQ");

// CHANGE THIS FOR RECEIVER WALLET
const RECEIVER_WALLET: &str = "8WiSJ8Z92Rc6SMnbFNV5GdSr1YTnFP9gQ7NwCjhgYKJE";
const BUYER_PREFIX: &str = "buyer";

#[program]
pub mod wobble_synth {

    use super::*;

    pub fn buy_first_song(ctx: Context<CreateUser0>) -> Result<()> {
        let user_0 = &mut ctx.accounts.user0;
        let token_mint = &mut ctx.accounts.tokenmint;
        let user = &mut ctx.accounts.user;
        let source = &mut ctx.accounts.source;
        let destination = &mut ctx.accounts.destination;
        let token_program_id = &mut ctx.accounts.token_program_id;

        let receiver_pubkey = Pubkey::from_str(RECEIVER_WALLET).unwrap();
        let receiver_ata = get_associated_token_address(&receiver_pubkey, &token_mint.key());
        let formatted_amount = ui_amount_to_amount(8.0, token_mint.decimals);

        let trasnferins = transfer_checked(
            &token_program_id.key(),
            &source.key(),
            &token_mint.key(),
            &receiver_ata,
            &user.key(),
            &[&user.key()],
            formatted_amount,
            token_mint.decimals,
        )?;

        invoke(
            &trasnferins,
            &[
                token_program_id.to_account_info(),
                token_mint.to_account_info(),
                destination.to_account_info(),
                source.to_account_info(),
                user.to_account_info(),
            ],
        )?;

        user_0.wallet = user.key();
        user_0.songs_count = 1;

        Ok(())
    }

    pub fn buy_song(ctx: Context<UpdateUser0>) -> Result<()> {
        let user_0 = &mut ctx.accounts.user0;
        let source = &mut ctx.accounts.source;
        let destination = &mut ctx.accounts.destination;
        let token_program_id = &mut ctx.accounts.token_program_id;
        let token_mint = &mut ctx.accounts.tokenmint;
        let receiver_pubkey = Pubkey::from_str(RECEIVER_WALLET).unwrap();
        let receiver_ata = get_associated_token_address(&receiver_pubkey, &token_mint.key());
        let formatted_amount = ui_amount_to_amount(8.0, token_mint.decimals);
        let wallet = &mut ctx.accounts.wallet;

        let trasnferins = transfer_checked(
            &token_program_id.key(),
            &source.key(),
            &token_mint.key(),
            &receiver_ata,
            &wallet.key(),
            &[&wallet.key()],
            formatted_amount,
            token_mint.decimals,
        )?;

        invoke(
            &trasnferins,
            &[
                token_program_id.to_account_info(),
                token_mint.to_account_info(),
                destination.to_account_info(),
                source.to_account_info(),
                wallet.to_account_info(),
            ],
        )?;

        user_0.songs_count = user_0.songs_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser0<'info> {
    #[account(init,payer=user,seeds=[BUYER_PREFIX.as_bytes(),user.key().as_ref(),tokenmint.key().as_ref()],bump, space=User0::LEN)]
    pub user0: Account<'info, User0>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub tokenmint: Account<'info, Mint>,
    pub token_program_id: Program<'info, Token>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    /// CHECK: already checked
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser0<'info> {
    #[account(mut, has_one=wallet)] // has_one=author => user0.author == UpdateUser0.author.key
    pub user0: Account<'info, User0>,
    pub wallet: Signer<'info>,
    pub token_program_id: Program<'info, Token>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    /// CHECK: already checked
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    pub tokenmint: Account<'info, Mint>,
}

#[account]
pub struct User0 {
    wallet: Pubkey,
    songs_count: u16,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const SONGS_COUNT_LENGTH: usize = 2; // u16 == 2 bytes
impl User0 {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + SONGS_COUNT_LENGTH;
}

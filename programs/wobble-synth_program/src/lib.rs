use anchor_lang::prelude::*;

declare_id!("54mzQZxsFhqshuPkQhgHU8EBKTnLMtdv51Lpx8Cj4YMB");

#[program]
pub mod wobble_synth {
    use super::*;

    pub fn buy_first_song(ctx: Context<CreateUser0>) -> Result<()> {
        let user_0 = &mut ctx.accounts.user0;
        user_0.songs_count = 1;

        let author = &ctx.accounts.user;
        user_0.wallet = *author.key;
        Ok(())
    }

    pub fn buy_song(ctx: Context<UpdateUser0>) -> Result<()> {
        let user_0 = &mut ctx.accounts.user0;
        user_0.songs_count = user_0.songs_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser0<'info> {
    #[account(init, payer=user, space=User0::LEN)]
    pub user0: Account<'info, User0>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser0<'info> {
    #[account(mut, has_one=wallet)] // has_one=author => user0.author == UpdateUser0.author.key
    pub user0: Account<'info, User0>,
    pub wallet: Signer<'info>,
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

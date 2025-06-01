use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("HcM4LfnygJcEv4rhpJojre5MaNbjLSSwdDuo3XStNdRN");

const BITCOIN_DECIMALS: u8 = 8;

#[event]
pub struct BurnTo {
    pub amount: u64,
    pub to: String,
}

#[program]
pub mod btc_token {
    use super::*;

    pub fn initialize_mint(_ctx: Context<InitializeMintAccount>) -> Result<()> {
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, btc_address: String, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.into_burn_context(), amount)?;

        emit!(BurnTo {
            amount,
            to: btc_address,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMintAccount<'info> {
    #[account(
        init,
        payer = signer,
        mint::decimals = BITCOIN_DECIMALS,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: used only for reading the address
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BurnTokens<'info> {
    fn into_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.mint.to_account_info(),
                from: self.from.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
}

use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, MetadataAccount,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
pub mod pda;
use crate::pda::find_metadata_account;

declare_id!("7B1LRb9KW4K6nfDR7K8stLFDM78UbReofX4aMc28WBNX");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn init_nft(
        ctx: Context<InitNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // create mint account

        // if *ctx.accounts.signer.key != *ctx.accounts.mint.authority {
        //     return Err(ErrorCode::Unauthorized.into());
        // }

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, 1)?;

        // create metadata account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        Ok(())
    }
    // pub fn create_raffle(
    //     ctx: Context<CreateRaffle>,
    //     price: u64,
    //     ends: i64,
    //     title: String,
    //     description: String,
    //     image: String,
    //     winners: u8,
    //     requires_author: u8,
    // ) -> Result<()> {
    //     if ![0, 1].contains(&requires_author) {
    //         return Err(RaffleError::InputError.into());
    //     }
    //     if image.chars().count() > 50 {
    //         return Err(RaffleError::InputError.into());
    //     }
    //     if title.chars().count() > 50 {
    //         return Err(RaffleError::InputError.into());
    //     }
    //     if description.chars().count() > 100 {
    //         return Err(RaffleError::InputError.into());
    //     }

    //     let raffle = &mut ctx.accounts.raffle;
    //     raffle.authority = ctx.accounts.authority.key();
    //     raffle.ends = ends;
    //     raffle.price = price;
    //     raffle.title = title;
    //     raffle.description = description;
    //     raffle.image = image;
    //     raffle.winners = winners;
    //     raffle.requires_author = requires_author;
    //     raffle.token = ctx.accounts.token_mint.key();
    //     Ok(())
    // }
    // pub fn purchase_ticket(
    //     ctx: Context<CreateTicket>
    // ) -> Result<()> {

    //     let clock: Clock = Clock::get().unwrap();
    //     let raffle  = &ctx.accounts.raffle;

    //     require_keys_eq!(
    //         ctx.accounts.raffle.authority,
    //         ctx.accounts.authority_ata.owner,
    //         RaffleError::Unauthorized
    //     );

    //     require_keys_eq!(
    //         ctx.accounts.raffle.token,
    //         ctx.accounts.token_mint.key(),
    //         RaffleError::Unauthorized
    //     );

    //     require_keys_eq!(
    //         ctx.accounts.raffle.authority,
    //         ctx.accounts.authority.key(),
    //         RaffleError::Unauthorized
    //     );

    //     if raffle.requires_author == 1 {
    //         require_keys_eq!(
    //             ctx.accounts.raffle.authority,
    //             ctx.accounts.need_signer.key(),
    //             RaffleError::Unauthorized
    //         );
    //     };

    //     if raffle.ends < clock.unix_timestamp {
    //         return Err(RaffleError::RaffleEnded.into());
    //     };

    //     let token_ctx = CpiContext::new(
    //         ctx.accounts.token_program.to_account_info(),
    //         token::Transfer {
    //             authority: ctx.accounts.participant.to_account_info(),
    //             from: ctx.accounts.participant_ata.to_account_info(),
    //             to: ctx.accounts.authority_ata.to_account_info(),
    //         },
    //     );

    //     token::transfer(token_ctx, raffle.price)?;

    //     let ticket = &mut ctx.accounts.ticket;
    //     ticket.raffle = ctx.accounts.raffle.key();
    //     ticket.participant = ctx.accounts.participant.key();
    //     Ok(())
    // }

    // pub fn end_raffle(_ctx: Context<EndRaffle>) -> Result<()> { Ok(()) }

    // pub fn close_ticket_account(_ctx: Context<CloseTicketAccount>) ->  Result<()>{ Ok(()) }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    /// CHECK - address
    #[account(
        mut,
        address=find_metadata_account(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: address
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// #[derive(Accounts)]
// pub struct CreateRaffle<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,

//     #[account(
//         init,
//         payer = authority,
//         space = 8 + Raffle::LEN
//     )]
//     pub raffle: Box<Account<'info, Raffle>>,

//     pub token_mint: Account<'info, Mint>,
//     pub token_program: Program<'info, Token>,

//     pub associated_token_program: Program<'info, AssociatedToken>,

//     pub system_program: Program<'info, System>,

// }
// #[derive(Accounts)]
// pub struct CreateTicket<'info> {

//     /// CHECK: we good
//     pub authority: AccountInfo<'info>,

//     pub need_signer: Signer<'info>,

//     #[account(mut)]
//     pub participant: Signer<'info>,

//     #[account(mut)]
//     pub raffle: Box<Account<'info, Raffle>>,

//     #[account(
//         init,
//         payer = participant,
//         space = 8 + Ticket::LEN
//     )]
//     pub ticket: Box<Account<'info, Ticket>>,

//     #[account(mut)]
//     pub participant_ata: Account<'info, TokenAccount>,

//     #[account(
//         init_if_needed,
//         payer = participant,
//         associated_token::mint = token_mint,
//         associated_token::authority = authority,
//     )]
//     pub authority_ata: Account<'info, TokenAccount>,

//     pub token_mint: Account<'info, Mint>,
//     pub token_program: Program<'info, Token>,

//     pub associated_token_program: Program<'info, AssociatedToken>,

//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

// #[derive(Accounts)]
// pub struct EndRaffle<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(
//         mut,
//         has_one = authority,
//         close = authority
//     )]
//     pub raffle: Box<Account<'info, Raffle>>,
// }

// #[derive(Accounts)]
// pub struct CloseTicketAccount<'info> {
//     #[account(mut)]
//     pub participant: Signer<'info>,
//     #[account(
//         mut,
//         has_one = participant,
//         close = participant
//     )]
//     pub ticket: Box<Account<'info, Ticket>>,
// }

// #[account]
// pub struct Raffle {
//     pub authority: Pubkey,
//     pub ends: i64,
//     pub title: String, // 50 * 4
//     pub description: String, // 100 * 4
//     pub image: String, //100 * 4
//     pub winners: u8, //
//     pub requires_author: u8,
//     pub price: u64,
//     pub token: Pubkey,
// }

// #[account]
// pub struct Ticket {
//     pub raffle: Pubkey,
//     pub participant: Pubkey
// }

// impl Raffle {
//     pub const LEN: usize = 32  + 8 + (50 * 4) + (100 * 4) + (100 * 4) + 1 + 8 + 16 + 1 + 32;
// }

// impl Ticket {
//     pub const LEN: usize = 32 + 32;
// }

// #[error_code]
// pub enum RaffleError {
//     #[msg("Raffle Has Ended")]
//     RaffleEnded,
//     #[msg("Input Error")]
//     InputError,
//     #[msg("Unauthorized")]
//     Unauthorized,
// }

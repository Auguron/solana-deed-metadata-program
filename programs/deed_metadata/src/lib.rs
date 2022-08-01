#![allow(unused_variables)]

use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::hash::hash;

declare_id!("HAcjmTqRVugqXdgyMkNyK5Kt4JsTDwn1dMKTbQSJ4i4K");

#[program]
pub mod deed_metadata {

    use super::*;
    pub fn new_deed_metadata(
        ctx: Context<NewDeedMetadata>,
        date_of_registration: u64,
        price: u64,
        price_denomination: [u8; 3],
        buyer: String,
        seller: String,
        property: String,
        property_hash: [u8; 32],
        uri: String,
        ) -> Result<()> {
        let act = &mut ctx.accounts.deed_metadata;
        act.authority = *ctx.accounts.authority.key;
        act.created_by = *ctx.accounts.authority.key;
        act.date_of_registration = date_of_registration;
        act.price = price;
        act.price_denomination = price_denomination;
        act.buyer = buyer;
        act.seller = seller;
        act.property = property;
        act.uri = uri;
        msg!(&act.buyer);
        msg!(&act.seller);
        msg!(&act.property);
        msg!(&act.uri);
        Ok(())
    }

    pub fn close_deed_metadata(
        ctx: Context<CloseDeedMetadata>,
        ) -> Result<()> {
        ctx.accounts.deed_metadata.close(ctx.accounts.authority.to_account_info())?;
        Ok(())
    }

    pub fn update_uri(
        ctx: Context<UpdateUri>,
        uri: String,
        ) -> Result<()> {
        let act = &mut ctx.accounts.deed_metadata;
        act.uri = uri;
        Ok(())
    }

    pub fn set_pending_authority(
        ctx: Context<SetPendingAuthority>,
        pending_authority: Pubkey,
        ) -> Result<()> {
        let deed_metadata = &mut ctx.accounts.deed_metadata;
        deed_metadata.pending_authority = Some(pending_authority);
        Ok(())
    }

    pub fn accept_pending_authority(
        ctx: Context<AcceptPendingAuthority>,
        ) -> Result<()> {
        let deed_metadata = &mut ctx.accounts.deed_metadata;
        deed_metadata.authority = deed_metadata.pending_authority.unwrap();
        deed_metadata.pending_authority = None;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    date_of_registration: u64,
    price: u64,
    price_denomination: [u8; 3],
    buyer: String,
    seller: String,
    property: String,
    property_hash: [u8; 32],
    uri: String,
    )]
pub struct NewDeedMetadata<'info> {
    #[account(init, payer=authority,
      constraint=hash(property.as_bytes()).as_ref() == property_hash,
      space=8 // Discriminator
      + 32 // authority
      + 33 // pending_authority
      + 32 // created_by
      + 8 // date_of_registration
      + 8 // price
      + 3 // price_denomination
      + 4 // buyer length (encoded automatically)
      + buyer.len()
      + 4 // seller length (encoded automatically)
      + seller.len()
      + 4 // property length (encoded automatically)
      + property.len()
      + 4 // uri length (encoded automatically)
      + uri.len(),
      seeds=[
        &authority.key.to_bytes().to_vec(),
        &property_hash.to_vec(),
        &b"deed_metadata".to_vec(),
      ],
      bump,
      )]
    pub deed_metadata: Account<'info, DeedMetadataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseDeedMetadata<'info> {
    #[account(mut, has_one = authority)]
    pub deed_metadata: Account<'info, DeedMetadataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct SetPendingAuthority<'info> {
    #[account(mut, has_one = authority)]
    pub deed_metadata: Account<'info, DeedMetadataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptPendingAuthority<'info> {
    #[account(mut,
        constraint = deed_metadata.pending_authority == Some(pending_authority.key())
    )]
    pub deed_metadata: Account<'info, DeedMetadataAccount>,
    #[account(mut)]
    pub pending_authority: Signer<'info>,
}


#[derive(Accounts)]
pub struct UpdateUri<'info> {
    #[account(mut, has_one = authority)]
    pub deed_metadata: Account<'info, DeedMetadataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}


#[account]
pub struct DeedMetadataAccount {
    pub authority: Pubkey,
    pub pending_authority: Option<Pubkey>,
    pub created_by: Pubkey,
    pub date_of_registration: u64,
    pub price: u64,
    pub price_denomination: [u8; 3],
    pub buyer: String,
    pub seller: String,
    pub property: String,
    pub uri: String,
}

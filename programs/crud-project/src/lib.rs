// For this project, we will be using the anchor framework to create a simple journal program.
// The journal will have a title and a body.
// The journal will be owned by the user who created it.
// The journal will be stored in the user's account.

use anchor_lang::prelude::*;

declare_id!("3p6qiHVMMcbLFtTHNzQDHUNV61HYzp43dMx4zQGurrC5");

#[program]
pub mod crud_project {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        journal_entry.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()], //PDA
        bump,
        space = 8 + JournalEntryState::INIT_SPACE, // this is to calculate the space required for the journal account
        payer = owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account{
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], //PDA
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true, // delete everything and calculate the space again
    }]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}   

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], //PDA
        bump,
        close = owner, // To close the account
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account] // Initialize the journal account
#[derive(InitSpace)] // To calculate the space required for the journal account - String usually will create error
pub struct JournalEntryState { 
    pub owner: Pubkey, // The owner of the journal
    #[max_len(50)]
    pub title: String, 
    #[max_len(200)]
    pub message: String,
}
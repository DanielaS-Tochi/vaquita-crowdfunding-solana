use anchor_lang::prelude::*;

declare_id!("HZ99vDQQ74KwcFGQzbCupESFryEPdj7Cf248mm7kTY4b");

#[program]
mod vaquita {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        goal: u64,
        deadline: i64,
    ) -> Result<()> {

        let campaign = &mut ctx.accounts.campaign;

        campaign.creator = ctx.accounts.creator.key();
        campaign.goal = goal;
        campaign.deadline = deadline;
        campaign.amount_raised = 0;
        campaign.claimed = false;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {

        let campaign = &mut ctx.accounts.campaign;
        let donation = &mut ctx.accounts.donation;

        donation.donor = ctx.accounts.donor.key();
        donation.campaign = campaign.key();
        donation.amount = donation.amount + amount;

        campaign.amount_raised = campaign.amount_raised + amount;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donor.key(),
            &campaign.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.donor.to_account_info(),
                campaign.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {

    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 8 + 8 + 1
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {

    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = donor,
        space = 8 + 32 + 32 + 8,
        seeds = [b"donation", campaign.key().as_ref(), donor.key().as_ref()],
        bump
    )]
    pub donation: Account<'info, Donation>,

    #[account(mut)]
    pub donor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Donation {
    pub donor: Pubkey,
    pub campaign: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Campaign {
    pub creator: Pubkey,
    pub goal: u64,
    pub deadline: i64,
    pub amount_raised: u64,
    pub claimed: bool,
}
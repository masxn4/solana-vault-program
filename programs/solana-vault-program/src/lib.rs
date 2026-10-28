use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("VaU1t1111111111111111111111111111111111111");

#[program]
pub mod solana_vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = ctx.bumps.vault;
        vault.total_deposited = 0;
        vault.total_withdrawn = 0;
        emit!(VaultInitialized { vault: vault.key(), authority: vault.authority });
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::ZeroAmount);
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        ctx.accounts.vault.total_deposited = ctx.accounts.vault.total_deposited
            .checked_add(amount).ok_or(VaultError::MathOverflow)?;
        emit!(Deposited { vault: ctx.accounts.vault.key(), authority: ctx.accounts.authority.key(), amount });
        Ok(())
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        let rent = Rent::get()?.minimum_balance(Vault::SPACE);
        require!(ctx.accounts.vault.to_account_info().lamports() <= rent, VaultError::VaultNotEmpty);
        emit!(VaultClosed { vault: ctx.accounts.vault.key(), authority: ctx.accounts.authority.key() });
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::ZeroAmount);
        let rent = Rent::get()?.minimum_balance(Vault::SPACE);
        let balance = ctx.accounts.vault.to_account_info().lamports();
        let available = balance.checked_sub(rent).ok_or(VaultError::InsufficientFunds)?;
        require!(amount <= available, VaultError::InsufficientFunds);

        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;
        ctx.accounts.vault.total_withdrawn = ctx.accounts.vault.total_withdrawn
            .checked_add(amount).ok_or(VaultError::MathOverflow)?;
        emit!(Withdrawn { vault: ctx.accounts.vault.key(), authority: ctx.accounts.authority.key(), amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"vault", authority.key().as_ref()], bump = vault.bump, has_one = authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault", authority.key().as_ref()], bump = vault.bump, has_one = authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
}

impl Vault {
    pub const SPACE: usize = 8 + 32 + 1 + 8 + 8;
}

#[event]
pub struct VaultInitialized { pub vault: Pubkey, pub authority: Pubkey }
#[event]
pub struct Deposited { pub vault: Pubkey, pub authority: Pubkey, pub amount: u64 }
#[event]
pub struct VaultClosed { pub vault: Pubkey, pub authority: Pubkey }
#[event]
pub struct Withdrawn { pub vault: Pubkey, pub authority: Pubkey, pub amount: u64 }

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than zero.")]
    ZeroAmount,
    #[msg("The vault does not have enough withdrawable SOL.")]
    InsufficientFunds,
    #[msg("Arithmetic overflow.")]
    MathOverflow,
    #[msg("Withdraw funds before closing the vault.")]
    VaultNotEmpty,
}

use anchor_lang::prelude::*;

declare_id!("8ThKqoTws1H2vAAWUUAnTKqQcLkCdc6Ua7GRyUhegqY1");

#[program]
pub mod solanox {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("BBocCx2HTUWjcsm9REjSJ3FYyE8RD5968KBiZ58ordJe");

#[program]
pub mod sol_gift {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

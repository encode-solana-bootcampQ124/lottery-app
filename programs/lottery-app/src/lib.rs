use anchor_lang::prelude::*;

declare_id!("2vz4HNoNRNhiWef7ZnHfHZbgaoGjYiGgx3jFFo9mtVJH");

#[program]
pub mod lottery_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

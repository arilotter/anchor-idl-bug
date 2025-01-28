use anchor_lang::prelude::*;

declare_id!("GETJtjFdo9LLt56QW681NPj3x74WuNhcFKULX2hU7eQc");

#[program]
pub mod idl_repro {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.coordinator.checkpointers = Foo::<16>::default();
        Ok(())
    }
}

pub const SOME_CONST: usize = 16;

#[account]
#[derive(
    Debug, Copy
)]
#[repr(C)]
pub struct Bar {
    // Using this const causes the error:
    // error[E0573]: expected type, found constant `SOME_CONST`
    //   --> programs/idl-repro/src/lib.rs:23:31
    //    |
    // 23 |     pub checkpointers: Foo<I, SOME_CONST>,
    //    |                               ^^^^^^^^^^ not a type
    //
    // However, if you replace SOME_CONST with the literal `16`, there's no error.
    pub checkpointers: Foo<SOME_CONST>, 
}

#[derive(Clone, Copy, Debug, AnchorDeserialize, AnchorSerialize,Default)]
pub struct Foo<const N: usize>;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<Bar>())]
    pub coordinator: Account<'info, Bar>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
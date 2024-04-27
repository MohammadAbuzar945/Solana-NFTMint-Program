use anchor_lang::prelude::Pubkey;


pub const PREFIX: &str = "metadata";
pub fn find_metadata_account(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[PREFIX.as_bytes(), crate::ID.as_ref(), mint.as_ref()],
        &crate::ID,
    )
}

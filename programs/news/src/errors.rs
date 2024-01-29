use anchor_lang::prelude::*;

#[error_code]
pub enum NewsError {
    #[msg("This news haven't been approved")]
    NotApprovedNews,
    #[msg("There is nothing revenue!")]
    NothingRevenue,
    #[msg("Overflow Maxcount!")]
    OverMaxCount,
    #[msg("Invalid Price!")]
    InvalidPrice,
    #[msg("Insufficient Funds!")]
    InsufficientFunds,
    #[msg("Not Junior Reporter!")]
    NotJuniorReporter,
    #[msg("Not Senior Reporter!")]
    NotSeniorReporter,
    #[msg("Not Admin!")]
    NotAdmin,
    #[msg("Create Reporter Error!")]
    CreateReporterError,
    #[msg("Edit Reporter Error!")]
    EditReporterError,
    #[msg("Delete Reporter Error!")]
    DeleteReporterError
}
// Error for transfer failed

#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum WedprError {
    #[fail(display = "Verification failed.")]
    VerificationError,
    #[fail(display = "Data could not be parsed.")]
    FormatError,
}

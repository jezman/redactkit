/// Common sensitive field names.
///
/// These are field names that often contain secrets or credentials.
pub const DEFAULT_SENSITIVE_FIELDS: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "access_token",
    "refresh_token",
    "api_key",
    "apikey",
    "authorization",
    "private_key",
    "client_secret",
    "database_url",
];

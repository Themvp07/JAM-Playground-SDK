// lib.rs

// Local definition of the `Context` struct.
// This struct represents the caller's context and contains information about who is performing the action.
// - `caller`: Identifies the caller, typically represented as an address or unique identifier.
pub struct Context {
    pub caller: String, // Caller's address or identifier
}

// Implementation of a whitelist-based authorizer.
// This authorizer checks if a caller is authorized to perform certain actions.
pub struct WhitelistAuthorizer {
    allowed_address: String, // Allowed address in the whitelist
}

impl WhitelistAuthorizer {
    // Constructor to create a new instance of `WhitelistAuthorizer`.
    // - `allowed_address`: The address that will be considered authorized.
    pub fn new(allowed_address: String) -> Self {
        Self { allowed_address }
    }

    // Method to check if a caller is authorized.
    // - `ctx`: The caller's context, which includes their address.
    // Returns `true` if the caller is authorized (i.e., their address matches the allowed address).
    pub fn is_authorized(&self, ctx: &Context) -> bool {
        ctx.caller == self.allowed_address
    }
}
use serde::Serialize;

/// Shared API response wrapper used by every JSON endpoint.
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

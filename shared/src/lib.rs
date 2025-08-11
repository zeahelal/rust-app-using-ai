pub mod models {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: Uuid,
        pub email: String,
        pub display_name: String,
        pub created_at: DateTime<Utc>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuthRequest {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuthResponse {
        pub token: String,
        pub user: User,
    }
}

pub mod utils {
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}

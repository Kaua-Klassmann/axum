use crate::jwt::JwtClaims;

pub async fn token(token: JwtClaims) -> String {
    format!("Seu id: {}", token.user_id)
}
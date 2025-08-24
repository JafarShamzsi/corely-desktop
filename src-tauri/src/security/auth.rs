use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub last_login: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub roles: Vec<String>,
    pub exp: usize, // Expiration time (as UTC timestamp)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub expires_at: String,
}

pub struct AuthManager {
    users: HashMap<String, User>,
    jwt_secret: Vec<u8>,
    token_expiry_hours: u64,
}

impl AuthManager {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            users: HashMap::new(),
            jwt_secret: jwt_secret.into_bytes(),
            token_expiry_hours: 24, // 24 hours default
        }
    }
    
    pub fn register_user(&mut self, username: String, email: String, password: String) -> Result<User, Box<dyn std::error::Error>> {
        // Check if user already exists
        if self.users.values().any(|u| u.username == username || u.email == email) {
            return Err("User already exists".into());
        }
        
        // Hash password
        let password_hash = hash(password, DEFAULT_COST)?;
        
        let user = User {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            created_at: chrono::Utc::now().to_rfc3339(),
            last_login: None,
            roles: vec!["user".to_string()],
        };
        
        self.users.insert(user.id.clone(), user.clone());
        Ok(user)
    }
    
    pub fn login(&mut self, request: LoginRequest) -> Result<AuthResponse, Box<dyn std::error::Error>> {
        // Find user by username
        let user = self.users.values_mut()
            .find(|u| u.username == request.username)
            .ok_or("Invalid credentials")?;
        
        // Verify password
        if !verify(&request.password, &user.password_hash)? {
            return Err("Invalid credentials".into());
        }
        
        // Update last login
        user.last_login = Some(chrono::Utc::now().to_rfc3339());
        
        // Generate JWT token
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(self.token_expiry_hours as i64))
            .expect("valid timestamp")
            .timestamp();
        
        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            roles: user.roles.clone(),
            exp: exp as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret)
        )?;
        
        Ok(AuthResponse {
            token,
            user: user.clone(),
            expires_at: chrono::DateTime::from_timestamp(exp, 0)
                .unwrap()
                .to_rfc3339(),
        })
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &Validation::default()
        )?;
        
        Ok(token_data.claims)
    }
    
    pub fn get_user_by_id(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }
    
    pub fn logout(&self, _token: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement token blacklisting if needed
        Ok(())
    }
}

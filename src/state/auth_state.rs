use dioxus::prelude::*;
use crate::models::ServiceType;
use std::collections::HashMap;

/// Authentication state per service
#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub authenticated_services: Signal<HashMap<ServiceType, bool>>,
    pub tokens: Signal<HashMap<ServiceType, String>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            authenticated_services: Signal::new(HashMap::new()),
            tokens: Signal::new(HashMap::new()),
        }
    }
    
    /// Check if service is authenticated
    pub fn is_authenticated(&self, service: ServiceType) -> bool {
        self.authenticated_services.read().get(&service).copied().unwrap_or(false)
    }
    
    /// Set authentication status for a service
    pub fn set_authenticated(&mut self, service: ServiceType, authenticated: bool) {
        self.authenticated_services.write().insert(service, authenticated);
        if !authenticated {
            self.tokens.write().remove(&service);
        }
    }
    
    /// Store authentication token
    pub fn set_token(&mut self, service: ServiceType, token: String) {
        self.tokens.write().insert(service, token);
        self.set_authenticated(service, true);
    }
    
    /// Get authentication token
    pub fn get_token(&self, service: ServiceType) -> Option<String> {
        self.tokens.read().get(&service).cloned()
    }
    
    /// Logout from service
    pub fn logout(&mut self, service: ServiceType) {
        self.set_authenticated(service, false);
    }
    
    /// Logout from all services
    pub fn logout_all(&mut self) {
        self.authenticated_services.write().clear();
        self.tokens.write().clear();
    }
    
    /// Get list of authenticated services
    pub fn get_authenticated_services(&self) -> Vec<ServiceType> {
        self.authenticated_services
            .read()
            .iter()
            .filter(|(_, &is_auth)| is_auth)
            .map(|(&service, _)| service)
            .collect()
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self::new()
    }
}

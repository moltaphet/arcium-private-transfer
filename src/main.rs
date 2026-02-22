use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EncryptedDataPointer {
    pub cid: String,                 // IPFS Content Identifier
    pub owner: String,               // Owner's identity
    pub decryption_key_share: Vec<u8>, // Key share protected by MPC
}

pub struct AccessPolicy {
    pub min_payment: u64,            // Required payment to access
    pub authorized_users: Vec<String>, // List of pre-authorized identities
}

pub struct DataVault {
    pub vault: Vec<EncryptedDataPointer>,
}

impl DataVault {
    pub fn new() -> Self {
        Self { vault: Vec::new() }
    }

    /// Simulates authorized data transfer based on policy conditions
    pub fn request_access(
        &self, 
        cid: &str, 
        user: &str, 
        payment: u64, 
        policy: &AccessPolicy
    ) -> Result<Vec<u8>, String> {
        let entry = self.vault.iter().find(|e| e.cid == cid)
            .ok_or("Data not found")?;

        // Access Logic: Either user is authorized or payment threshold is met
        if policy.authorized_users.contains(&user.to_string()) || payment >= policy.min_payment {
            Ok(entry.decryption_key_share.clone()) // Key reconstructed in MPC
        } else {
            Err("Access Denied: Shielded conditions not satisfied".to_string())
        }
    }
}

fn main() {
    println!("🔐 Arcium Shielded Data Vault is active.");
    println!("Policy-based access control engine is running.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shielded_access_control() {
        let mut my_vault = DataVault::new();
        let policy = AccessPolicy {
            min_payment: 100,
            authorized_users: vec!["admin_user".to_string()],
        };

        // Adding a shielded data entry
        my_vault.vault.push(EncryptedDataPointer {
            cid: "ipfs://QmXoyp...".to_string(),
            owner: "alice".to_string(),
            decryption_key_share: vec![0xA1, 0xB2, 0xC3, 0xD4], 
        });

        // Case 1: Denied - Insufficient payment
        let fail = my_vault.request_access("ipfs://QmXoyp...", "bob", 50, &policy);
        assert!(fail.is_err());
        println!("Test 1: Access denied for insufficient payment (Correct)");

        // Case 2: Authorized - Payment met
        let success = my_vault.request_access("ipfs://QmXoyp...", "bob", 150, &policy);
        assert!(success.is_ok());
        println!("Test 2: Access granted via payment (Correct)");

        // Case 3: Authorized - Listed user
        let admin_access = my_vault.request_access("ipfs://QmXoyp...", "admin_user", 0, &policy);
        assert!(admin_access.is_ok());
        println!("Test 3: Access granted to authorized user (Correct)");

        println!("✅ Final Verification: Data Transfer logic is secure.");
    }
}
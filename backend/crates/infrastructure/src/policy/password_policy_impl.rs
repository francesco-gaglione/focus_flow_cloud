use domain::traits::password_policy::{PasswordPolicy, PasswordPolicyResult};

#[derive(Default)]
pub struct PasswordPolicyImpl;

impl PasswordPolicyImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordPolicy for PasswordPolicyImpl {
    fn validate(&self, password: &str) -> PasswordPolicyResult<()> {
        let _ = password;
        //TODO define password policy validation logic here
        Ok(())
    }
}

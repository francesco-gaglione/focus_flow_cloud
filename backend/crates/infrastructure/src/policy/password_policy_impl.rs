use domain::traits::password_policy::PasswordPolicy;

#[derive(Default)]
pub struct PasswordPolicyImpl;

impl PasswordPolicyImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordPolicy for PasswordPolicyImpl {
    fn validate(&self, password: &str) -> domain::error::domain_error::DomainResult<()> {
        let _ = password;
        //TODO define password policy validation logic here
        Ok(())
    }
}

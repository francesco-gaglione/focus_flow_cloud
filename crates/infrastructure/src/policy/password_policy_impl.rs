use domain::traits::password_policy::PasswordPolicy;

pub struct PasswordPolicyImpl;

impl PasswordPolicyImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordPolicy for PasswordPolicyImpl {
    fn validate(&self, password: &str) -> domain::error::domain_error::DomainResult<()> {
        //TODO define password policy validation logic here
        Ok(())
    }
}

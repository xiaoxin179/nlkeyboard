#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialKey {
    pub service: String,
    pub account: String,
}

pub trait CredentialStore {
    fn get_secret(&self, key: &CredentialKey) -> anyhow::Result<Option<String>>;
    fn set_secret(&self, key: &CredentialKey, secret: &str) -> anyhow::Result<()>;
    fn delete_secret(&self, key: &CredentialKey) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
pub struct NotImplementedCredentialStore;

impl CredentialStore for NotImplementedCredentialStore {
    fn get_secret(&self, _key: &CredentialKey) -> anyhow::Result<Option<String>> {
        Ok(None)
    }

    fn set_secret(&self, _key: &CredentialKey, _secret: &str) -> anyhow::Result<()> {
        anyhow::bail!("credential storage is not implemented yet")
    }

    fn delete_secret(&self, _key: &CredentialKey) -> anyhow::Result<()> {
        Ok(())
    }
}

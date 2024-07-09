use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderGitlabData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cacert_file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_auth_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

struct ProviderGitlab_ {
    data: RefCell<ProviderGitlabData>,
}

pub struct ProviderGitlab(Rc<ProviderGitlab_>);

impl ProviderGitlab {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "gitlab", alias)
        } else {
            "gitlab".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `base_url`.\nThis is the target GitLab base API endpoint. Providing a value is a requirement when working with GitLab CE or GitLab Enterprise e.g. `https://my.gitlab.server/api/v4/`. It is optional to provide this value and it can also be sourced from the `GITLAB_BASE_URL` environment variable. The value must end with a slash."]
    pub fn set_base_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().base_url = Some(v.into());
        self
    }

    #[doc= "Set the field `cacert_file`.\nThis is a file containing the ca cert to verify the gitlab instance. This is available for use when working with GitLab CE or Gitlab Enterprise with a locally-issued or self-signed certificate chain."]
    pub fn set_cacert_file(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cacert_file = Some(v.into());
        self
    }

    #[doc= "Set the field `client_cert`.\nFile path to client certificate when GitLab instance is behind company proxy. File must contain PEM encoded data."]
    pub fn set_client_cert(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_cert = Some(v.into());
        self
    }

    #[doc= "Set the field `client_key`.\nFile path to client key when GitLab instance is behind company proxy. File must contain PEM encoded data. Required when `client_cert` is set."]
    pub fn set_client_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `early_auth_check`.\n(Experimental) By default the provider does a dummy request to get the current user in order to verify that the provider configuration is correct and the GitLab API is reachable. Set this to `false` to skip this check. This may be useful if the GitLab instance does not yet exist and is created within the same terraform module. It may be sourced from the `GITLAB_EARLY_AUTH_CHECK`. This is an experimental feature and may change in the future. Please make sure to always keep backups of your state."]
    pub fn set_early_auth_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().early_auth_check = Some(v.into());
        self
    }

    #[doc= "Set the field `insecure`.\nWhen set to true this disables SSL verification of the connection to the GitLab instance."]
    pub fn set_insecure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nThe OAuth2 Token, Project, Group, Personal Access Token or CI Job Token used to connect to GitLab. The OAuth method is used in this provider for authentication (using Bearer authorization token). See https://docs.gitlab.com/ee/api/#authentication for details. It may be sourced from the `GITLAB_TOKEN` environment variable."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }
}

impl Provider for ProviderGitlab_ {
    fn extract_type_tf_id(&self) -> String {
        "gitlab".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "gitlabhq/gitlab",
            "version": "17.1.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderGitlab {}

impl BuildProviderGitlab {
    pub fn build(self, stack: &mut Stack) -> ProviderGitlab {
        let out = ProviderGitlab(Rc::new(ProviderGitlab_ { data: RefCell::new(ProviderGitlabData {
            alias: None,
            base_url: core::default::Default::default(),
            cacert_file: core::default::Default::default(),
            client_cert: core::default::Default::default(),
            client_key: core::default::Default::default(),
            early_auth_check: core::default::Default::default(),
            insecure: core::default::Default::default(),
            token: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}

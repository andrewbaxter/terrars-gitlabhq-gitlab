use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct PagesDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_ssl_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct PagesDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PagesDomainData>,
}

#[derive(Clone)]
pub struct PagesDomain(Rc<PagesDomain_>);

impl PagesDomain {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGitlab) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `auto_ssl_enabled`.\nEnables [automatic generation](https://docs.gitlab.com/ee/user/project/pages/custom_domains_ssl_tls_certification/lets_encrypt_integration.html) of SSL certificates issued by Let’s Encrypt for custom domains. When this is set to \"true\", certificate can't be provided."]
    pub fn set_auto_ssl_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_ssl_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate`.\nThe certificate in PEM format with intermediates following in most specific to least specific order."]
    pub fn set_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `expired`.\nWhether the certificate is expired."]
    pub fn set_expired(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().expired = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\nThe certificate key in PEM format."]
    pub fn set_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `auto_ssl_enabled` after provisioning.\nEnables [automatic generation](https://docs.gitlab.com/ee/user/project/pages/custom_domains_ssl_tls_certification/lets_encrypt_integration.html) of SSL certificates issued by Let’s Encrypt for custom domains. When this is set to \"true\", certificate can't be provided."]
    pub fn auto_ssl_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_ssl_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe certificate in PEM format with intermediates following in most specific to least specific order."]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe custom domain indicated by the user."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expired` after provisioning.\nWhether the certificate is expired."]
    pub fn expired(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>:<domain>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe certificate key in PEM format."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding) owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL for the given domain."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verification_code` after provisioning.\nThe verification code for the domain."]
    pub fn verification_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verified` after provisioning.\nThe certificate data."]
    pub fn verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified", self.extract_ref()))
    }
}

impl Referable for PagesDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PagesDomain { }

impl ToListMappable for PagesDomain {
    type O = ListRef<PagesDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PagesDomain_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_pages_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPagesDomain {
    pub tf_id: String,
    #[doc= "The custom domain indicated by the user."]
    pub domain: PrimField<String>,
    #[doc= "The ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding) owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildPagesDomain {
    pub fn build(self, stack: &mut Stack) -> PagesDomain {
        let out = PagesDomain(Rc::new(PagesDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PagesDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_ssl_enabled: core::default::Default::default(),
                certificate: core::default::Default::default(),
                domain: self.domain,
                expired: core::default::Default::default(),
                key: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PagesDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for PagesDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PagesDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_ssl_enabled` after provisioning.\nEnables [automatic generation](https://docs.gitlab.com/ee/user/project/pages/custom_domains_ssl_tls_certification/lets_encrypt_integration.html) of SSL certificates issued by Let’s Encrypt for custom domains. When this is set to \"true\", certificate can't be provided."]
    pub fn auto_ssl_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_ssl_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\nThe certificate in PEM format with intermediates following in most specific to least specific order."]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe custom domain indicated by the user."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expired` after provisioning.\nWhether the certificate is expired."]
    pub fn expired(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>:<domain>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe certificate key in PEM format."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding) owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL for the given domain."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verification_code` after provisioning.\nThe verification code for the domain."]
    pub fn verification_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verification_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verified` after provisioning.\nThe certificate data."]
    pub fn verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified", self.extract_ref()))
    }
}

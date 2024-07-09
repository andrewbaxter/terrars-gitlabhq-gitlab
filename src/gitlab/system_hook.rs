use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct SystemHookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssl_verification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_update_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    url: PrimField<String>,
}

struct SystemHook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SystemHookData>,
}

#[derive(Clone)]
pub struct SystemHook(Rc<SystemHook_>);

impl SystemHook {
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

    #[doc= "Set the field `enable_ssl_verification`.\nDo SSL verification when triggering the hook."]
    pub fn set_enable_ssl_verification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ssl_verification = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_events`.\nTrigger hook on merge requests events."]
    pub fn set_merge_requests_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_events = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nWhen true, the hook fires on push events."]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_update_events`.\nTrigger hook on repository update events."]
    pub fn set_repository_update_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().repository_update_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nWhen true, the hook fires on new tags being pushed."]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nSecret token to validate received payloads; this isn’t returned in the response. This attribute is not available for imported resources."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time the hook was created in ISO8601 format."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nDo SSL verification when triggering the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nTrigger hook on merge requests events."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nWhen true, the hook fires on push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_update_events` after provisioning.\nTrigger hook on repository update events."]
    pub fn repository_update_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_update_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nWhen true, the hook fires on new tags being pushed."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nSecret token to validate received payloads; this isn’t returned in the response. This attribute is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe hook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl Referable for SystemHook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SystemHook { }

impl ToListMappable for SystemHook {
    type O = ListRef<SystemHookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SystemHook_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_system_hook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSystemHook {
    pub tf_id: String,
    #[doc= "The hook URL."]
    pub url: PrimField<String>,
}

impl BuildSystemHook {
    pub fn build(self, stack: &mut Stack) -> SystemHook {
        let out = SystemHook(Rc::new(SystemHook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SystemHookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable_ssl_verification: core::default::Default::default(),
                id: core::default::Default::default(),
                merge_requests_events: core::default::Default::default(),
                push_events: core::default::Default::default(),
                repository_update_events: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
                token: core::default::Default::default(),
                url: self.url,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SystemHookRef {
    shared: StackShared,
    base: String,
}

impl Ref for SystemHookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SystemHookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time the hook was created in ISO8601 format."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nDo SSL verification when triggering the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nTrigger hook on merge requests events."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nWhen true, the hook fires on push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_update_events` after provisioning.\nTrigger hook on repository update events."]
    pub fn repository_update_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_update_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nWhen true, the hook fires on new tags being pushed."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nSecret token to validate received payloads; this isn’t returned in the response. This attribute is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe hook URL."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

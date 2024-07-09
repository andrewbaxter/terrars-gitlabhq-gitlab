use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ServiceEmailsOnPushData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches_to_be_notified: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_diffs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    recipients: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_from_committer_email: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
}

struct ServiceEmailsOnPush_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServiceEmailsOnPushData>,
}

#[derive(Clone)]
pub struct ServiceEmailsOnPush(Rc<ServiceEmailsOnPush_>);

impl ServiceEmailsOnPush {
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

    #[doc= "Set the field `branches_to_be_notified`.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, `default_and_protected`. Notifications are always fired for tag pushes."]
    pub fn set_branches_to_be_notified(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branches_to_be_notified = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_diffs`.\nDisable code diffs."]
    pub fn set_disable_diffs(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_diffs = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nEnable notifications for push events."]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `send_from_committer_email`.\nSend from committer."]
    pub fn set_send_from_committer_email(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().send_from_committer_email = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nEnable notifications for tag push events."]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nWhether the integration is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, `default_and_protected`. Notifications are always fired for tag pushes."]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe ISO8601 date/time that this integration was activated at in UTC."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_diffs` after provisioning.\nDisable code diffs."]
    pub fn disable_diffs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_diffs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID or full-path of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recipients` after provisioning.\nEmails separated by whitespace."]
    pub fn recipients(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `send_from_committer_email` after provisioning.\nSend from committer."]
    pub fn send_from_committer_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.send_from_committer_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nThe name of the integration in lowercase, shortened to 63 bytes, and with everything except 0-9 and a-z replaced with -. No leading / trailing -. Use in URLs, host names and domain names."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle of the integration."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe ISO8601 date/time that this integration was last updated at in UTC."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for ServiceEmailsOnPush {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ServiceEmailsOnPush { }

impl ToListMappable for ServiceEmailsOnPush {
    type O = ListRef<ServiceEmailsOnPushRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServiceEmailsOnPush_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_service_emails_on_push".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServiceEmailsOnPush {
    pub tf_id: String,
    #[doc= "ID or full-path of the project you want to activate integration on."]
    pub project: PrimField<String>,
    #[doc= "Emails separated by whitespace."]
    pub recipients: PrimField<String>,
}

impl BuildServiceEmailsOnPush {
    pub fn build(self, stack: &mut Stack) -> ServiceEmailsOnPush {
        let out = ServiceEmailsOnPush(Rc::new(ServiceEmailsOnPush_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServiceEmailsOnPushData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branches_to_be_notified: core::default::Default::default(),
                disable_diffs: core::default::Default::default(),
                id: core::default::Default::default(),
                project: self.project,
                push_events: core::default::Default::default(),
                recipients: self.recipients,
                send_from_committer_email: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceEmailsOnPushRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceEmailsOnPushRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceEmailsOnPushRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nWhether the integration is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, `default_and_protected`. Notifications are always fired for tag pushes."]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe ISO8601 date/time that this integration was activated at in UTC."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_diffs` after provisioning.\nDisable code diffs."]
    pub fn disable_diffs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_diffs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID or full-path of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recipients` after provisioning.\nEmails separated by whitespace."]
    pub fn recipients(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `send_from_committer_email` after provisioning.\nSend from committer."]
    pub fn send_from_committer_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.send_from_committer_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nThe name of the integration in lowercase, shortened to 63 bytes, and with everything except 0-9 and a-z replaced with -. No leading / trailing -. Use in URLs, host names and domain names."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle of the integration."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe ISO8601 date/time that this integration was last updated at in UTC."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

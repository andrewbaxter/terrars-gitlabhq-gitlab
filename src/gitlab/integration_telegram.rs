use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct IntegrationTelegramData {
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
    confidential_issues_events: PrimField<bool>,
    confidential_note_events: PrimField<bool>,
    issues_events: PrimField<bool>,
    merge_requests_events: PrimField<bool>,
    note_events: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_only_broken_pipelines: Option<PrimField<bool>>,
    pipeline_events: PrimField<bool>,
    project: PrimField<String>,
    push_events: PrimField<bool>,
    room: PrimField<String>,
    tag_push_events: PrimField<bool>,
    token: PrimField<String>,
    wiki_page_events: PrimField<bool>,
}

struct IntegrationTelegram_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationTelegramData>,
}

#[derive(Clone)]
pub struct IntegrationTelegram(Rc<IntegrationTelegram_>);

impl IntegrationTelegram {
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

    #[doc= "Set the field `branches_to_be_notified`.\nBranches to send notifications for (introduced in GitLab 16.5). Update of this attribute was not supported before Gitlab 16.11 due to API bug. Valid options are `all`, `default`, `protected`, `default_and_protected`."]
    pub fn set_branches_to_be_notified(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branches_to_be_notified = Some(v.into());
        self
    }

    #[doc= "Set the field `notify_only_broken_pipelines`.\nSend notifications for broken pipelines."]
    pub fn set_notify_only_broken_pipelines(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().notify_only_broken_pipelines = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for (introduced in GitLab 16.5). Update of this attribute was not supported before Gitlab 16.11 due to API bug. Valid options are `all`, `default`, `protected`, `default_and_protected`."]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nEnable notifications for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nEnable notifications for confidential note events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge requests events."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nSend notifications for broken pipelines."]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project to integrate with Telegram."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `room` after provisioning.\nUnique identifier for the target chat or the username of the target channel (in the format `@channelusername`)"]
    pub fn room(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.room", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe Telegram bot token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nEnable notifications for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

impl Referable for IntegrationTelegram {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IntegrationTelegram { }

impl ToListMappable for IntegrationTelegram {
    type O = ListRef<IntegrationTelegramRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IntegrationTelegram_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_integration_telegram".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationTelegram {
    pub tf_id: String,
    #[doc= "Enable notifications for confidential issues events."]
    pub confidential_issues_events: PrimField<bool>,
    #[doc= "Enable notifications for confidential note events."]
    pub confidential_note_events: PrimField<bool>,
    #[doc= "Enable notifications for issues events."]
    pub issues_events: PrimField<bool>,
    #[doc= "Enable notifications for merge requests events."]
    pub merge_requests_events: PrimField<bool>,
    #[doc= "Enable notifications for note events."]
    pub note_events: PrimField<bool>,
    #[doc= "Enable notifications for pipeline events."]
    pub pipeline_events: PrimField<bool>,
    #[doc= "The ID or full path of the project to integrate with Telegram."]
    pub project: PrimField<String>,
    #[doc= "Enable notifications for push events."]
    pub push_events: PrimField<bool>,
    #[doc= "Unique identifier for the target chat or the username of the target channel (in the format `@channelusername`)"]
    pub room: PrimField<String>,
    #[doc= "Enable notifications for tag push events."]
    pub tag_push_events: PrimField<bool>,
    #[doc= "The Telegram bot token."]
    pub token: PrimField<String>,
    #[doc= "Enable notifications for wiki page events."]
    pub wiki_page_events: PrimField<bool>,
}

impl BuildIntegrationTelegram {
    pub fn build(self, stack: &mut Stack) -> IntegrationTelegram {
        let out = IntegrationTelegram(Rc::new(IntegrationTelegram_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationTelegramData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branches_to_be_notified: core::default::Default::default(),
                confidential_issues_events: self.confidential_issues_events,
                confidential_note_events: self.confidential_note_events,
                issues_events: self.issues_events,
                merge_requests_events: self.merge_requests_events,
                note_events: self.note_events,
                notify_only_broken_pipelines: core::default::Default::default(),
                pipeline_events: self.pipeline_events,
                project: self.project,
                push_events: self.push_events,
                room: self.room,
                tag_push_events: self.tag_push_events,
                token: self.token,
                wiki_page_events: self.wiki_page_events,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationTelegramRef {
    shared: StackShared,
    base: String,
}

impl Ref for IntegrationTelegramRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationTelegramRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for (introduced in GitLab 16.5). Update of this attribute was not supported before Gitlab 16.11 due to API bug. Valid options are `all`, `default`, `protected`, `default_and_protected`."]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nEnable notifications for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nEnable notifications for confidential note events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge requests events."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nSend notifications for broken pipelines."]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project to integrate with Telegram."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `room` after provisioning.\nUnique identifier for the target chat or the username of the target channel (in the format `@channelusername`)"]
    pub fn room(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.room", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe Telegram bot token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nEnable notifications for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

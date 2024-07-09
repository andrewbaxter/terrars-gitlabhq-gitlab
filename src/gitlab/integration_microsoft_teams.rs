use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct IntegrationMicrosoftTeamsData {
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
    confidential_issues_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_note_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_only_broken_pipelines: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_events: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
    webhook: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_page_events: Option<PrimField<bool>>,
}

struct IntegrationMicrosoftTeams_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationMicrosoftTeamsData>,
}

#[derive(Clone)]
pub struct IntegrationMicrosoftTeams(Rc<IntegrationMicrosoftTeams_>);

impl IntegrationMicrosoftTeams {
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

    #[doc= "Set the field `branches_to_be_notified`.\nBranches to send notifications for. Valid options are “all”, “default”, “protected”, and “default_and_protected”. The default value is “default”"]
    pub fn set_branches_to_be_notified(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branches_to_be_notified = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_issues_events`.\nEnable notifications for confidential issue events"]
    pub fn set_confidential_issues_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential_issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_note_events`.\nEnable notifications for confidential note events"]
    pub fn set_confidential_note_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential_note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_events`.\nEnable notifications for issue events"]
    pub fn set_issues_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_events`.\nEnable notifications for merge request events"]
    pub fn set_merge_requests_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_events = Some(v.into());
        self
    }

    #[doc= "Set the field `note_events`.\nEnable notifications for note events"]
    pub fn set_note_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `notify_only_broken_pipelines`.\nSend notifications for broken pipelines"]
    pub fn set_notify_only_broken_pipelines(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().notify_only_broken_pipelines = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_events`.\nEnable notifications for pipeline events"]
    pub fn set_pipeline_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pipeline_events = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nEnable notifications for push events"]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nEnable notifications for tag push events"]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_page_events`.\nEnable notifications for wiki page events"]
    pub fn set_wiki_page_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wiki_page_events = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nWhether the integration is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are “all”, “default”, “protected”, and “default_and_protected”. The default value is “default”"]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nEnable notifications for confidential issue events"]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nEnable notifications for confidential note events"]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issue events"]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge request events"]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events"]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nSend notifications for broken pipelines"]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events"]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events"]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events"]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nUpdate time."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webhook` after provisioning.\nThe Microsoft Teams webhook (Example, https://outlook.office.com/webhook/...). This value cannot be imported."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nEnable notifications for wiki page events"]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

impl Referable for IntegrationMicrosoftTeams {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IntegrationMicrosoftTeams { }

impl ToListMappable for IntegrationMicrosoftTeams {
    type O = ListRef<IntegrationMicrosoftTeamsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IntegrationMicrosoftTeams_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_integration_microsoft_teams".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationMicrosoftTeams {
    pub tf_id: String,
    #[doc= "ID of the project you want to activate integration on."]
    pub project: PrimField<String>,
    #[doc= "The Microsoft Teams webhook (Example, https://outlook.office.com/webhook/...). This value cannot be imported."]
    pub webhook: PrimField<String>,
}

impl BuildIntegrationMicrosoftTeams {
    pub fn build(self, stack: &mut Stack) -> IntegrationMicrosoftTeams {
        let out = IntegrationMicrosoftTeams(Rc::new(IntegrationMicrosoftTeams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationMicrosoftTeamsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branches_to_be_notified: core::default::Default::default(),
                confidential_issues_events: core::default::Default::default(),
                confidential_note_events: core::default::Default::default(),
                id: core::default::Default::default(),
                issues_events: core::default::Default::default(),
                merge_requests_events: core::default::Default::default(),
                note_events: core::default::Default::default(),
                notify_only_broken_pipelines: core::default::Default::default(),
                pipeline_events: core::default::Default::default(),
                project: self.project,
                push_events: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
                webhook: self.webhook,
                wiki_page_events: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationMicrosoftTeamsRef {
    shared: StackShared,
    base: String,
}

impl Ref for IntegrationMicrosoftTeamsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationMicrosoftTeamsRef {
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

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are “all”, “default”, “protected”, and “default_and_protected”. The default value is “default”"]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nEnable notifications for confidential issue events"]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nEnable notifications for confidential note events"]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issue events"]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge request events"]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events"]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nSend notifications for broken pipelines"]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events"]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events"]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events"]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nUpdate time."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webhook` after provisioning.\nThe Microsoft Teams webhook (Example, https://outlook.office.com/webhook/...). This value cannot be imported."]
    pub fn webhook(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nEnable notifications for wiki page events"]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

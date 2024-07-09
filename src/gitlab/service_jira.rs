use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ServiceJiraData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_on_event_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jira_issue_transition_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_events: Option<PrimField<bool>>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_events: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
    url: PrimField<String>,
    username: PrimField<String>,
}

struct ServiceJira_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServiceJiraData>,
}

#[derive(Clone)]
pub struct ServiceJira(Rc<ServiceJira_>);

impl ServiceJira {
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

    #[doc= "Set the field `api_url`.\nThe base URL to the Jira instance API. Web URL value is used if not set. For example, https://jira-api.example.com."]
    pub fn set_api_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_url = Some(v.into());
        self
    }

    #[doc= "Set the field `comment_on_event_enabled`.\nEnable comments inside Jira issues on each GitLab event (commit / merge request)"]
    pub fn set_comment_on_event_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().comment_on_event_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_events`.\nEnable notifications for commit events"]
    pub fn set_commit_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().commit_events = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_events`.\nEnable notifications for issues events."]
    pub fn set_issues_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `jira_issue_transition_id`.\nThe ID of a transition that moves issues to a closed state. You can find this number under the JIRA workflow administration (Administration > Issues > Workflows) by selecting View under Operations of the desired workflow of your project. By default, this ID is set to 2. *Note**: importing this field is only supported since GitLab 15.2."]
    pub fn set_jira_issue_transition_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().jira_issue_transition_id = Some(v.into());
        self
    }

    #[doc= "Set the field `job_events`.\nEnable notifications for job events."]
    pub fn set_job_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().job_events = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_events`.\nEnable notifications for merge request events"]
    pub fn set_merge_requests_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_events = Some(v.into());
        self
    }

    #[doc= "Set the field `note_events`.\nEnable notifications for note events."]
    pub fn set_note_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_events`.\nEnable notifications for pipeline events."]
    pub fn set_pipeline_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pipeline_events = Some(v.into());
        self
    }

    #[doc= "Set the field `project_key`.\nThe short identifier for your JIRA project, all uppercase, e.g., PROJ."]
    pub fn set_project_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_key = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nEnable notifications for push events."]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nEnable notifications for tag_push events."]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nWhether the integration is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to the Jira instance API. Web URL value is used if not set. For example, https://jira-api.example.com."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment_on_event_enabled` after provisioning.\nEnable comments inside Jira issues on each GitLab event (commit / merge request)"]
    pub fn comment_on_event_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment_on_event_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_events` after provisioning.\nEnable notifications for commit events"]
    pub fn commit_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jira_issue_transition_id` after provisioning.\nThe ID of a transition that moves issues to a closed state. You can find this number under the JIRA workflow administration (Administration > Issues > Workflows) by selecting View under Operations of the desired workflow of your project. By default, this ID is set to 2. *Note**: importing this field is only supported since GitLab 15.2."]
    pub fn jira_issue_transition_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jira_issue_transition_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nEnable notifications for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge request events"]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password of the user created to be used with GitLab/JIRA."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_key` after provisioning.\nThe short identifier for your JIRA project, all uppercase, e.g., PROJ."]
    pub fn project_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag_push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nUpdate time."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL to the JIRA project which is being linked to this GitLab project. For example, https://jira.example.com."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user created to be used with GitLab/JIRA."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for ServiceJira {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ServiceJira { }

impl ToListMappable for ServiceJira {
    type O = ListRef<ServiceJiraRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServiceJira_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_service_jira".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServiceJira {
    pub tf_id: String,
    #[doc= "The password of the user created to be used with GitLab/JIRA."]
    pub password: PrimField<String>,
    #[doc= "ID of the project you want to activate integration on."]
    pub project: PrimField<String>,
    #[doc= "The URL to the JIRA project which is being linked to this GitLab project. For example, https://jira.example.com."]
    pub url: PrimField<String>,
    #[doc= "The username of the user created to be used with GitLab/JIRA."]
    pub username: PrimField<String>,
}

impl BuildServiceJira {
    pub fn build(self, stack: &mut Stack) -> ServiceJira {
        let out = ServiceJira(Rc::new(ServiceJira_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServiceJiraData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_url: core::default::Default::default(),
                comment_on_event_enabled: core::default::Default::default(),
                commit_events: core::default::Default::default(),
                id: core::default::Default::default(),
                issues_events: core::default::Default::default(),
                jira_issue_transition_id: core::default::Default::default(),
                job_events: core::default::Default::default(),
                merge_requests_events: core::default::Default::default(),
                note_events: core::default::Default::default(),
                password: self.password,
                pipeline_events: core::default::Default::default(),
                project: self.project,
                project_key: core::default::Default::default(),
                push_events: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
                url: self.url,
                username: self.username,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceJiraRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceJiraRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceJiraRef {
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

    #[doc= "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to the Jira instance API. Web URL value is used if not set. For example, https://jira-api.example.com."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comment_on_event_enabled` after provisioning.\nEnable comments inside Jira issues on each GitLab event (commit / merge request)"]
    pub fn comment_on_event_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment_on_event_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_events` after provisioning.\nEnable notifications for commit events"]
    pub fn commit_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nEnable notifications for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jira_issue_transition_id` after provisioning.\nThe ID of a transition that moves issues to a closed state. You can find this number under the JIRA workflow administration (Administration > Issues > Workflows) by selecting View under Operations of the desired workflow of your project. By default, this ID is set to 2. *Note**: importing this field is only supported since GitLab 15.2."]
    pub fn jira_issue_transition_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jira_issue_transition_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nEnable notifications for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nEnable notifications for merge request events"]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nEnable notifications for note events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password of the user created to be used with GitLab/JIRA."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nEnable notifications for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_key` after provisioning.\nThe short identifier for your JIRA project, all uppercase, e.g., PROJ."]
    pub fn project_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag_push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nUpdate time."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL to the JIRA project which is being linked to this GitLab project. For example, https://jira.example.com."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user created to be used with GitLab/JIRA."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

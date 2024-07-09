use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectHookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_issues_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_note_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_webhook_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssl_verification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_events: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events_branch_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    releases_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_page_events: Option<PrimField<bool>>,
}

struct ProjectHook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectHookData>,
}

#[derive(Clone)]
pub struct ProjectHook(Rc<ProjectHook_>);

impl ProjectHook {
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

    #[doc= "Set the field `confidential_issues_events`.\nInvoke the hook for confidential issues events."]
    pub fn set_confidential_issues_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential_issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_note_events`.\nInvoke the hook for confidential notes events."]
    pub fn set_confidential_note_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential_note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_webhook_template`.\nSet a custom webhook template."]
    pub fn set_custom_webhook_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_webhook_template = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_events`.\nInvoke the hook for deployment events."]
    pub fn set_deployment_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deployment_events = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_ssl_verification`.\nEnable ssl verification when invoking the hook."]
    pub fn set_enable_ssl_verification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ssl_verification = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_events`.\nInvoke the hook for issues events."]
    pub fn set_issues_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `job_events`.\nInvoke the hook for job events."]
    pub fn set_job_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().job_events = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_events`.\nInvoke the hook for merge requests."]
    pub fn set_merge_requests_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_events = Some(v.into());
        self
    }

    #[doc= "Set the field `note_events`.\nInvoke the hook for notes events."]
    pub fn set_note_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_events`.\nInvoke the hook for pipeline events."]
    pub fn set_pipeline_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pipeline_events = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nInvoke the hook for push events."]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events_branch_filter`.\nInvoke the hook for push events on matching branches only."]
    pub fn set_push_events_branch_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().push_events_branch_filter = Some(v.into());
        self
    }

    #[doc= "Set the field `releases_events`.\nInvoke the hook for releases events."]
    pub fn set_releases_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().releases_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nInvoke the hook for tag push events."]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nA token to present when invoking the hook. The token is not available for imported resources."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_page_events`.\nInvoke the hook for wiki page events."]
    pub fn set_wiki_page_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wiki_page_events = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nInvoke the hook for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nInvoke the hook for confidential notes events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_webhook_template` after provisioning.\nSet a custom webhook template."]
    pub fn custom_webhook_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_webhook_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_events` after provisioning.\nInvoke the hook for deployment events."]
    pub fn deployment_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable ssl verification when invoking the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hook_id` after provisioning.\nThe id of the project hook."]
    pub fn hook_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nInvoke the hook for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nInvoke the hook for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nInvoke the hook for merge requests."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nInvoke the hook for notes events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nInvoke the hook for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the hook to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe id of the project for the hook."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nInvoke the hook for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events_branch_filter` after provisioning.\nInvoke the hook for push events on matching branches only."]
    pub fn push_events_branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events_branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_events` after provisioning.\nInvoke the hook for releases events."]
    pub fn releases_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nInvoke the hook for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nA token to present when invoking the hook. The token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe url of the hook to invoke. Forces re-creation to preserve `token`."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nInvoke the hook for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

impl Referable for ProjectHook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectHook { }

impl ToListMappable for ProjectHook {
    type O = ListRef<ProjectHookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectHook_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_hook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectHook {
    pub tf_id: String,
    #[doc= "The name or id of the project to add the hook to."]
    pub project: PrimField<String>,
    #[doc= "The url of the hook to invoke. Forces re-creation to preserve `token`."]
    pub url: PrimField<String>,
}

impl BuildProjectHook {
    pub fn build(self, stack: &mut Stack) -> ProjectHook {
        let out = ProjectHook(Rc::new(ProjectHook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectHookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                confidential_issues_events: core::default::Default::default(),
                confidential_note_events: core::default::Default::default(),
                custom_webhook_template: core::default::Default::default(),
                deployment_events: core::default::Default::default(),
                enable_ssl_verification: core::default::Default::default(),
                id: core::default::Default::default(),
                issues_events: core::default::Default::default(),
                job_events: core::default::Default::default(),
                merge_requests_events: core::default::Default::default(),
                note_events: core::default::Default::default(),
                pipeline_events: core::default::Default::default(),
                project: self.project,
                push_events: core::default::Default::default(),
                push_events_branch_filter: core::default::Default::default(),
                releases_events: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
                token: core::default::Default::default(),
                url: self.url,
                wiki_page_events: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectHookRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectHookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectHookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nInvoke the hook for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nInvoke the hook for confidential notes events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_webhook_template` after provisioning.\nSet a custom webhook template."]
    pub fn custom_webhook_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_webhook_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_events` after provisioning.\nInvoke the hook for deployment events."]
    pub fn deployment_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable ssl verification when invoking the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hook_id` after provisioning.\nThe id of the project hook."]
    pub fn hook_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nInvoke the hook for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nInvoke the hook for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nInvoke the hook for merge requests."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nInvoke the hook for notes events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nInvoke the hook for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the hook to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe id of the project for the hook."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nInvoke the hook for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events_branch_filter` after provisioning.\nInvoke the hook for push events on matching branches only."]
    pub fn push_events_branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events_branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_events` after provisioning.\nInvoke the hook for releases events."]
    pub fn releases_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nInvoke the hook for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nA token to present when invoking the hook. The token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe url of the hook to invoke. Forces re-creation to preserve `token`."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nInvoke the hook for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

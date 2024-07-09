use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectHooksData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct DataProjectHooks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectHooksData>,
}

#[derive(Clone)]
pub struct DataProjectHooks(Rc<DataProjectHooks_>);

impl DataProjectHooks {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGitlab) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `hooks` after provisioning.\nThe list of hooks."]
    pub fn hooks(&self) -> ListRef<DataProjectHooksHooksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for DataProjectHooks {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectHooks { }

impl ToListMappable for DataProjectHooks {
    type O = ListRef<DataProjectHooksRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectHooks_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_hooks".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectHooks {
    pub tf_id: String,
    #[doc= "The name or id of the project."]
    pub project: PrimField<String>,
}

impl BuildDataProjectHooks {
    pub fn build(self, stack: &mut Stack) -> DataProjectHooks {
        let out = DataProjectHooks(Rc::new(DataProjectHooks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectHooksData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectHooksRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectHooksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectHooksRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `hooks` after provisioning.\nThe list of hooks."]
    pub fn hooks(&self) -> ListRef<DataProjectHooksHooksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectHooksHooksEl {
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
    hook_id: Option<PrimField<f64>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<f64>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_page_events: Option<PrimField<bool>>,
}

impl DataProjectHooksHooksEl {
    #[doc= "Set the field `confidential_issues_events`.\n"]
    pub fn set_confidential_issues_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.confidential_issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential_note_events`.\n"]
    pub fn set_confidential_note_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.confidential_note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_webhook_template`.\n"]
    pub fn set_custom_webhook_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_webhook_template = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_events`.\n"]
    pub fn set_deployment_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deployment_events = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_ssl_verification`.\n"]
    pub fn set_enable_ssl_verification(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ssl_verification = Some(v.into());
        self
    }

    #[doc= "Set the field `hook_id`.\n"]
    pub fn set_hook_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hook_id = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_events`.\n"]
    pub fn set_issues_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.issues_events = Some(v.into());
        self
    }

    #[doc= "Set the field `job_events`.\n"]
    pub fn set_job_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.job_events = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_events`.\n"]
    pub fn set_merge_requests_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.merge_requests_events = Some(v.into());
        self
    }

    #[doc= "Set the field `note_events`.\n"]
    pub fn set_note_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.note_events = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_events`.\n"]
    pub fn set_pipeline_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pipeline_events = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\n"]
    pub fn set_push_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events_branch_filter`.\n"]
    pub fn set_push_events_branch_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.push_events_branch_filter = Some(v.into());
        self
    }

    #[doc= "Set the field `releases_events`.\n"]
    pub fn set_releases_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.releases_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\n"]
    pub fn set_tag_push_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tag_push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_page_events`.\n"]
    pub fn set_wiki_page_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wiki_page_events = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectHooksHooksEl {
    type O = BlockAssignable<DataProjectHooksHooksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectHooksHooksEl {}

impl BuildDataProjectHooksHooksEl {
    pub fn build(self) -> DataProjectHooksHooksEl {
        DataProjectHooksHooksEl {
            confidential_issues_events: core::default::Default::default(),
            confidential_note_events: core::default::Default::default(),
            custom_webhook_template: core::default::Default::default(),
            deployment_events: core::default::Default::default(),
            enable_ssl_verification: core::default::Default::default(),
            hook_id: core::default::Default::default(),
            issues_events: core::default::Default::default(),
            job_events: core::default::Default::default(),
            merge_requests_events: core::default::Default::default(),
            note_events: core::default::Default::default(),
            pipeline_events: core::default::Default::default(),
            project: core::default::Default::default(),
            project_id: core::default::Default::default(),
            push_events: core::default::Default::default(),
            push_events_branch_filter: core::default::Default::default(),
            releases_events: core::default::Default::default(),
            tag_push_events: core::default::Default::default(),
            token: core::default::Default::default(),
            url: core::default::Default::default(),
            wiki_page_events: core::default::Default::default(),
        }
    }
}

pub struct DataProjectHooksHooksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectHooksHooksElRef {
    fn new(shared: StackShared, base: String) -> DataProjectHooksHooksElRef {
        DataProjectHooksHooksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectHooksHooksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\n"]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.base))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\n"]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_webhook_template` after provisioning.\n"]
    pub fn custom_webhook_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_webhook_template", self.base))
    }

    #[doc= "Get a reference to the value of field `deployment_events` after provisioning.\n"]
    pub fn deployment_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_events", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\n"]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.base))
    }

    #[doc= "Get a reference to the value of field `hook_id` after provisioning.\n"]
    pub fn hook_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_id", self.base))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\n"]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.base))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\n"]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\n"]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.base))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\n"]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.base))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\n"]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\n"]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.base))
    }

    #[doc= "Get a reference to the value of field `push_events_branch_filter` after provisioning.\n"]
    pub fn push_events_branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events_branch_filter", self.base))
    }

    #[doc= "Get a reference to the value of field `releases_events` after provisioning.\n"]
    pub fn releases_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_events", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\n"]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.base))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\n"]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.base))
    }
}

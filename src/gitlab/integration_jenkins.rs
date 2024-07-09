use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct IntegrationJenkinsData {
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
    jenkins_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_request_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    project: PrimField<String>,
    project_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_push_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

struct IntegrationJenkins_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationJenkinsData>,
}

#[derive(Clone)]
pub struct IntegrationJenkins(Rc<IntegrationJenkins_>);

impl IntegrationJenkins {
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

    #[doc= "Set the field `enable_ssl_verification`.\nEnable SSL verification. Defaults to `true` (enabled)."]
    pub fn set_enable_ssl_verification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ssl_verification = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_request_events`.\nEnable notifications for merge request events."]
    pub fn set_merge_request_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_request_events = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\nPassword for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }

    #[doc= "Set the field `push_events`.\nEnable notifications for push events."]
    pub fn set_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_push_events`.\nEnable notifications for tag push events."]
    pub fn set_tag_push_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tag_push_events = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\nUsername for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nWhether the integration is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable SSL verification. Defaults to `true` (enabled)."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jenkins_url` after provisioning.\nJenkins URL like `http://jenkins.example.com`"]
    pub fn jenkins_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jenkins_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_request_events` after provisioning.\nEnable notifications for merge request events."]
    pub fn merge_request_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_request_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nPassword for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_name` after provisioning.\nThe URL-friendly project name. Example: `my_project_name`."]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for IntegrationJenkins {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IntegrationJenkins { }

impl ToListMappable for IntegrationJenkins {
    type O = ListRef<IntegrationJenkinsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IntegrationJenkins_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_integration_jenkins".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationJenkins {
    pub tf_id: String,
    #[doc= "Jenkins URL like `http://jenkins.example.com`"]
    pub jenkins_url: PrimField<String>,
    #[doc= "ID of the project you want to activate integration on."]
    pub project: PrimField<String>,
    #[doc= "The URL-friendly project name. Example: `my_project_name`."]
    pub project_name: PrimField<String>,
}

impl BuildIntegrationJenkins {
    pub fn build(self, stack: &mut Stack) -> IntegrationJenkins {
        let out = IntegrationJenkins(Rc::new(IntegrationJenkins_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationJenkinsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable_ssl_verification: core::default::Default::default(),
                jenkins_url: self.jenkins_url,
                merge_request_events: core::default::Default::default(),
                password: core::default::Default::default(),
                project: self.project,
                project_name: self.project_name,
                push_events: core::default::Default::default(),
                tag_push_events: core::default::Default::default(),
                username: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationJenkinsRef {
    shared: StackShared,
    base: String,
}

impl Ref for IntegrationJenkinsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationJenkinsRef {
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

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable SSL verification. Defaults to `true` (enabled)."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jenkins_url` after provisioning.\nJenkins URL like `http://jenkins.example.com`"]
    pub fn jenkins_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jenkins_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_request_events` after provisioning.\nEnable notifications for merge request events."]
    pub fn merge_request_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_request_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nPassword for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_name` after provisioning.\nThe URL-friendly project name. Example: `my_project_name`."]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nEnable notifications for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nEnable notifications for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername for authentication with the Jenkins server, if authentication is required by the server."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

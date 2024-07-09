use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct UserRunnerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<f64>>,
    runner_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    untagged: Option<PrimField<bool>>,
}

struct UserRunner_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<UserRunnerData>,
}

#[derive(Clone)]
pub struct UserRunner(Rc<UserRunner_>);

impl UserRunner {
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

    #[doc= "Set the field `access_level`.\nThe access level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn set_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the runner."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group that the runner is created in. Required if runner_type is group_type."]
    pub fn set_group_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `locked`.\nSpecifies if the runner should be locked for the current project."]
    pub fn set_locked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().locked = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_timeout`.\nMaximum timeout that limits the amount of time (in seconds) that runners can run jobs. Must be at least 600 (10 minutes)."]
    pub fn set_maximum_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nSpecifies if the runner should ignore new jobs."]
    pub fn set_paused(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\nThe ID of the project that the runner is created in. Required if runner_type is project_type."]
    pub fn set_project_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_list`.\nA list of runner tags."]
    pub fn set_tag_list(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tag_list = Some(v.into());
        self
    }

    #[doc= "Set the field `untagged`.\nSpecifies if the runner should handle untagged jobs."]
    pub fn set_untagged(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().untagged = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the runner."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group that the runner is created in. Required if runner_type is group_type."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the gitlab runner."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nSpecifies if the runner should be locked for the current project."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_timeout` after provisioning.\nMaximum timeout that limits the amount of time (in seconds) that runners can run jobs. Must be at least 600 (10 minutes)."]
    pub fn maximum_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nSpecifies if the runner should ignore new jobs."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the runner is created in. Required if runner_type is project_type."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runner_type` after provisioning.\nThe scope of the runner. Valid values are: `instance_type`, `group_type`, `project_type`."]
    pub fn runner_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runner_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_list` after provisioning.\nA list of runner tags."]
    pub fn tag_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe authentication token to use when setting up a new runner with this configuration. This value cannot be imported."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `untagged` after provisioning.\nSpecifies if the runner should handle untagged jobs."]
    pub fn untagged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.untagged", self.extract_ref()))
    }
}

impl Referable for UserRunner {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for UserRunner { }

impl ToListMappable for UserRunner {
    type O = ListRef<UserRunnerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for UserRunner_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_user_runner".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUserRunner {
    pub tf_id: String,
    #[doc= "The scope of the runner. Valid values are: `instance_type`, `group_type`, `project_type`."]
    pub runner_type: PrimField<String>,
}

impl BuildUserRunner {
    pub fn build(self, stack: &mut Stack) -> UserRunner {
        let out = UserRunner(Rc::new(UserRunner_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(UserRunnerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: core::default::Default::default(),
                description: core::default::Default::default(),
                group_id: core::default::Default::default(),
                locked: core::default::Default::default(),
                maximum_timeout: core::default::Default::default(),
                paused: core::default::Default::default(),
                project_id: core::default::Default::default(),
                runner_type: self.runner_type,
                tag_list: core::default::Default::default(),
                untagged: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UserRunnerRef {
    shared: StackShared,
    base: String,
}

impl Ref for UserRunnerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UserRunnerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the runner."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group that the runner is created in. Required if runner_type is group_type."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the gitlab runner."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nSpecifies if the runner should be locked for the current project."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_timeout` after provisioning.\nMaximum timeout that limits the amount of time (in seconds) that runners can run jobs. Must be at least 600 (10 minutes)."]
    pub fn maximum_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nSpecifies if the runner should ignore new jobs."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the runner is created in. Required if runner_type is project_type."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runner_type` after provisioning.\nThe scope of the runner. Valid values are: `instance_type`, `group_type`, `project_type`."]
    pub fn runner_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runner_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_list` after provisioning.\nA list of runner tags."]
    pub fn tag_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe authentication token to use when setting up a new runner with this configuration. This value cannot be imported."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `untagged` after provisioning.\nSpecifies if the runner should handle untagged jobs."]
    pub fn untagged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.untagged", self.extract_ref()))
    }
}

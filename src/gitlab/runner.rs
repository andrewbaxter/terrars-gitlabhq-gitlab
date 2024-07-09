use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct RunnerData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
    registration_token: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_untagged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_list: Option<SetField<PrimField<String>>>,
}

struct Runner_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RunnerData>,
}

#[derive(Clone)]
pub struct Runner(Rc<Runner_>);

impl Runner {
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

    #[doc= "Set the field `access_level`.\nThe access_level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn set_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe runner's description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `locked`.\nWhether the runner should be locked for current project."]
    pub fn set_locked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().locked = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_timeout`.\nMaximum timeout set when this runner handles the job."]
    pub fn set_maximum_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nWhether the runner should ignore new jobs."]
    pub fn set_paused(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = Some(v.into());
        self
    }

    #[doc= "Set the field `run_untagged`.\nWhether the runner should handle untagged jobs."]
    pub fn set_run_untagged(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().run_untagged = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_list`.\nList of runner’s tags."]
    pub fn set_tag_list(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tag_list = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access_level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authentication_token` after provisioning.\nThe authentication token used for building a config.toml file. This value is not present when imported."]
    pub fn authentication_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe runner's description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nWhether the runner should be locked for current project."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_timeout` after provisioning.\nMaximum timeout set when this runner handles the job."]
    pub fn maximum_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhether the runner should ignore new jobs."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registration_token` after provisioning.\nThe registration token used to register the runner."]
    pub fn registration_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `run_untagged` after provisioning.\nWhether the runner should handle untagged jobs."]
    pub fn run_untagged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_untagged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of runners to show, one of: online and offline. active and paused are also possible values\n\t\t\t\t              which were deprecated in GitLab 14.8 and will be removed in GitLab 16.0."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_list` after provisioning.\nList of runner’s tags."]
    pub fn tag_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_list", self.extract_ref()))
    }
}

impl Referable for Runner {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Runner { }

impl ToListMappable for Runner {
    type O = ListRef<RunnerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Runner_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_runner".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRunner {
    pub tf_id: String,
    #[doc= "The registration token used to register the runner."]
    pub registration_token: PrimField<String>,
}

impl BuildRunner {
    pub fn build(self, stack: &mut Stack) -> Runner {
        let out = Runner(Rc::new(Runner_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RunnerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                locked: core::default::Default::default(),
                maximum_timeout: core::default::Default::default(),
                paused: core::default::Default::default(),
                registration_token: self.registration_token,
                run_untagged: core::default::Default::default(),
                tag_list: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RunnerRef {
    shared: StackShared,
    base: String,
}

impl Ref for RunnerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RunnerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access_level of the runner. Valid values are: `not_protected`, `ref_protected`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authentication_token` after provisioning.\nThe authentication token used for building a config.toml file. This value is not present when imported."]
    pub fn authentication_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe runner's description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nWhether the runner should be locked for current project."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_timeout` after provisioning.\nMaximum timeout set when this runner handles the job."]
    pub fn maximum_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nWhether the runner should ignore new jobs."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registration_token` after provisioning.\nThe registration token used to register the runner."]
    pub fn registration_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `run_untagged` after provisioning.\nWhether the runner should handle untagged jobs."]
    pub fn run_untagged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_untagged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of runners to show, one of: online and offline. active and paused are also possible values\n\t\t\t\t              which were deprecated in GitLab 14.8 and will be removed in GitLab 16.0."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_list` after provisioning.\nList of runner’s tags."]
    pub fn tag_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_list", self.extract_ref()))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct IntegrationPipelinesEmailData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_only_broken_pipelines: Option<PrimField<bool>>,
    project: PrimField<String>,
    recipients: SetField<PrimField<String>>,
}

struct IntegrationPipelinesEmail_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IntegrationPipelinesEmailData>,
}

#[derive(Clone)]
pub struct IntegrationPipelinesEmail(Rc<IntegrationPipelinesEmail_>);

impl IntegrationPipelinesEmail {
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

    #[doc= "Set the field `branches_to_be_notified`.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, and `default_and_protected`. Default is `default`"]
    pub fn set_branches_to_be_notified(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branches_to_be_notified = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `notify_only_broken_pipelines`.\nNotify only broken pipelines. Default is true."]
    pub fn set_notify_only_broken_pipelines(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().notify_only_broken_pipelines = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, and `default_and_protected`. Default is `default`"]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nNotify only broken pipelines. Default is true."]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recipients` after provisioning.\n) email addresses where notifications are sent."]
    pub fn recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }
}

impl Referable for IntegrationPipelinesEmail {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IntegrationPipelinesEmail { }

impl ToListMappable for IntegrationPipelinesEmail {
    type O = ListRef<IntegrationPipelinesEmailRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IntegrationPipelinesEmail_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_integration_pipelines_email".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationPipelinesEmail {
    pub tf_id: String,
    #[doc= "ID of the project you want to activate integration on."]
    pub project: PrimField<String>,
    #[doc= ") email addresses where notifications are sent."]
    pub recipients: SetField<PrimField<String>>,
}

impl BuildIntegrationPipelinesEmail {
    pub fn build(self, stack: &mut Stack) -> IntegrationPipelinesEmail {
        let out = IntegrationPipelinesEmail(Rc::new(IntegrationPipelinesEmail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IntegrationPipelinesEmailData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branches_to_be_notified: core::default::Default::default(),
                id: core::default::Default::default(),
                notify_only_broken_pipelines: core::default::Default::default(),
                project: self.project,
                recipients: self.recipients,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationPipelinesEmailRef {
    shared: StackShared,
    base: String,
}

impl Ref for IntegrationPipelinesEmailRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationPipelinesEmailRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branches_to_be_notified` after provisioning.\nBranches to send notifications for. Valid options are `all`, `default`, `protected`, and `default_and_protected`. Default is `default`"]
    pub fn branches_to_be_notified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branches_to_be_notified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_only_broken_pipelines` after provisioning.\nNotify only broken pipelines. Default is true."]
    pub fn notify_only_broken_pipelines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_only_broken_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID of the project you want to activate integration on."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recipients` after provisioning.\n) email addresses where notifications are sent."]
    pub fn recipients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectFreezePeriodData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_timezone: Option<PrimField<String>>,
    freeze_end: PrimField<String>,
    freeze_start: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct ProjectFreezePeriod_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectFreezePeriodData>,
}

#[derive(Clone)]
pub struct ProjectFreezePeriod(Rc<ProjectFreezePeriod_>);

impl ProjectFreezePeriod {
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

    #[doc= "Set the field `cron_timezone`.\nThe timezone."]
    pub fn set_cron_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cron_timezone = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cron_timezone` after provisioning.\nThe timezone."]
    pub fn cron_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_timezone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `freeze_end` after provisioning.\nEnd of the Freeze Period in cron format (e.g. `0 2 * * *`)."]
    pub fn freeze_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.freeze_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `freeze_start` after provisioning.\nStart of the Freeze Period in cron format (e.g. `0 1 * * *`)."]
    pub fn freeze_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.freeze_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for ProjectFreezePeriod {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectFreezePeriod { }

impl ToListMappable for ProjectFreezePeriod {
    type O = ListRef<ProjectFreezePeriodRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectFreezePeriod_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_freeze_period".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectFreezePeriod {
    pub tf_id: String,
    #[doc= "End of the Freeze Period in cron format (e.g. `0 2 * * *`)."]
    pub freeze_end: PrimField<String>,
    #[doc= "Start of the Freeze Period in cron format (e.g. `0 1 * * *`)."]
    pub freeze_start: PrimField<String>,
    #[doc= "The ID or URL-encoded path of the project to add the schedule to."]
    pub project: PrimField<String>,
}

impl BuildProjectFreezePeriod {
    pub fn build(self, stack: &mut Stack) -> ProjectFreezePeriod {
        let out = ProjectFreezePeriod(Rc::new(ProjectFreezePeriod_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectFreezePeriodData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cron_timezone: core::default::Default::default(),
                freeze_end: self.freeze_end,
                freeze_start: self.freeze_start,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectFreezePeriodRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectFreezePeriodRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectFreezePeriodRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cron_timezone` after provisioning.\nThe timezone."]
    pub fn cron_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_timezone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `freeze_end` after provisioning.\nEnd of the Freeze Period in cron format (e.g. `0 2 * * *`)."]
    pub fn freeze_end(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.freeze_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `freeze_start` after provisioning.\nStart of the Freeze Period in cron format (e.g. `0 1 * * *`)."]
    pub fn freeze_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.freeze_start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

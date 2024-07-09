use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct PipelineScheduleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    cron: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_timezone: Option<PrimField<String>>,
    description: PrimField<String>,
    project: PrimField<String>,
    #[serde(rename = "ref")]
    ref_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    take_ownership: Option<PrimField<bool>>,
}

struct PipelineSchedule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PipelineScheduleData>,
}

#[derive(Clone)]
pub struct PipelineSchedule(Rc<PipelineSchedule_>);

impl PipelineSchedule {
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

    #[doc= "Set the field `active`.\nThe activation of pipeline schedule. If false is set, the pipeline schedule will deactivated initially."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }

    #[doc= "Set the field `cron_timezone`.\nThe timezone."]
    pub fn set_cron_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cron_timezone = Some(v.into());
        self
    }

    #[doc= "Set the field `take_ownership`.\nWhen set to `true`, the user represented by the token running Terraform will take ownership of the scheduled pipeline prior to editing it. This can help when managing scheduled pipeline drift when other users are making changes outside Terraform."]
    pub fn set_take_ownership(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().take_ownership = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nThe activation of pipeline schedule. If false is set, the pipeline schedule will deactivated initially."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cron` after provisioning.\nThe cron (e.g. `0 1 * * *`)."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cron_timezone` after provisioning.\nThe timezone."]
    pub fn cron_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_timezone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the pipeline schedule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project-id>:<pipeline-schedule-id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nThe ID of the user that owns the pipeline schedule."]
    pub fn owner(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_schedule_id` after provisioning.\nThe pipeline schedule id."]
    pub fn pipeline_schedule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_schedule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe branch/tag name to be triggered. This must be the full branch reference, for example: `refs/heads/main`, not `main`."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `take_ownership` after provisioning.\nWhen set to `true`, the user represented by the token running Terraform will take ownership of the scheduled pipeline prior to editing it. This can help when managing scheduled pipeline drift when other users are making changes outside Terraform."]
    pub fn take_ownership(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.take_ownership", self.extract_ref()))
    }
}

impl Referable for PipelineSchedule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PipelineSchedule { }

impl ToListMappable for PipelineSchedule {
    type O = ListRef<PipelineScheduleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PipelineSchedule_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_pipeline_schedule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPipelineSchedule {
    pub tf_id: String,
    #[doc= "The cron (e.g. `0 1 * * *`)."]
    pub cron: PrimField<String>,
    #[doc= "The description of the pipeline schedule."]
    pub description: PrimField<String>,
    #[doc= "The name or id of the project to add the schedule to."]
    pub project: PrimField<String>,
    #[doc= "The branch/tag name to be triggered. This must be the full branch reference, for example: `refs/heads/main`, not `main`."]
    pub ref_: PrimField<String>,
}

impl BuildPipelineSchedule {
    pub fn build(self, stack: &mut Stack) -> PipelineSchedule {
        let out = PipelineSchedule(Rc::new(PipelineSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PipelineScheduleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active: core::default::Default::default(),
                cron: self.cron,
                cron_timezone: core::default::Default::default(),
                description: self.description,
                project: self.project,
                ref_: self.ref_,
                take_ownership: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PipelineScheduleRef {
    shared: StackShared,
    base: String,
}

impl Ref for PipelineScheduleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PipelineScheduleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nThe activation of pipeline schedule. If false is set, the pipeline schedule will deactivated initially."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cron` after provisioning.\nThe cron (e.g. `0 1 * * *`)."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cron_timezone` after provisioning.\nThe timezone."]
    pub fn cron_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_timezone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the pipeline schedule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project-id>:<pipeline-schedule-id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nThe ID of the user that owns the pipeline schedule."]
    pub fn owner(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_schedule_id` after provisioning.\nThe pipeline schedule id."]
    pub fn pipeline_schedule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_schedule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe branch/tag name to be triggered. This must be the full branch reference, for example: `refs/heads/main`, not `main`."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `take_ownership` after provisioning.\nWhen set to `true`, the user represented by the token running Terraform will take ownership of the scheduled pipeline prior to editing it. This can help when managing scheduled pipeline drift when other users are making changes outside Terraform."]
    pub fn take_ownership(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.take_ownership", self.extract_ref()))
    }
}

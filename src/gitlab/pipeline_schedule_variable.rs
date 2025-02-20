use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct PipelineScheduleVariableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key: PrimField<String>,
    pipeline_schedule_id: PrimField<f64>,
    project: PrimField<String>,
    value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_type: Option<PrimField<String>>,
}

struct PipelineScheduleVariable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PipelineScheduleVariableData>,
}

#[derive(Clone)]
pub struct PipelineScheduleVariable(Rc<PipelineScheduleVariable_>);

impl PipelineScheduleVariable {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `variable_type`.\nThe type of a variable. Available types are: `env_var`, `file`. Default is `env_var`."]
    pub fn set_variable_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().variable_type = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nName of the variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_schedule_id` after provisioning.\nThe id of the pipeline schedule."]
    pub fn pipeline_schedule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_schedule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_type` after provisioning.\nThe type of a variable. Available types are: `env_var`, `file`. Default is `env_var`."]
    pub fn variable_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_type", self.extract_ref()))
    }
}

impl Referable for PipelineScheduleVariable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PipelineScheduleVariable { }

impl ToListMappable for PipelineScheduleVariable {
    type O = ListRef<PipelineScheduleVariableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PipelineScheduleVariable_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_pipeline_schedule_variable".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPipelineScheduleVariable {
    pub tf_id: String,
    #[doc= "Name of the variable."]
    pub key: PrimField<String>,
    #[doc= "The id of the pipeline schedule."]
    pub pipeline_schedule_id: PrimField<f64>,
    #[doc= "The id of the project to add the schedule to."]
    pub project: PrimField<String>,
    #[doc= "Value of the variable."]
    pub value: PrimField<String>,
}

impl BuildPipelineScheduleVariable {
    pub fn build(self, stack: &mut Stack) -> PipelineScheduleVariable {
        let out = PipelineScheduleVariable(Rc::new(PipelineScheduleVariable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PipelineScheduleVariableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                key: self.key,
                pipeline_schedule_id: self.pipeline_schedule_id,
                project: self.project,
                value: self.value,
                variable_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PipelineScheduleVariableRef {
    shared: StackShared,
    base: String,
}

impl Ref for PipelineScheduleVariableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PipelineScheduleVariableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nName of the variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_schedule_id` after provisioning.\nThe id of the pipeline schedule."]
    pub fn pipeline_schedule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_schedule_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project to add the schedule to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_type` after provisioning.\nThe type of a variable. Available types are: `env_var`, `file`. Default is `env_var`."]
    pub fn variable_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_type", self.extract_ref()))
    }
}

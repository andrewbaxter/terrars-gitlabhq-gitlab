use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupVariableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_scope: Option<PrimField<String>>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw: Option<PrimField<bool>>,
    value: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_type: Option<PrimField<String>>,
}

struct GroupVariable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupVariableData>,
}

#[derive(Clone)]
pub struct GroupVariable(Rc<GroupVariable_>);

impl GroupVariable {
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

    #[doc= "Set the field `description`.\nThe description of the variable."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_scope`.\nThe environment scope of the variable. Defaults to all environment (`*`). Note that in Community Editions of Gitlab, values other than `*` will cause inconsistent plans."]
    pub fn set_environment_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environment_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `masked`.\nIf set to `true`, the value of the variable will be hidden in job logs. The value must meet the [masking requirements](https://docs.gitlab.com/ee/ci/variables/#masked-variables). Defaults to `false`."]
    pub fn set_masked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().masked = Some(v.into());
        self
    }

    #[doc= "Set the field `protected`.\nIf set to `true`, the variable will be passed only to pipelines running on protected branches and tags. Defaults to `false`."]
    pub fn set_protected(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().protected = Some(v.into());
        self
    }

    #[doc= "Set the field `raw`.\nWhether the variable is treated as a raw string. Default: false. When true, variables in the value are not expanded."]
    pub fn set_raw(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().raw = Some(v.into());
        self
    }

    #[doc= "Set the field `variable_type`.\nThe type of a variable. Valid values are: `env_var`, `file`. Default is `env_var`."]
    pub fn set_variable_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().variable_type = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the variable."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe environment scope of the variable. Defaults to all environment (`*`). Note that in Community Editions of Gitlab, values other than `*` will cause inconsistent plans."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe name or id of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `masked` after provisioning.\nIf set to `true`, the value of the variable will be hidden in job logs. The value must meet the [masking requirements](https://docs.gitlab.com/ee/ci/variables/#masked-variables). Defaults to `false`."]
    pub fn masked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.masked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nIf set to `true`, the variable will be passed only to pipelines running on protected branches and tags. Defaults to `false`."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw` after provisioning.\nWhether the variable is treated as a raw string. Default: false. When true, variables in the value are not expanded."]
    pub fn raw(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_type` after provisioning.\nThe type of a variable. Valid values are: `env_var`, `file`. Default is `env_var`."]
    pub fn variable_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_type", self.extract_ref()))
    }
}

impl Referable for GroupVariable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupVariable { }

impl ToListMappable for GroupVariable {
    type O = ListRef<GroupVariableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupVariable_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_variable".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupVariable {
    pub tf_id: String,
    #[doc= "The name or id of the group."]
    pub group: PrimField<String>,
    #[doc= "The name of the variable."]
    pub key: PrimField<String>,
    #[doc= "The value of the variable."]
    pub value: PrimField<String>,
}

impl BuildGroupVariable {
    pub fn build(self, stack: &mut Stack) -> GroupVariable {
        let out = GroupVariable(Rc::new(GroupVariable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupVariableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                environment_scope: core::default::Default::default(),
                group: self.group,
                id: core::default::Default::default(),
                key: self.key,
                masked: core::default::Default::default(),
                protected: core::default::Default::default(),
                raw: core::default::Default::default(),
                value: self.value,
                variable_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupVariableRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupVariableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupVariableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the variable."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe environment scope of the variable. Defaults to all environment (`*`). Note that in Community Editions of Gitlab, values other than `*` will cause inconsistent plans."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe name or id of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `masked` after provisioning.\nIf set to `true`, the value of the variable will be hidden in job logs. The value must meet the [masking requirements](https://docs.gitlab.com/ee/ci/variables/#masked-variables). Defaults to `false`."]
    pub fn masked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.masked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nIf set to `true`, the variable will be passed only to pipelines running on protected branches and tags. Defaults to `false`."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw` after provisioning.\nWhether the variable is treated as a raw string. Default: false. When true, variables in the value are not expanded."]
    pub fn raw(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_type` after provisioning.\nThe type of a variable. Valid values are: `env_var`, `file`. Default is `env_var`."]
    pub fn variable_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_type", self.extract_ref()))
    }
}

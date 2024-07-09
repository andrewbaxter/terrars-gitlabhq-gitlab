use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataInstanceVariableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key: PrimField<String>,
}

struct DataInstanceVariable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataInstanceVariableData>,
}

#[derive(Clone)]
pub struct DataInstanceVariable(Rc<DataInstanceVariable_>);

impl DataInstanceVariable {
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

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the variable. Maximum of 255 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

impl Referable for DataInstanceVariable {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataInstanceVariable { }

impl ToListMappable for DataInstanceVariable {
    type O = ListRef<DataInstanceVariableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataInstanceVariable_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_instance_variable".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataInstanceVariable {
    pub tf_id: String,
    #[doc= "The name of the variable."]
    pub key: PrimField<String>,
}

impl BuildDataInstanceVariable {
    pub fn build(self, stack: &mut Stack) -> DataInstanceVariable {
        let out = DataInstanceVariable(Rc::new(DataInstanceVariable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataInstanceVariableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                key: self.key,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataInstanceVariableRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceVariableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataInstanceVariableRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the variable. Maximum of 255 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectVariablesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct DataProjectVariables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectVariablesData>,
}

#[derive(Clone)]
pub struct DataProjectVariables(Rc<DataProjectVariables_>);

impl DataProjectVariables {
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

    #[doc= "Set the field `environment_scope`.\nThe environment scope of the variable. Defaults to all environment (`*`)."]
    pub fn set_environment_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environment_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe environment scope of the variable. Defaults to all environment (`*`)."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\nThe list of variables returned by the search"]
    pub fn variables(&self) -> ListRef<DataProjectVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

impl Referable for DataProjectVariables {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectVariables { }

impl ToListMappable for DataProjectVariables {
    type O = ListRef<DataProjectVariablesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectVariables_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_variables".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectVariables {
    pub tf_id: String,
    #[doc= "The name or id of the project."]
    pub project: PrimField<String>,
}

impl BuildDataProjectVariables {
    pub fn build(self, stack: &mut Stack) -> DataProjectVariables {
        let out = DataProjectVariables(Rc::new(DataProjectVariables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectVariablesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                environment_scope: core::default::Default::default(),
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectVariablesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectVariablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectVariablesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe environment scope of the variable. Defaults to all environment (`*`)."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\nThe list of variables returned by the search"]
    pub fn variables(&self) -> ListRef<DataProjectVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectVariablesVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_type: Option<PrimField<String>>,
}

impl DataProjectVariablesVariablesEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_scope`.\n"]
    pub fn set_environment_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `masked`.\n"]
    pub fn set_masked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.masked = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `protected`.\n"]
    pub fn set_protected(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protected = Some(v.into());
        self
    }

    #[doc= "Set the field `raw`.\n"]
    pub fn set_raw(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.raw = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `variable_type`.\n"]
    pub fn set_variable_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.variable_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectVariablesVariablesEl {
    type O = BlockAssignable<DataProjectVariablesVariablesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectVariablesVariablesEl {}

impl BuildDataProjectVariablesVariablesEl {
    pub fn build(self) -> DataProjectVariablesVariablesEl {
        DataProjectVariablesVariablesEl {
            description: core::default::Default::default(),
            environment_scope: core::default::Default::default(),
            key: core::default::Default::default(),
            masked: core::default::Default::default(),
            project: core::default::Default::default(),
            protected: core::default::Default::default(),
            raw: core::default::Default::default(),
            value: core::default::Default::default(),
            variable_type: core::default::Default::default(),
        }
    }
}

pub struct DataProjectVariablesVariablesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectVariablesVariablesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectVariablesVariablesElRef {
        DataProjectVariablesVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectVariablesVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\n"]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `masked` after provisioning.\n"]
    pub fn masked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.masked", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\n"]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.base))
    }

    #[doc= "Get a reference to the value of field `raw` after provisioning.\n"]
    pub fn raw(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `variable_type` after provisioning.\n"]
    pub fn variable_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_type", self.base))
    }
}

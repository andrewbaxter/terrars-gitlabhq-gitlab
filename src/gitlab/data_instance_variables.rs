use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataInstanceVariablesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataInstanceVariables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataInstanceVariablesData>,
}

#[derive(Clone)]
pub struct DataInstanceVariables(Rc<DataInstanceVariables_>);

impl DataInstanceVariables {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\nThe list of variables returned by the search"]
    pub fn variables(&self) -> ListRef<DataInstanceVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

impl Referable for DataInstanceVariables {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataInstanceVariables { }

impl ToListMappable for DataInstanceVariables {
    type O = ListRef<DataInstanceVariablesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataInstanceVariables_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_instance_variables".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataInstanceVariables {
    pub tf_id: String,
}

impl BuildDataInstanceVariables {
    pub fn build(self, stack: &mut Stack) -> DataInstanceVariables {
        let out = DataInstanceVariables(Rc::new(DataInstanceVariables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataInstanceVariablesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataInstanceVariablesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceVariablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataInstanceVariablesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\nThe list of variables returned by the search"]
    pub fn variables(&self) -> ListRef<DataInstanceVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataInstanceVariablesVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    masked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_type: Option<PrimField<String>>,
}

impl DataInstanceVariablesVariablesEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
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

impl ToListMappable for DataInstanceVariablesVariablesEl {
    type O = BlockAssignable<DataInstanceVariablesVariablesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceVariablesVariablesEl {}

impl BuildDataInstanceVariablesVariablesEl {
    pub fn build(self) -> DataInstanceVariablesVariablesEl {
        DataInstanceVariablesVariablesEl {
            description: core::default::Default::default(),
            key: core::default::Default::default(),
            masked: core::default::Default::default(),
            protected: core::default::Default::default(),
            raw: core::default::Default::default(),
            value: core::default::Default::default(),
            variable_type: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceVariablesVariablesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceVariablesVariablesElRef {
    fn new(shared: StackShared, base: String) -> DataInstanceVariablesVariablesElRef {
        DataInstanceVariablesVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceVariablesVariablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `masked` after provisioning.\n"]
    pub fn masked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.masked", self.base))
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

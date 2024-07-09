use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataClusterAgentsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct DataClusterAgents_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataClusterAgentsData>,
}

#[derive(Clone)]
pub struct DataClusterAgents(Rc<DataClusterAgents_>);

impl DataClusterAgents {
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

    #[doc= "Get a reference to the value of field `cluster_agents` after provisioning.\nList of the registered agents."]
    pub fn cluster_agents(&self) -> ListRef<DataClusterAgentsClusterAgentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_agents", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for DataClusterAgents {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataClusterAgents { }

impl ToListMappable for DataClusterAgents {
    type O = ListRef<DataClusterAgentsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataClusterAgents_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_cluster_agents".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataClusterAgents {
    pub tf_id: String,
    #[doc= "The ID or full path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildDataClusterAgents {
    pub fn build(self, stack: &mut Stack) -> DataClusterAgents {
        let out = DataClusterAgents(Rc::new(DataClusterAgents_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataClusterAgentsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataClusterAgentsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataClusterAgentsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataClusterAgentsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `cluster_agents` after provisioning.\nList of the registered agents."]
    pub fn cluster_agents(&self) -> ListRef<DataClusterAgentsClusterAgentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_agents", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataClusterAgentsClusterAgentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by_user_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

impl DataClusterAgentsClusterAgentsEl {
    #[doc= "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.agent_id = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `created_by_user_id`.\n"]
    pub fn set_created_by_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.created_by_user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}

impl ToListMappable for DataClusterAgentsClusterAgentsEl {
    type O = BlockAssignable<DataClusterAgentsClusterAgentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataClusterAgentsClusterAgentsEl {}

impl BuildDataClusterAgentsClusterAgentsEl {
    pub fn build(self) -> DataClusterAgentsClusterAgentsEl {
        DataClusterAgentsClusterAgentsEl {
            agent_id: core::default::Default::default(),
            created_at: core::default::Default::default(),
            created_by_user_id: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct DataClusterAgentsClusterAgentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataClusterAgentsClusterAgentsElRef {
    fn new(shared: StackShared, base: String) -> DataClusterAgentsClusterAgentsElRef {
        DataClusterAgentsClusterAgentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataClusterAgentsClusterAgentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `created_by_user_id` after provisioning.\n"]
    pub fn created_by_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_by_user_id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

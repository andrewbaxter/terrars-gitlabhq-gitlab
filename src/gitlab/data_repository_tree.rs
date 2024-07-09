use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataRepositoryTreeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<PrimField<bool>>,
    #[serde(rename = "ref")]
    ref_: PrimField<String>,
}

struct DataRepositoryTree_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryTreeData>,
}

#[derive(Clone)]
pub struct DataRepositoryTree(Rc<DataRepositoryTree_>);

impl DataRepositoryTree {
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

    #[doc= "Set the field `path`.\nThe path inside repository. Used to get content of subdirectories."]
    pub fn set_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path = Some(v.into());
        self
    }

    #[doc= "Set the field `recursive`.\nBoolean value used to get a recursive tree (false by default)."]
    pub fn set_recursive(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recursive = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path inside repository. Used to get content of subdirectories."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recursive` after provisioning.\nBoolean value used to get a recursive tree (false by default)."]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of a repository branch or tag."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tree` after provisioning.\nThe list of files/directories returned by the search"]
    pub fn tree(&self) -> ListRef<DataRepositoryTreeTreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tree", self.extract_ref()))
    }
}

impl Referable for DataRepositoryTree {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryTree { }

impl ToListMappable for DataRepositoryTree {
    type O = ListRef<DataRepositoryTreeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryTree_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_repository_tree".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryTree {
    pub tf_id: String,
    #[doc= "The ID or full path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
    #[doc= "The name of a repository branch or tag."]
    pub ref_: PrimField<String>,
}

impl BuildDataRepositoryTree {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryTree {
        let out = DataRepositoryTree(Rc::new(DataRepositoryTree_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryTreeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                path: core::default::Default::default(),
                project: self.project,
                recursive: core::default::Default::default(),
                ref_: self.ref_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryTreeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryTreeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryTreeRef {
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

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path inside repository. Used to get content of subdirectories."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recursive` after provisioning.\nBoolean value used to get a recursive tree (false by default)."]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of a repository branch or tag."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tree` after provisioning.\nThe list of files/directories returned by the search"]
    pub fn tree(&self) -> ListRef<DataRepositoryTreeTreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tree", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryTreeTreeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataRepositoryTreeTreeEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryTreeTreeEl {
    type O = BlockAssignable<DataRepositoryTreeTreeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryTreeTreeEl {}

impl BuildDataRepositoryTreeTreeEl {
    pub fn build(self) -> DataRepositoryTreeTreeEl {
        DataRepositoryTreeTreeEl {
            id: core::default::Default::default(),
            mode: core::default::Default::default(),
            name: core::default::Default::default(),
            path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryTreeTreeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryTreeTreeElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryTreeTreeElRef {
        DataRepositoryTreeTreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryTreeTreeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

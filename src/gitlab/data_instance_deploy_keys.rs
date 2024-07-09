use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataInstanceDeployKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<PrimField<bool>>,
}

struct DataInstanceDeployKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataInstanceDeployKeysData>,
}

#[derive(Clone)]
pub struct DataInstanceDeployKeys(Rc<DataInstanceDeployKeys_>);

impl DataInstanceDeployKeys {
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

    #[doc= "Set the field `public`.\nOnly return deploy keys that are public."]
    pub fn set_public(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `deploy_keys` after provisioning.\nThe list of all deploy keys across all projects of the GitLab instance."]
    pub fn deploy_keys(&self) -> ListRef<DataInstanceDeployKeysDeployKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public` after provisioning.\nOnly return deploy keys that are public."]
    pub fn public(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public", self.extract_ref()))
    }
}

impl Referable for DataInstanceDeployKeys {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataInstanceDeployKeys { }

impl ToListMappable for DataInstanceDeployKeys {
    type O = ListRef<DataInstanceDeployKeysRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataInstanceDeployKeys_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_instance_deploy_keys".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataInstanceDeployKeys {
    pub tf_id: String,
}

impl BuildDataInstanceDeployKeys {
    pub fn build(self, stack: &mut Stack) -> DataInstanceDeployKeys {
        let out = DataInstanceDeployKeys(Rc::new(DataInstanceDeployKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataInstanceDeployKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                public: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataInstanceDeployKeysRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceDeployKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataInstanceDeployKeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `deploy_keys` after provisioning.\nThe list of all deploy keys across all projects of the GitLab instance."]
    pub fn deploy_keys(&self) -> ListRef<DataInstanceDeployKeysDeployKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public` after provisioning.\nOnly return deploy keys that are public."]
    pub fn public(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_with_namespace: Option<PrimField<String>>,
}

impl DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_with_namespace`.\n"]
    pub fn set_name_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `path_with_namespace`.\n"]
    pub fn set_path_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_with_namespace = Some(v.into());
        self
    }
}

impl ToListMappable for DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
    type O = BlockAssignable<DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {}

impl BuildDataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
    pub fn build(self) -> DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
        DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            name_with_namespace: core::default::Default::default(),
            path: core::default::Default::default(),
            path_with_namespace: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef {
    fn new(shared: StackShared, base: String) -> DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef {
        DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `name_with_namespace` after provisioning.\n"]
    pub fn name_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_with_namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\n"]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceDeployKeysDeployKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fingerprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects_with_write_access: Option<ListField<DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

impl DataInstanceDeployKeysDeployKeysEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `fingerprint`.\n"]
    pub fn set_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fingerprint = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `projects_with_write_access`.\n"]
    pub fn set_projects_with_write_access(
        mut self,
        v: impl Into<ListField<DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessEl>>,
    ) -> Self {
        self.projects_with_write_access = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl ToListMappable for DataInstanceDeployKeysDeployKeysEl {
    type O = BlockAssignable<DataInstanceDeployKeysDeployKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceDeployKeysDeployKeysEl {}

impl BuildDataInstanceDeployKeysDeployKeysEl {
    pub fn build(self) -> DataInstanceDeployKeysDeployKeysEl {
        DataInstanceDeployKeysDeployKeysEl {
            created_at: core::default::Default::default(),
            fingerprint: core::default::Default::default(),
            id: core::default::Default::default(),
            key: core::default::Default::default(),
            projects_with_write_access: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceDeployKeysDeployKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstanceDeployKeysDeployKeysElRef {
    fn new(shared: StackShared, base: String) -> DataInstanceDeployKeysDeployKeysElRef {
        DataInstanceDeployKeysDeployKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceDeployKeysDeployKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\n"]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `projects_with_write_access` after provisioning.\n"]
    pub fn projects_with_write_access(&self) -> ListRef<DataInstanceDeployKeysDeployKeysElProjectsWithWriteAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.projects_with_write_access", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

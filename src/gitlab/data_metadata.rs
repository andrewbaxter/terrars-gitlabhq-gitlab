use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataMetadataData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
}

struct DataMetadata_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMetadataData>,
}

#[derive(Clone)]
pub struct DataMetadata(Rc<DataMetadata_>);

impl DataMetadata {
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

    #[doc= "Get a reference to the value of field `enterprise` after provisioning.\nIf the GitLab instance is an enterprise instance or not. Supported for GitLab 15.6 onwards."]
    pub fn enterprise(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enterprise", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe id of the data source. It will always be `1`"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kas` after provisioning.\nMetadata about the GitLab agent server for Kubernetes (KAS)."]
    pub fn kas(&self) -> DataMetadataKasRef {
        DataMetadataKasRef::new(self.shared().clone(), format!("{}.kas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nRevision of the GitLab instance."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the GitLab instance."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl Referable for DataMetadata {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataMetadata { }

impl ToListMappable for DataMetadata {
    type O = ListRef<DataMetadataRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMetadata_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_metadata".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMetadata {
    pub tf_id: String,
}

impl BuildDataMetadata {
    pub fn build(self, stack: &mut Stack) -> DataMetadata {
        let out = DataMetadata(Rc::new(DataMetadata_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMetadataData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMetadataRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMetadataRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMetadataRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `enterprise` after provisioning.\nIf the GitLab instance is an enterprise instance or not. Supported for GitLab 15.6 onwards."]
    pub fn enterprise(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enterprise", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe id of the data source. It will always be `1`"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kas` after provisioning.\nMetadata about the GitLab agent server for Kubernetes (KAS)."]
    pub fn kas(&self) -> DataMetadataKasRef {
        DataMetadataKasRef::new(self.shared().clone(), format!("{}.kas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nRevision of the GitLab instance."]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the GitLab instance."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMetadataKas {}

impl DataMetadataKas { }

impl ToListMappable for DataMetadataKas {
    type O = BlockAssignable<DataMetadataKas>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMetadataKas {}

impl BuildDataMetadataKas {
    pub fn build(self) -> DataMetadataKas {
        DataMetadataKas {}
    }
}

pub struct DataMetadataKasRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMetadataKasRef {
    fn new(shared: StackShared, base: String) -> DataMetadataKasRef {
        DataMetadataKasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMetadataKasRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether KAS is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `external_url` after provisioning.\nURL used by the agents to communicate with KAS. It’s null if kas.enabled is false."]
    pub fn external_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_url", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of KAS. It’s null if kas.enabled is false."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

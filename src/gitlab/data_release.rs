use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataReleaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    project_id: PrimField<String>,
    tag_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<DataReleaseAssetsEl>,
}

struct DataRelease_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataReleaseData>,
}

#[derive(Clone)]
pub struct DataRelease(Rc<DataRelease_>);

impl DataRelease {
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

    #[doc= "Set the field `assets`.\n"]
    pub fn set_assets(self, v: impl Into<DataReleaseAssetsEl>) -> Self {
        self.0.data.borrow_mut().assets = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date the release was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn HTML rendered description of the release."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project_id:tag_name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID or URL-encoded path of the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `released_at` after provisioning.\nThe date the release was created."]
    pub fn released_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.released_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe Git tag the release is associated with."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets` after provisioning.\n"]
    pub fn assets(&self) -> DataReleaseAssetsElRef {
        DataReleaseAssetsElRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }
}

impl Referable for DataRelease {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRelease { }

impl ToListMappable for DataRelease {
    type O = ListRef<DataReleaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRelease_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_release".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRelease {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the project."]
    pub project_id: PrimField<String>,
    #[doc= "The Git tag the release is associated with."]
    pub tag_name: PrimField<String>,
}

impl BuildDataRelease {
    pub fn build(self, stack: &mut Stack) -> DataRelease {
        let out = DataRelease(Rc::new(DataRelease_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataReleaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                project_id: self.project_id,
                tag_name: self.tag_name,
                assets: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataReleaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataReleaseRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date the release was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn HTML rendered description of the release."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project_id:tag_name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID or URL-encoded path of the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `released_at` after provisioning.\nThe date the release was created."]
    pub fn released_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.released_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe Git tag the release is associated with."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets` after provisioning.\n"]
    pub fn assets(&self) -> DataReleaseAssetsElRef {
        DataReleaseAssetsElRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataReleaseAssetsElLinksEl {}

impl DataReleaseAssetsElLinksEl { }

impl ToListMappable for DataReleaseAssetsElLinksEl {
    type O = BlockAssignable<DataReleaseAssetsElLinksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataReleaseAssetsElLinksEl {}

impl BuildDataReleaseAssetsElLinksEl {
    pub fn build(self) -> DataReleaseAssetsElLinksEl {
        DataReleaseAssetsElLinksEl {}
    }
}

pub struct DataReleaseAssetsElLinksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseAssetsElLinksElRef {
    fn new(shared: StackShared, base: String) -> DataReleaseAssetsElLinksElRef {
        DataReleaseAssetsElLinksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataReleaseAssetsElLinksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the link"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `link_type` after provisioning.\nThe type of the link"]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_type", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the link"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the link"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataReleaseAssetsElSourcesEl {}

impl DataReleaseAssetsElSourcesEl { }

impl ToListMappable for DataReleaseAssetsElSourcesEl {
    type O = BlockAssignable<DataReleaseAssetsElSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataReleaseAssetsElSourcesEl {}

impl BuildDataReleaseAssetsElSourcesEl {
    pub fn build(self) -> DataReleaseAssetsElSourcesEl {
        DataReleaseAssetsElSourcesEl {}
    }
}

pub struct DataReleaseAssetsElSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseAssetsElSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataReleaseAssetsElSourcesElRef {
        DataReleaseAssetsElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataReleaseAssetsElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `format` after provisioning.\nThe format of the source"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the source"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataReleaseAssetsElDynamic {
    links: Option<DynamicBlock<DataReleaseAssetsElLinksEl>>,
    sources: Option<DynamicBlock<DataReleaseAssetsElSourcesEl>>,
}

#[derive(Serialize)]
pub struct DataReleaseAssetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Vec<DataReleaseAssetsElLinksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<DataReleaseAssetsElSourcesEl>>,
    dynamic: DataReleaseAssetsElDynamic,
}

impl DataReleaseAssetsEl {
    #[doc= "Set the field `links`.\n"]
    pub fn set_links(mut self, v: impl Into<BlockAssignable<DataReleaseAssetsElLinksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.links = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.links = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sources`.\n"]
    pub fn set_sources(mut self, v: impl Into<BlockAssignable<DataReleaseAssetsElSourcesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataReleaseAssetsEl {
    type O = BlockAssignable<DataReleaseAssetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataReleaseAssetsEl {}

impl BuildDataReleaseAssetsEl {
    pub fn build(self) -> DataReleaseAssetsEl {
        DataReleaseAssetsEl {
            links: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataReleaseAssetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseAssetsElRef {
    fn new(shared: StackShared, base: String) -> DataReleaseAssetsElRef {
        DataReleaseAssetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataReleaseAssetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\nThe number of assets for a release"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(&self) -> ListRef<DataReleaseAssetsElLinksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }

    #[doc= "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<DataReleaseAssetsElSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

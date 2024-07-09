use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataReleaseLinksData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
    tag_name: PrimField<String>,
}

struct DataReleaseLinks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataReleaseLinksData>,
}

#[derive(Clone)]
pub struct DataReleaseLinks(Rc<DataReleaseLinks_>);

impl DataReleaseLinks {
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

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path to the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_links` after provisioning.\nList of release links"]
    pub fn release_links(&self) -> ListRef<DataReleaseLinksReleaseLinksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.release_links", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe tag associated with the Release."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }
}

impl Referable for DataReleaseLinks {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataReleaseLinks { }

impl ToListMappable for DataReleaseLinks {
    type O = ListRef<DataReleaseLinksRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataReleaseLinks_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_release_links".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataReleaseLinks {
    pub tf_id: String,
    #[doc= "The ID or full path to the project."]
    pub project: PrimField<String>,
    #[doc= "The tag associated with the Release."]
    pub tag_name: PrimField<String>,
}

impl BuildDataReleaseLinks {
    pub fn build(self, stack: &mut Stack) -> DataReleaseLinks {
        let out = DataReleaseLinks(Rc::new(DataReleaseLinks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataReleaseLinksData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: self.project,
                tag_name: self.tag_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataReleaseLinksRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseLinksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataReleaseLinksRef {
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

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path to the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_links` after provisioning.\nList of release links"]
    pub fn release_links(&self) -> ListRef<DataReleaseLinksReleaseLinksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.release_links", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe tag associated with the Release."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataReleaseLinksReleaseLinksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_asset_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filepath: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl DataReleaseLinksReleaseLinksEl {
    #[doc= "Set the field `direct_asset_url`.\n"]
    pub fn set_direct_asset_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direct_asset_url = Some(v.into());
        self
    }

    #[doc= "Set the field `external`.\n"]
    pub fn set_external(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.external = Some(v.into());
        self
    }

    #[doc= "Set the field `filepath`.\n"]
    pub fn set_filepath(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filepath = Some(v.into());
        self
    }

    #[doc= "Set the field `link_id`.\n"]
    pub fn set_link_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.link_id = Some(v.into());
        self
    }

    #[doc= "Set the field `link_type`.\n"]
    pub fn set_link_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.link_type = Some(v.into());
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

    #[doc= "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for DataReleaseLinksReleaseLinksEl {
    type O = BlockAssignable<DataReleaseLinksReleaseLinksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataReleaseLinksReleaseLinksEl {}

impl BuildDataReleaseLinksReleaseLinksEl {
    pub fn build(self) -> DataReleaseLinksReleaseLinksEl {
        DataReleaseLinksReleaseLinksEl {
            direct_asset_url: core::default::Default::default(),
            external: core::default::Default::default(),
            filepath: core::default::Default::default(),
            link_id: core::default::Default::default(),
            link_type: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            tag_name: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataReleaseLinksReleaseLinksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseLinksReleaseLinksElRef {
    fn new(shared: StackShared, base: String) -> DataReleaseLinksReleaseLinksElRef {
        DataReleaseLinksReleaseLinksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataReleaseLinksReleaseLinksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `direct_asset_url` after provisioning.\n"]
    pub fn direct_asset_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_asset_url", self.base))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\n"]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.base))
    }

    #[doc= "Get a reference to the value of field `filepath` after provisioning.\n"]
    pub fn filepath(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filepath", self.base))
    }

    #[doc= "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.base))
    }

    #[doc= "Get a reference to the value of field `link_type` after provisioning.\n"]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_type", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataReleaseLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    link_id: PrimField<f64>,
    project: PrimField<String>,
    tag_name: PrimField<String>,
}

struct DataReleaseLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataReleaseLinkData>,
}

#[derive(Clone)]
pub struct DataReleaseLink(Rc<DataReleaseLink_>);

impl DataReleaseLink {
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

    #[doc= "Get a reference to the value of field `direct_asset_url` after provisioning.\nFull path for a [Direct Asset link](https://docs.gitlab.com/ee/user/project/releases/index.html#permanent-links-to-release-assets)."]
    pub fn direct_asset_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_asset_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\nExternal or internal link."]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filepath` after provisioning.\nRelative path for a [Direct Asset link](https://docs.gitlab.com/ee/user/project/releases/index.html#permanent-links-to-release-assets)."]
    pub fn filepath(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filepath", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_id` after provisioning.\nThe ID of the link."]
    pub fn link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_type` after provisioning.\nThe type of the link. Valid values are `other`, `runbook`, `image`, `package`. Defaults to other."]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the link. Link names must be unique within the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding)."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe tag associated with the Release."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the link. Link URLs must be unique within the release."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl Referable for DataReleaseLink {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataReleaseLink { }

impl ToListMappable for DataReleaseLink {
    type O = ListRef<DataReleaseLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataReleaseLink_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_release_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataReleaseLink {
    pub tf_id: String,
    #[doc= "The ID of the link."]
    pub link_id: PrimField<f64>,
    #[doc= "The ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding)."]
    pub project: PrimField<String>,
    #[doc= "The tag associated with the Release."]
    pub tag_name: PrimField<String>,
}

impl BuildDataReleaseLink {
    pub fn build(self, stack: &mut Stack) -> DataReleaseLink {
        let out = DataReleaseLink(Rc::new(DataReleaseLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataReleaseLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                link_id: self.link_id,
                project: self.project,
                tag_name: self.tag_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataReleaseLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataReleaseLinkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `direct_asset_url` after provisioning.\nFull path for a [Direct Asset link](https://docs.gitlab.com/ee/user/project/releases/index.html#permanent-links-to-release-assets)."]
    pub fn direct_asset_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direct_asset_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\nExternal or internal link."]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filepath` after provisioning.\nRelative path for a [Direct Asset link](https://docs.gitlab.com/ee/user/project/releases/index.html#permanent-links-to-release-assets)."]
    pub fn filepath(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filepath", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_id` after provisioning.\nThe ID of the link."]
    pub fn link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_type` after provisioning.\nThe type of the link. Valid values are `other`, `runbook`, `image`, `package`. Defaults to other."]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the link. Link names must be unique within the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding)."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe tag associated with the Release."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the link. Link URLs must be unique within the release."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

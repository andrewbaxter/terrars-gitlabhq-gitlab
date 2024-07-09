use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ReleaseLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filepath: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_type: Option<PrimField<String>>,
    name: PrimField<String>,
    project: PrimField<String>,
    tag_name: PrimField<String>,
    url: PrimField<String>,
}

struct ReleaseLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ReleaseLinkData>,
}

#[derive(Clone)]
pub struct ReleaseLink(Rc<ReleaseLink_>);

impl ReleaseLink {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGitlab) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `filepath`.\nRelative path for a [Direct Asset link](https://docs.gitlab.com/ee/user/project/releases/index.html#permanent-links-to-release-assets)."]
    pub fn set_filepath(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filepath = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `link_type`.\nThe type of the link. Valid values are `other`, `runbook`, `image`, `package`. Defaults to other."]
    pub fn set_link_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().link_type = Some(v.into());
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

impl Referable for ReleaseLink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ReleaseLink { }

impl ToListMappable for ReleaseLink {
    type O = ListRef<ReleaseLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ReleaseLink_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_release_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildReleaseLink {
    pub tf_id: String,
    #[doc= "The name of the link. Link names must be unique within the release."]
    pub name: PrimField<String>,
    #[doc= "The ID or [URL-encoded path of the project](https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding)."]
    pub project: PrimField<String>,
    #[doc= "The tag associated with the Release."]
    pub tag_name: PrimField<String>,
    #[doc= "The URL of the link. Link URLs must be unique within the release."]
    pub url: PrimField<String>,
}

impl BuildReleaseLink {
    pub fn build(self, stack: &mut Stack) -> ReleaseLink {
        let out = ReleaseLink(Rc::new(ReleaseLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ReleaseLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                filepath: core::default::Default::default(),
                id: core::default::Default::default(),
                link_type: core::default::Default::default(),
                name: self.name,
                project: self.project,
                tag_name: self.tag_name,
                url: self.url,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ReleaseLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for ReleaseLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ReleaseLinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

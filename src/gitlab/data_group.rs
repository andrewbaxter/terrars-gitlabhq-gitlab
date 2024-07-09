use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGroupData>,
}

#[derive(Clone)]
pub struct DataGroup(Rc<DataGroup_>);

impl DataGroup {
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

    #[doc= "Set the field `full_path`.\nThe full path of the group."]
    pub fn set_full_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_path = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group."]
    pub fn set_group_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nWhether developers and maintainers can push to the applicable default branch."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the group."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extra_shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Additional CI/CD minutes for this group."]
    pub fn extra_shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nThe full name of the group."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\nThe full path of the group."]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nBoolean, is LFS enabled for projects in this group."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_lock` after provisioning.\nUsers cannot be added to projects in this group."]
    pub fn membership_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of this group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\nInteger, ID of the parent group."]
    pub fn parent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the group."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_forking_outside_group` after provisioning.\nWhen enabled, users can not fork projects from this group to external namespaces."]
    pub fn prevent_forking_outside_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_forking_outside_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nBoolean, is request for access enabled to the group."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nThe group level registration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Maximum number of monthly CI/CD minutes for this group. Can be nil (default; inherit system default), 0 (unlimited), or > 0."]
    pub fn shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\nEnable or disable shared runners for a group’s subgroups and projects. Valid values are: `enabled`, `disabled_and_overridable`, `disabled_and_unoverridable`, `disabled_with_override`."]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_with_groups` after provisioning.\nDescribes groups which have access shared to this group."]
    pub fn shared_with_groups(&self) -> ListRef<DataGroupSharedWithGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shared_with_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nVisibility level of the group. Possible values are `private`, `internal`, `public`."]
    pub fn visibility_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nWeb URL of the group."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\nThe group's wiki access level. Only available on Premium and Ultimate plans. Valid values are `disabled`, `private`, `enabled`."]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.extract_ref()))
    }
}

impl Referable for DataGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGroup { }

impl ToListMappable for DataGroup {
    type O = ListRef<DataGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGroup_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGroup {
    pub tf_id: String,
}

impl BuildDataGroup {
    pub fn build(self, stack: &mut Stack) -> DataGroup {
        let out = DataGroup(Rc::new(DataGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                full_path: core::default::Default::default(),
                group_id: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nWhether developers and maintainers can push to the applicable default branch."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the group."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extra_shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Additional CI/CD minutes for this group."]
    pub fn extra_shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nThe full name of the group."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\nThe full path of the group."]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nBoolean, is LFS enabled for projects in this group."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_lock` after provisioning.\nUsers cannot be added to projects in this group."]
    pub fn membership_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of this group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\nInteger, ID of the parent group."]
    pub fn parent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the group."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_forking_outside_group` after provisioning.\nWhen enabled, users can not fork projects from this group to external namespaces."]
    pub fn prevent_forking_outside_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_forking_outside_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nBoolean, is request for access enabled to the group."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nThe group level registration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Maximum number of monthly CI/CD minutes for this group. Can be nil (default; inherit system default), 0 (unlimited), or > 0."]
    pub fn shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\nEnable or disable shared runners for a group’s subgroups and projects. Valid values are: `enabled`, `disabled_and_overridable`, `disabled_and_unoverridable`, `disabled_with_override`."]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_with_groups` after provisioning.\nDescribes groups which have access shared to this group."]
    pub fn shared_with_groups(&self) -> ListRef<DataGroupSharedWithGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shared_with_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nVisibility level of the group. Possible values are `private`, `internal`, `public`."]
    pub fn visibility_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nWeb URL of the group."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\nThe group's wiki access level. Only available on Premium and Ultimate plans. Valid values are `disabled`, `private`, `enabled`."]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGroupSharedWithGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_access_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataGroupSharedWithGroupsEl {
    #[doc= "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `group_access_level`.\n"]
    pub fn set_group_access_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_full_path`.\n"]
    pub fn set_group_full_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_full_path = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataGroupSharedWithGroupsEl {
    type O = BlockAssignable<DataGroupSharedWithGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGroupSharedWithGroupsEl {}

impl BuildDataGroupSharedWithGroupsEl {
    pub fn build(self) -> DataGroupSharedWithGroupsEl {
        DataGroupSharedWithGroupsEl {
            expires_at: core::default::Default::default(),
            group_access_level: core::default::Default::default(),
            group_full_path: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
        }
    }
}

pub struct DataGroupSharedWithGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupSharedWithGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataGroupSharedWithGroupsElRef {
        DataGroupSharedWithGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGroupSharedWithGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc= "Get a reference to the value of field `group_access_level` after provisioning.\n"]
    pub fn group_access_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `group_full_path` after provisioning.\n"]
    pub fn group_full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_full_path", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

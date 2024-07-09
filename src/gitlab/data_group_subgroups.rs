use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataGroupSubgroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_available: Option<PrimField<bool>>,
    group_id: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_groups: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_custom_attributes: Option<PrimField<bool>>,
}

struct DataGroupSubgroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGroupSubgroupsData>,
}

#[derive(Clone)]
pub struct DataGroupSubgroups(Rc<DataGroupSubgroups_>);

impl DataGroupSubgroups {
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

    #[doc= "Set the field `all_available`.\nShow all the groups you have access to."]
    pub fn set_all_available(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().all_available = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `min_access_level`.\nLimit to groups where current user has at least this access level."]
    pub fn set_min_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().min_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `order_by`.\nOrder groups by name, path or id."]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `owned`.\nLimit to groups explicitly owned by the current user."]
    pub fn set_owned(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().owned = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nReturn the list of authorized groups matching the search criteria."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_groups`.\nSkip the group IDs passed."]
    pub fn set_skip_groups(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().skip_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nOrder groups in asc or desc order."]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Set the field `statistics`.\nInclude group statistics (administrators only)."]
    pub fn set_statistics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().statistics = Some(v.into());
        self
    }

    #[doc= "Set the field `with_custom_attributes`.\nInclude custom attributes in response (administrators only)."]
    pub fn set_with_custom_attributes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_custom_attributes = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `all_available` after provisioning.\nShow all the groups you have access to."]
    pub fn all_available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_available", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_access_level` after provisioning.\nLimit to groups where current user has at least this access level."]
    pub fn min_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder groups by name, path or id."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owned` after provisioning.\nLimit to groups explicitly owned by the current user."]
    pub fn owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.owned", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn the list of authorized groups matching the search criteria."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_groups` after provisioning.\nSkip the group IDs passed."]
    pub fn skip_groups(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.skip_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nOrder groups in asc or desc order."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\nInclude group statistics (administrators only)."]
    pub fn statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroups` after provisioning.\nSubgroups of the parent group."]
    pub fn subgroups(&self) -> ListRef<DataGroupSubgroupsSubgroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subgroups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_custom_attributes` after provisioning.\nInclude custom attributes in response (administrators only)."]
    pub fn with_custom_attributes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_custom_attributes", self.extract_ref()))
    }
}

impl Referable for DataGroupSubgroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGroupSubgroups { }

impl ToListMappable for DataGroupSubgroups {
    type O = ListRef<DataGroupSubgroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGroupSubgroups_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_group_subgroups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGroupSubgroups {
    pub tf_id: String,
    #[doc= "The ID of the group."]
    pub group_id: PrimField<f64>,
}

impl BuildDataGroupSubgroups {
    pub fn build(self, stack: &mut Stack) -> DataGroupSubgroups {
        let out = DataGroupSubgroups(Rc::new(DataGroupSubgroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGroupSubgroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                all_available: core::default::Default::default(),
                group_id: self.group_id,
                id: core::default::Default::default(),
                min_access_level: core::default::Default::default(),
                order_by: core::default::Default::default(),
                owned: core::default::Default::default(),
                search: core::default::Default::default(),
                skip_groups: core::default::Default::default(),
                sort: core::default::Default::default(),
                statistics: core::default::Default::default(),
                with_custom_attributes: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGroupSubgroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupSubgroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGroupSubgroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `all_available` after provisioning.\nShow all the groups you have access to."]
    pub fn all_available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_available", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_access_level` after provisioning.\nLimit to groups where current user has at least this access level."]
    pub fn min_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder groups by name, path or id."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owned` after provisioning.\nLimit to groups explicitly owned by the current user."]
    pub fn owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.owned", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn the list of authorized groups matching the search criteria."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_groups` after provisioning.\nSkip the group IDs passed."]
    pub fn skip_groups(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.skip_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nOrder groups in asc or desc order."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\nInclude group statistics (administrators only)."]
    pub fn statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroups` after provisioning.\nSubgroups of the parent group."]
    pub fn subgroups(&self) -> ListRef<DataGroupSubgroupsSubgroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subgroups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_custom_attributes` after provisioning.\nInclude custom attributes in response (administrators only)."]
    pub fn with_custom_attributes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_custom_attributes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGroupSubgroupsSubgroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_protection: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_template_project_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_restriction_ranges: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentions_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_creation_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_two_factor_authentication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_with_group_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_setting: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subgroup_creation_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_factor_grace_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_access_level: Option<PrimField<String>>,
}

impl DataGroupSubgroupsSubgroupsEl {
    #[doc= "Set the field `auto_devops_enabled`.\n"]
    pub fn set_auto_devops_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_devops_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar_url`.\n"]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch_protection`.\n"]
    pub fn set_default_branch_protection(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_branch_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `emails_enabled`.\n"]
    pub fn set_emails_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `file_template_project_id`.\n"]
    pub fn set_file_template_project_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.file_template_project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `full_name`.\n"]
    pub fn set_full_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_name = Some(v.into());
        self
    }

    #[doc= "Set the field `full_path`.\n"]
    pub fn set_full_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_restriction_ranges`.\n"]
    pub fn set_ip_restriction_ranges(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_restriction_ranges = Some(v.into());
        self
    }

    #[doc= "Set the field `lfs_enabled`.\n"]
    pub fn set_lfs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lfs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `mentions_disabled`.\n"]
    pub fn set_mentions_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mentions_disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_id`.\n"]
    pub fn set_parent_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parent_id = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `project_creation_level`.\n"]
    pub fn set_project_creation_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_creation_level = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access_enabled`.\n"]
    pub fn set_request_access_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_access_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `require_two_factor_authentication`.\n"]
    pub fn set_require_two_factor_authentication(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_two_factor_authentication = Some(v.into());
        self
    }

    #[doc= "Set the field `share_with_group_lock`.\n"]
    pub fn set_share_with_group_lock(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.share_with_group_lock = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_setting`.\n"]
    pub fn set_shared_runners_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shared_runners_setting = Some(v.into());
        self
    }

    #[doc= "Set the field `statistics`.\n"]
    pub fn set_statistics(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.statistics = Some(v.into());
        self
    }

    #[doc= "Set the field `subgroup_creation_level`.\n"]
    pub fn set_subgroup_creation_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subgroup_creation_level = Some(v.into());
        self
    }

    #[doc= "Set the field `two_factor_grace_period`.\n"]
    pub fn set_two_factor_grace_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.two_factor_grace_period = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_access_level`.\n"]
    pub fn set_wiki_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wiki_access_level = Some(v.into());
        self
    }
}

impl ToListMappable for DataGroupSubgroupsSubgroupsEl {
    type O = BlockAssignable<DataGroupSubgroupsSubgroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGroupSubgroupsSubgroupsEl {}

impl BuildDataGroupSubgroupsSubgroupsEl {
    pub fn build(self) -> DataGroupSubgroupsSubgroupsEl {
        DataGroupSubgroupsSubgroupsEl {
            auto_devops_enabled: core::default::Default::default(),
            avatar_url: core::default::Default::default(),
            created_at: core::default::Default::default(),
            default_branch_protection: core::default::Default::default(),
            description: core::default::Default::default(),
            emails_enabled: core::default::Default::default(),
            file_template_project_id: core::default::Default::default(),
            full_name: core::default::Default::default(),
            full_path: core::default::Default::default(),
            group_id: core::default::Default::default(),
            ip_restriction_ranges: core::default::Default::default(),
            lfs_enabled: core::default::Default::default(),
            mentions_disabled: core::default::Default::default(),
            name: core::default::Default::default(),
            parent_id: core::default::Default::default(),
            path: core::default::Default::default(),
            project_creation_level: core::default::Default::default(),
            request_access_enabled: core::default::Default::default(),
            require_two_factor_authentication: core::default::Default::default(),
            share_with_group_lock: core::default::Default::default(),
            shared_runners_setting: core::default::Default::default(),
            statistics: core::default::Default::default(),
            subgroup_creation_level: core::default::Default::default(),
            two_factor_grace_period: core::default::Default::default(),
            visibility: core::default::Default::default(),
            web_url: core::default::Default::default(),
            wiki_access_level: core::default::Default::default(),
        }
    }
}

pub struct DataGroupSubgroupsSubgroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupSubgroupsSubgroupsElRef {
    fn new(shared: StackShared, base: String) -> DataGroupSubgroupsSubgroupsElRef {
        DataGroupSubgroupsSubgroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGroupSubgroupsSubgroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\n"]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\n"]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\n"]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `file_template_project_id` after provisioning.\n"]
    pub fn file_template_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_template_project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.base))
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\n"]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_restriction_ranges` after provisioning.\n"]
    pub fn ip_restriction_ranges(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_restriction_ranges", self.base))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\n"]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `mentions_disabled` after provisioning.\n"]
    pub fn mentions_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mentions_disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `project_creation_level` after provisioning.\n"]
    pub fn project_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_creation_level", self.base))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\n"]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `require_two_factor_authentication` after provisioning.\n"]
    pub fn require_two_factor_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_two_factor_authentication", self.base))
    }

    #[doc= "Get a reference to the value of field `share_with_group_lock` after provisioning.\n"]
    pub fn share_with_group_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_with_group_lock", self.base))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\n"]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.base))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\n"]
    pub fn statistics(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.statistics", self.base))
    }

    #[doc= "Get a reference to the value of field `subgroup_creation_level` after provisioning.\n"]
    pub fn subgroup_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subgroup_creation_level", self.base))
    }

    #[doc= "Get a reference to the value of field `two_factor_grace_period` after provisioning.\n"]
    pub fn two_factor_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_grace_period", self.base))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\n"]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.base))
    }
}

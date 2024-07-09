use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_level_only: Option<PrimField<bool>>,
}

struct DataGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGroupsData>,
}

#[derive(Clone)]
pub struct DataGroups(Rc<DataGroups_>);

impl DataGroups {
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

    #[doc= "Set the field `order_by`.\nOrder the groups' list by `id`, `name`, `path`, or `similarity`. (Requires administrator privileges)"]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nSearch groups by name or path."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nSort groups' list in asc or desc order. (Requires administrator privileges)"]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Set the field `top_level_only`.\nLimit to top level groups, excluding all subgroups."]
    pub fn set_top_level_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().top_level_only = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `groups` after provisioning.\nThe list of groups."]
    pub fn groups(&self) -> ListRef<DataGroupsGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder the groups' list by `id`, `name`, `path`, or `similarity`. (Requires administrator privileges)"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch groups by name or path."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nSort groups' list in asc or desc order. (Requires administrator privileges)"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `top_level_only` after provisioning.\nLimit to top level groups, excluding all subgroups."]
    pub fn top_level_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_level_only", self.extract_ref()))
    }
}

impl Referable for DataGroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGroups { }

impl ToListMappable for DataGroups {
    type O = ListRef<DataGroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGroups_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_groups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGroups {
    pub tf_id: String,
}

impl BuildDataGroups {
    pub fn build(self, stack: &mut Stack) -> DataGroups {
        let out = DataGroups(Rc::new(DataGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                order_by: core::default::Default::default(),
                search: core::default::Default::default(),
                sort: core::default::Default::default(),
                top_level_only: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `groups` after provisioning.\nThe list of groups."]
    pub fn groups(&self) -> ListRef<DataGroupsGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder the groups' list by `id`, `name`, `path`, or `similarity`. (Requires administrator privileges)"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch groups by name or path."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nSort groups' list in asc or desc order. (Requires administrator privileges)"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `top_level_only` after provisioning.\nLimit to top level groups, excluding all subgroups."]
    pub fn top_level_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.top_level_only", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGroupsGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_protection: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_forking_outside_group: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runners_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_setting: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_access_level: Option<PrimField<String>>,
}

impl DataGroupsGroupsEl {
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

    #[doc= "Set the field `lfs_enabled`.\n"]
    pub fn set_lfs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lfs_enabled = Some(v.into());
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

    #[doc= "Set the field `prevent_forking_outside_group`.\n"]
    pub fn set_prevent_forking_outside_group(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_forking_outside_group = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access_enabled`.\n"]
    pub fn set_request_access_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_access_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `runners_token`.\n"]
    pub fn set_runners_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runners_token = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_setting`.\n"]
    pub fn set_shared_runners_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shared_runners_setting = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility_level`.\n"]
    pub fn set_visibility_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility_level = Some(v.into());
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

impl ToListMappable for DataGroupsGroupsEl {
    type O = BlockAssignable<DataGroupsGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGroupsGroupsEl {}

impl BuildDataGroupsGroupsEl {
    pub fn build(self) -> DataGroupsGroupsEl {
        DataGroupsGroupsEl {
            default_branch_protection: core::default::Default::default(),
            description: core::default::Default::default(),
            full_name: core::default::Default::default(),
            full_path: core::default::Default::default(),
            group_id: core::default::Default::default(),
            lfs_enabled: core::default::Default::default(),
            name: core::default::Default::default(),
            parent_id: core::default::Default::default(),
            path: core::default::Default::default(),
            prevent_forking_outside_group: core::default::Default::default(),
            request_access_enabled: core::default::Default::default(),
            runners_token: core::default::Default::default(),
            shared_runners_setting: core::default::Default::default(),
            visibility_level: core::default::Default::default(),
            web_url: core::default::Default::default(),
            wiki_access_level: core::default::Default::default(),
        }
    }
}

pub struct DataGroupsGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupsGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataGroupsGroupsElRef {
        DataGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGroupsGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\n"]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
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

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\n"]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.base))
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

    #[doc= "Get a reference to the value of field `prevent_forking_outside_group` after provisioning.\n"]
    pub fn prevent_forking_outside_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_forking_outside_group", self.base))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\n"]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\n"]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.base))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\n"]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.base))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\n"]
    pub fn visibility_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility_level", self.base))
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

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_protection: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_shared_runners_minutes_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_restriction_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentions_disabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<PrimField<f64>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_forking_outside_group: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_creation_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_two_factor_authentication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_with_group_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_minutes_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_setting: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subgroup_creation_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_factor_grace_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_rules: Option<Vec<GroupPushRulesEl>>,
    dynamic: GroupDynamic,
}

struct Group_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupData>,
}

#[derive(Clone)]
pub struct Group(Rc<Group_>);

impl Group {
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

    #[doc= "Set the field `auto_devops_enabled`.\nDefault to Auto DevOps pipeline for all projects within this group."]
    pub fn set_auto_devops_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_devops_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar`.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn set_avatar(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar_hash`.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn set_avatar_hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar_hash = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch_protection`.\nSee https://docs.gitlab.com/ee/api/groups.html#options-for-default_branch_protection. Valid values are: `0`, `1`, `2`, `3`, `4`."]
    pub fn set_default_branch_protection(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_branch_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe group's description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `emails_enabled`.\nEnable email notifications."]
    pub fn set_emails_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `extra_shared_runners_minutes_limit`.\nCan be set by administrators only. Additional CI/CD minutes for this group."]
    pub fn set_extra_shared_runners_minutes_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().extra_shared_runners_minutes_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_restriction_ranges`.\nA list of IP addresses or subnet masks to restrict group access. Will be concatenated together into a comma separated string. Only allowed on top level groups."]
    pub fn set_ip_restriction_ranges(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ip_restriction_ranges = Some(v.into());
        self
    }

    #[doc= "Set the field `lfs_enabled`.\nEnable/disable Large File Storage (LFS) for the projects in this group."]
    pub fn set_lfs_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lfs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `membership_lock`.\nUsers cannot be added to projects in this group."]
    pub fn set_membership_lock(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().membership_lock = Some(v.into());
        self
    }

    #[doc= "Set the field `mentions_disabled`.\nDisable the capability of a group from getting mentioned."]
    pub fn set_mentions_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mentions_disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_id`.\nId of the parent group (creates a nested group)."]
    pub fn set_parent_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().parent_id = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_forking_outside_group`.\nDefaults to false. When enabled, users can not fork projects from this group to external namespaces."]
    pub fn set_prevent_forking_outside_group(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prevent_forking_outside_group = Some(v.into());
        self
    }

    #[doc= "Set the field `project_creation_level`.\nDetermine if developers can create projects in the group. Valid values are: `noone`, `maintainer`, `developer`"]
    pub fn set_project_creation_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_creation_level = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access_enabled`.\nAllow users to request member access."]
    pub fn set_request_access_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().request_access_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `require_two_factor_authentication`.\nRequire all users in this group to setup Two-factor authentication."]
    pub fn set_require_two_factor_authentication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_two_factor_authentication = Some(v.into());
        self
    }

    #[doc= "Set the field `share_with_group_lock`.\nPrevent sharing a project with another group within this group."]
    pub fn set_share_with_group_lock(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().share_with_group_lock = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_minutes_limit`.\nCan be set by administrators only. Maximum number of monthly CI/CD minutes for this group. Can be nil (default; inherit system default), 0 (unlimited), or > 0."]
    pub fn set_shared_runners_minutes_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().shared_runners_minutes_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_setting`.\nEnable or disable shared runners for a group’s subgroups and projects. Valid values are: `enabled`, `disabled_and_overridable`, `disabled_and_unoverridable`, `disabled_with_override`."]
    pub fn set_shared_runners_setting(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().shared_runners_setting = Some(v.into());
        self
    }

    #[doc= "Set the field `subgroup_creation_level`.\nAllowed to create subgroups. Valid values are: `owner`, `maintainer`."]
    pub fn set_subgroup_creation_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subgroup_creation_level = Some(v.into());
        self
    }

    #[doc= "Set the field `two_factor_grace_period`.\nDefaults to 48. Time before Two-factor authentication is enforced (in hours)."]
    pub fn set_two_factor_grace_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().two_factor_grace_period = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility_level`.\nThe group's visibility. Can be `private`, `internal`, or `public`. Valid values are: `private`, `internal`, `public`."]
    pub fn set_visibility_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility_level = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_access_level`.\nThe group's wiki access level. Only available on Premium and Ultimate plans. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_wiki_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wiki_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `push_rules`.\n"]
    pub fn set_push_rules(self, v: impl Into<BlockAssignable<GroupPushRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().push_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.push_rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nDefault to Auto DevOps pipeline for all projects within this group."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar` after provisioning.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn avatar(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_hash` after provisioning.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn avatar_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe URL of the avatar image."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nSee https://docs.gitlab.com/ee/api/groups.html#options-for-default_branch_protection. Valid values are: `0`, `1`, `2`, `3`, `4`."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe group's description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\nEnable email notifications."]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_restriction_ranges` after provisioning.\nA list of IP addresses or subnet masks to restrict group access. Will be concatenated together into a comma separated string. Only allowed on top level groups."]
    pub fn ip_restriction_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_restriction_ranges", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nEnable/disable Large File Storage (LFS) for the projects in this group."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_lock` after provisioning.\nUsers cannot be added to projects in this group."]
    pub fn membership_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mentions_disabled` after provisioning.\nDisable the capability of a group from getting mentioned."]
    pub fn mentions_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mentions_disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\nId of the parent group (creates a nested group)."]
    pub fn parent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the group."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_forking_outside_group` after provisioning.\nDefaults to false. When enabled, users can not fork projects from this group to external namespaces."]
    pub fn prevent_forking_outside_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_forking_outside_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_creation_level` after provisioning.\nDetermine if developers can create projects in the group. Valid values are: `noone`, `maintainer`, `developer`"]
    pub fn project_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_creation_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nAllow users to request member access."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_two_factor_authentication` after provisioning.\nRequire all users in this group to setup Two-factor authentication."]
    pub fn require_two_factor_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_two_factor_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nThe group level registration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_with_group_lock` after provisioning.\nPrevent sharing a project with another group within this group."]
    pub fn share_with_group_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_with_group_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Maximum number of monthly CI/CD minutes for this group. Can be nil (default; inherit system default), 0 (unlimited), or > 0."]
    pub fn shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\nEnable or disable shared runners for a group’s subgroups and projects. Valid values are: `enabled`, `disabled_and_overridable`, `disabled_and_unoverridable`, `disabled_with_override`."]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroup_creation_level` after provisioning.\nAllowed to create subgroups. Valid values are: `owner`, `maintainer`."]
    pub fn subgroup_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subgroup_creation_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_grace_period` after provisioning.\nDefaults to 48. Time before Two-factor authentication is enforced (in hours)."]
    pub fn two_factor_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nThe group's visibility. Can be `private`, `internal`, or `public`. Valid values are: `private`, `internal`, `public`."]
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

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\n"]
    pub fn push_rules(&self) -> ListRef<GroupPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }
}

impl Referable for Group {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Group { }

impl ToListMappable for Group {
    type O = ListRef<GroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Group_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroup {
    pub tf_id: String,
    #[doc= "The name of the group."]
    pub name: PrimField<String>,
    #[doc= "The path of the group."]
    pub path: PrimField<String>,
}

impl BuildGroup {
    pub fn build(self, stack: &mut Stack) -> Group {
        let out = Group(Rc::new(Group_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_devops_enabled: core::default::Default::default(),
                avatar: core::default::Default::default(),
                avatar_hash: core::default::Default::default(),
                default_branch_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                emails_enabled: core::default::Default::default(),
                extra_shared_runners_minutes_limit: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_restriction_ranges: core::default::Default::default(),
                lfs_enabled: core::default::Default::default(),
                membership_lock: core::default::Default::default(),
                mentions_disabled: core::default::Default::default(),
                name: self.name,
                parent_id: core::default::Default::default(),
                path: self.path,
                prevent_forking_outside_group: core::default::Default::default(),
                project_creation_level: core::default::Default::default(),
                request_access_enabled: core::default::Default::default(),
                require_two_factor_authentication: core::default::Default::default(),
                share_with_group_lock: core::default::Default::default(),
                shared_runners_minutes_limit: core::default::Default::default(),
                shared_runners_setting: core::default::Default::default(),
                subgroup_creation_level: core::default::Default::default(),
                two_factor_grace_period: core::default::Default::default(),
                visibility_level: core::default::Default::default(),
                wiki_access_level: core::default::Default::default(),
                push_rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nDefault to Auto DevOps pipeline for all projects within this group."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar` after provisioning.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn avatar(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_hash` after provisioning.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn avatar_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe URL of the avatar image."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nSee https://docs.gitlab.com/ee/api/groups.html#options-for-default_branch_protection. Valid values are: `0`, `1`, `2`, `3`, `4`."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe group's description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\nEnable email notifications."]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_restriction_ranges` after provisioning.\nA list of IP addresses or subnet masks to restrict group access. Will be concatenated together into a comma separated string. Only allowed on top level groups."]
    pub fn ip_restriction_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_restriction_ranges", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nEnable/disable Large File Storage (LFS) for the projects in this group."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_lock` after provisioning.\nUsers cannot be added to projects in this group."]
    pub fn membership_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mentions_disabled` after provisioning.\nDisable the capability of a group from getting mentioned."]
    pub fn mentions_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mentions_disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\nId of the parent group (creates a nested group)."]
    pub fn parent_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the group."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_forking_outside_group` after provisioning.\nDefaults to false. When enabled, users can not fork projects from this group to external namespaces."]
    pub fn prevent_forking_outside_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_forking_outside_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_creation_level` after provisioning.\nDetermine if developers can create projects in the group. Valid values are: `noone`, `maintainer`, `developer`"]
    pub fn project_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_creation_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nAllow users to request member access."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_two_factor_authentication` after provisioning.\nRequire all users in this group to setup Two-factor authentication."]
    pub fn require_two_factor_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_two_factor_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nThe group level registration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_with_group_lock` after provisioning.\nPrevent sharing a project with another group within this group."]
    pub fn share_with_group_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_with_group_lock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes_limit` after provisioning.\nCan be set by administrators only. Maximum number of monthly CI/CD minutes for this group. Can be nil (default; inherit system default), 0 (unlimited), or > 0."]
    pub fn shared_runners_minutes_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_setting` after provisioning.\nEnable or disable shared runners for a group’s subgroups and projects. Valid values are: `enabled`, `disabled_and_overridable`, `disabled_and_unoverridable`, `disabled_with_override`."]
    pub fn shared_runners_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_setting", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroup_creation_level` after provisioning.\nAllowed to create subgroups. Valid values are: `owner`, `maintainer`."]
    pub fn subgroup_creation_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subgroup_creation_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_grace_period` after provisioning.\nDefaults to 48. Time before Two-factor authentication is enforced (in hours)."]
    pub fn two_factor_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nThe group's visibility. Can be `private`, `internal`, or `public`. Valid values are: `private`, `internal`, `public`."]
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

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\n"]
    pub fn push_rules(&self) -> ListRef<GroupPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GroupPushRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    author_email_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_committer_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_committer_name_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message_negative_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_delete_tag: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_check: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_secrets: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reject_unsigned_commits: Option<PrimField<bool>>,
}

impl GroupPushRulesEl {
    #[doc= "Set the field `author_email_regex`.\nAll commit author emails must match this regex, e.g. `@my-company.com$`."]
    pub fn set_author_email_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.author_email_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `branch_name_regex`.\nAll branch names must match this regex, e.g. `(feature|hotfix)\\/*`."]
    pub fn set_branch_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_check`.\nOnly commits pushed using verified emails are allowed.  **Note** This attribute is only supported in GitLab versions >= 16.4."]
    pub fn set_commit_committer_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_name_check`.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn set_commit_committer_name_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_name_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_negative_regex`.\nNo commit message is allowed to match this regex, for example `ssh\\:\\/\\/`."]
    pub fn set_commit_message_negative_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_message_negative_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_regex`.\nAll commit messages must match this regex, e.g. `Fixed \\d+\\..*`."]
    pub fn set_commit_message_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_message_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `deny_delete_tag`.\nDeny deleting a tag."]
    pub fn set_deny_delete_tag(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deny_delete_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `file_name_regex`.\nFilenames matching the regular expression provided in this attribute are not allowed, for example, `(jar|exe)$`."]
    pub fn set_file_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `max_file_size`.\nMaximum file size (MB) allowed."]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `member_check`.\nAllows only GitLab users to author commits."]
    pub fn set_member_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.member_check = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_secrets`.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn set_prevent_secrets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `reject_unsigned_commits`.\nOnly commits signed through GPG are allowed.  **Note** This attribute is only supported in GitLab versions >= 16.4."]
    pub fn set_reject_unsigned_commits(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reject_unsigned_commits = Some(v.into());
        self
    }
}

impl ToListMappable for GroupPushRulesEl {
    type O = BlockAssignable<GroupPushRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGroupPushRulesEl {}

impl BuildGroupPushRulesEl {
    pub fn build(self) -> GroupPushRulesEl {
        GroupPushRulesEl {
            author_email_regex: core::default::Default::default(),
            branch_name_regex: core::default::Default::default(),
            commit_committer_check: core::default::Default::default(),
            commit_committer_name_check: core::default::Default::default(),
            commit_message_negative_regex: core::default::Default::default(),
            commit_message_regex: core::default::Default::default(),
            deny_delete_tag: core::default::Default::default(),
            file_name_regex: core::default::Default::default(),
            max_file_size: core::default::Default::default(),
            member_check: core::default::Default::default(),
            prevent_secrets: core::default::Default::default(),
            reject_unsigned_commits: core::default::Default::default(),
        }
    }
}

pub struct GroupPushRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupPushRulesElRef {
    fn new(shared: StackShared, base: String) -> GroupPushRulesElRef {
        GroupPushRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GroupPushRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `author_email_regex` after provisioning.\nAll commit author emails must match this regex, e.g. `@my-company.com$`."]
    pub fn author_email_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `branch_name_regex` after provisioning.\nAll branch names must match this regex, e.g. `(feature|hotfix)\\/*`."]
    pub fn branch_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_committer_check` after provisioning.\nOnly commits pushed using verified emails are allowed.  **Note** This attribute is only supported in GitLab versions >= 16.4."]
    pub fn commit_committer_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_committer_name_check` after provisioning.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn commit_committer_name_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_name_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_negative_regex` after provisioning.\nNo commit message is allowed to match this regex, for example `ssh\\:\\/\\/`."]
    pub fn commit_message_negative_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_negative_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_regex` after provisioning.\nAll commit messages must match this regex, e.g. `Fixed \\d+\\..*`."]
    pub fn commit_message_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `deny_delete_tag` after provisioning.\nDeny deleting a tag."]
    pub fn deny_delete_tag(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_delete_tag", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name_regex` after provisioning.\nFilenames matching the regular expression provided in this attribute are not allowed, for example, `(jar|exe)$`."]
    pub fn file_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\nMaximum file size (MB) allowed."]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc= "Get a reference to the value of field `member_check` after provisioning.\nAllows only GitLab users to author commits."]
    pub fn member_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_check", self.base))
    }

    #[doc= "Get a reference to the value of field `prevent_secrets` after provisioning.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn prevent_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `reject_unsigned_commits` after provisioning.\nOnly commits signed through GPG are allowed.  **Note** This attribute is only supported in GitLab versions >= 16.4."]
    pub fn reject_unsigned_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reject_unsigned_commits", self.base))
    }
}

#[derive(Serialize, Default)]
struct GroupDynamic {
    push_rules: Option<DynamicBlock<GroupPushRulesEl>>,
}

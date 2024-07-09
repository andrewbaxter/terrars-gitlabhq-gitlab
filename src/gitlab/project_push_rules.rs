use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectPushRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
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
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reject_unsigned_commits: Option<PrimField<bool>>,
}

struct ProjectPushRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectPushRulesData>,
}

#[derive(Clone)]
pub struct ProjectPushRules(Rc<ProjectPushRules_>);

impl ProjectPushRules {
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

    #[doc= "Set the field `author_email_regex`.\nAll commit author emails must match this regex, e.g. `@my-company.com$`."]
    pub fn set_author_email_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().author_email_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `branch_name_regex`.\nAll branch names must match this regex, e.g. `(feature|hotfix)\\/*`."]
    pub fn set_branch_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branch_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_check`.\nUsers can only push commits to this repository that were committed with one of their own verified emails."]
    pub fn set_commit_committer_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().commit_committer_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_name_check`.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn set_commit_committer_name_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().commit_committer_name_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_negative_regex`.\nNo commit message is allowed to match this regex, e.g. `ssh\\:\\/\\/`."]
    pub fn set_commit_message_negative_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_message_negative_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_regex`.\nAll commit messages must match this regex, e.g. `Fixed \\d+\\..*`."]
    pub fn set_commit_message_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_message_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `deny_delete_tag`.\nDeny deleting a tag."]
    pub fn set_deny_delete_tag(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deny_delete_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `file_name_regex`.\nAll committed filenames must not match this regex, e.g. `(jar|exe)$`."]
    pub fn set_file_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `max_file_size`.\nMaximum file size (MB)."]
    pub fn set_max_file_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `member_check`.\nRestrict commits by author (email) to existing GitLab users."]
    pub fn set_member_check(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().member_check = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_secrets`.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn set_prevent_secrets(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prevent_secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `reject_unsigned_commits`.\nReject commit when it’s not signed."]
    pub fn set_reject_unsigned_commits(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reject_unsigned_commits = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `author_email_regex` after provisioning.\nAll commit author emails must match this regex, e.g. `@my-company.com$`."]
    pub fn author_email_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch_name_regex` after provisioning.\nAll branch names must match this regex, e.g. `(feature|hotfix)\\/*`."]
    pub fn branch_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_committer_check` after provisioning.\nUsers can only push commits to this repository that were committed with one of their own verified emails."]
    pub fn commit_committer_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_committer_name_check` after provisioning.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn commit_committer_name_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_name_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message_negative_regex` after provisioning.\nNo commit message is allowed to match this regex, e.g. `ssh\\:\\/\\/`."]
    pub fn commit_message_negative_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_negative_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message_regex` after provisioning.\nAll commit messages must match this regex, e.g. `Fixed \\d+\\..*`."]
    pub fn commit_message_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deny_delete_tag` after provisioning.\nDeny deleting a tag."]
    pub fn deny_delete_tag(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_delete_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name_regex` after provisioning.\nAll committed filenames must not match this regex, e.g. `(jar|exe)$`."]
    pub fn file_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\nMaximum file size (MB)."]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_check` after provisioning.\nRestrict commits by author (email) to existing GitLab users."]
    pub fn member_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_secrets` after provisioning.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn prevent_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_secrets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reject_unsigned_commits` after provisioning.\nReject commit when it’s not signed."]
    pub fn reject_unsigned_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reject_unsigned_commits", self.extract_ref()))
    }
}

impl Referable for ProjectPushRules {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectPushRules { }

impl ToListMappable for ProjectPushRules {
    type O = ListRef<ProjectPushRulesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectPushRules_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_push_rules".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectPushRules {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the project."]
    pub project: PrimField<String>,
}

impl BuildProjectPushRules {
    pub fn build(self, stack: &mut Stack) -> ProjectPushRules {
        let out = ProjectPushRules(Rc::new(ProjectPushRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectPushRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                project: self.project,
                reject_unsigned_commits: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectPushRulesRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectPushRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectPushRulesRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `author_email_regex` after provisioning.\nAll commit author emails must match this regex, e.g. `@my-company.com$`."]
    pub fn author_email_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch_name_regex` after provisioning.\nAll branch names must match this regex, e.g. `(feature|hotfix)\\/*`."]
    pub fn branch_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_committer_check` after provisioning.\nUsers can only push commits to this repository that were committed with one of their own verified emails."]
    pub fn commit_committer_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_committer_name_check` after provisioning.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn commit_committer_name_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_name_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message_negative_regex` after provisioning.\nNo commit message is allowed to match this regex, e.g. `ssh\\:\\/\\/`."]
    pub fn commit_message_negative_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_negative_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message_regex` after provisioning.\nAll commit messages must match this regex, e.g. `Fixed \\d+\\..*`."]
    pub fn commit_message_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deny_delete_tag` after provisioning.\nDeny deleting a tag."]
    pub fn deny_delete_tag(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_delete_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name_regex` after provisioning.\nAll committed filenames must not match this regex, e.g. `(jar|exe)$`."]
    pub fn file_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\nMaximum file size (MB)."]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_check` after provisioning.\nRestrict commits by author (email) to existing GitLab users."]
    pub fn member_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_check", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prevent_secrets` after provisioning.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn prevent_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_secrets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reject_unsigned_commits` after provisioning.\nReject commit when it’s not signed."]
    pub fn reject_unsigned_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reject_unsigned_commits", self.extract_ref()))
    }
}

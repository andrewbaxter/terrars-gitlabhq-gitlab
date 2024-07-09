use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_merge_on_skipped_pipeline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approvals_before_merge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_cancel_pending_pipelines: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_deploy_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoclose_referenced_issues: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_coverage_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_git_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    builds_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_config_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_default_git_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_forward_deployment_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_restrict_pipeline_cancellation_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_separated_caches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environments_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_classification_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_flags_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forked_from_project_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forking_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_runners_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_with_project_templates_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infrastructure_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialize_with_readme: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_latest_artifact: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_commit_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_pipelines_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_trains_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_overwrites_diverged_branches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_trigger_builds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mr_default_target_self: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_all_discussions_are_resolved: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_pipeline_succeeds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_mirror_protected_branches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packages_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipelines_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    printing_merge_request_link_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_builds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_jobs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    releases_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_source_branch_after_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_outdated_diff_discussions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_user_defined_variables: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_and_compliance_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_wait_for_default_branch_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_commit_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggestion_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_project_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_custom_template: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_expiration_policy: Option<Vec<ProjectContainerExpirationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_rules: Option<Vec<ProjectPushRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ProjectTimeoutsEl>,
    dynamic: ProjectDynamic,
}

struct Project_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectData>,
}

#[derive(Clone)]
pub struct Project(Rc<Project_>);

impl Project {
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

    #[doc= "Set the field `allow_merge_on_skipped_pipeline`.\nSet to true if you want to treat skipped pipelines as if they finished with success."]
    pub fn set_allow_merge_on_skipped_pipeline(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_merge_on_skipped_pipeline = Some(v.into());
        self
    }

    #[doc= "Set the field `analytics_access_level`.\nSet the analytics access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_analytics_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().analytics_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `approvals_before_merge`.\nNumber of merge request approvals required for merging. Default is 0.\n  This field **does not** work well in combination with the `gitlab_project_approval_rule` resource\n  and is most likely gonna be deprecated in a future GitLab version (see [this upstream epic](https://gitlab.com/groups/gitlab-org/-/epics/7572)).\n  In the meantime we recommend against using this attribute and use `gitlab_project_approval_rule` instead.\n"]
    pub fn set_approvals_before_merge(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().approvals_before_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `archive_on_destroy`.\nSet to `true` to archive the project instead of deleting on destroy. If set to `true` it will entire omit the `DELETE` operation."]
    pub fn set_archive_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archive_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `archived`.\nWhether the project is in read-only mode (archived). Repositories can be archived/unarchived by toggling this parameter."]
    pub fn set_archived(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archived = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_cancel_pending_pipelines`.\nAuto-cancel pending pipelines. This isn’t a boolean, but enabled/disabled."]
    pub fn set_auto_cancel_pending_pipelines(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_cancel_pending_pipelines = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_deploy_strategy`.\nAuto Deploy strategy. Valid values are `continuous`, `manual`, `timed_incremental`."]
    pub fn set_auto_devops_deploy_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_devops_deploy_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_enabled`.\nEnable Auto DevOps for this project."]
    pub fn set_auto_devops_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_devops_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `autoclose_referenced_issues`.\nSet whether auto-closing referenced issues on default branch."]
    pub fn set_autoclose_referenced_issues(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().autoclose_referenced_issues = Some(v.into());
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

    #[doc= "Set the field `build_coverage_regex`.\nTest coverage parsing for the project. This is deprecated feature in GitLab 15.0."]
    pub fn set_build_coverage_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_coverage_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `build_git_strategy`.\nThe Git strategy. Defaults to fetch. Valid values are `clone`, `fetch`."]
    pub fn set_build_git_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_git_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `build_timeout`.\nThe maximum amount of time, in seconds, that a job can run."]
    pub fn set_build_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().build_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `builds_access_level`.\nSet the builds access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_builds_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().builds_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_config_path`.\nCustom Path to CI config file."]
    pub fn set_ci_config_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ci_config_path = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_default_git_depth`.\nDefault number of revisions for shallow cloning."]
    pub fn set_ci_default_git_depth(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ci_default_git_depth = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_forward_deployment_enabled`.\nWhen a new deployment job starts, skip older deployment jobs that are still pending."]
    pub fn set_ci_forward_deployment_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ci_forward_deployment_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_restrict_pipeline_cancellation_role`.\nThe role required to cancel a pipeline or job. Introduced in GitLab 16.8. Premium and Ultimate only. Valid values are `developer`, `maintainer`, `no one`"]
    pub fn set_ci_restrict_pipeline_cancellation_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ci_restrict_pipeline_cancellation_role = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_separated_caches`.\nUse separate caches for protected branches."]
    pub fn set_ci_separated_caches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ci_separated_caches = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_access_level`.\nSet visibility of container registry, for this project. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_container_registry_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_registry_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_enabled`.\nEnable container registry for the project."]
    pub fn set_container_registry_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().container_registry_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch`.\nThe default branch for the project."]
    pub fn set_default_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description of the project."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `emails_enabled`.\nEnable email notifications."]
    pub fn set_emails_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `environments_access_level`.\nSet the environments access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_environments_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environments_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_classification_label`.\nThe classification label for the project."]
    pub fn set_external_authorization_classification_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_authorization_classification_label = Some(v.into());
        self
    }

    #[doc= "Set the field `feature_flags_access_level`.\nSet the feature flags access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_feature_flags_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().feature_flags_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `forked_from_project_id`.\nThe id of the project to fork. During create the project is forked and during an update the fork relation is changed."]
    pub fn set_forked_from_project_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().forked_from_project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `forking_access_level`.\nSet the forking access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_forking_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().forking_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_runners_enabled`.\nEnable group runners for this project."]
    pub fn set_group_runners_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().group_runners_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `group_with_project_templates_id`.\nFor group-level custom templates, specifies ID of group from which all the custom project templates are sourced. Leave empty for instance-level templates. Requires use_custom_template to be true (enterprise edition)."]
    pub fn set_group_with_project_templates_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_with_project_templates_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `import_url`.\nGit URL to a repository to be imported. Together with `mirror = true` it will setup a Pull Mirror. This can also be used together with `forked_from_project_id` to setup a Pull Mirror for a fork. The fork takes precedence over the import. Make sure to provide the credentials in `import_url_username` and `import_url_password`. GitLab never returns the credentials, thus the provider cannot detect configuration drift in the credentials. They can also not be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn set_import_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().import_url = Some(v.into());
        self
    }

    #[doc= "Set the field `import_url_password`.\nThe password for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn set_import_url_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().import_url_password = Some(v.into());
        self
    }

    #[doc= "Set the field `import_url_username`.\nThe username for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`.  See the examples section for how to properly use it."]
    pub fn set_import_url_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().import_url_username = Some(v.into());
        self
    }

    #[doc= "Set the field `infrastructure_access_level`.\nSet the infrastructure access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_infrastructure_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().infrastructure_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `initialize_with_readme`.\nCreate main branch with first commit containing a README.md file. Must be set to `true` if importing an uninitialized project with a different `default_branch`."]
    pub fn set_initialize_with_readme(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().initialize_with_readme = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_access_level`.\nSet the issues access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_issues_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().issues_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_enabled`.\nEnable issue tracking for the project."]
    pub fn set_issues_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().issues_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_template`.\nSets the template for new issues in the project."]
    pub fn set_issues_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().issues_template = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_latest_artifact`.\nDisable or enable the ability to keep the latest artifact for this project."]
    pub fn set_keep_latest_artifact(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_latest_artifact = Some(v.into());
        self
    }

    #[doc= "Set the field `lfs_enabled`.\nEnable LFS for the project."]
    pub fn set_lfs_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lfs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_commit_template`.\nTemplate used to create merge commit message in merge requests. (Introduced in GitLab 14.5.)"]
    pub fn set_merge_commit_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_commit_template = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_method`.\nSet the merge method. Valid values are `merge`, `rebase_merge`, `ff`."]
    pub fn set_merge_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_method = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_pipelines_enabled`.\nEnable or disable merge pipelines."]
    pub fn set_merge_pipelines_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_pipelines_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_access_level`.\nSet the merge requests access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_merge_requests_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_requests_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_enabled`.\nEnable merge requests for the project."]
    pub fn set_merge_requests_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_template`.\nSets the template for new merge requests in the project."]
    pub fn set_merge_requests_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_requests_template = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_trains_enabled`.\nEnable or disable merge trains. Requires `merge_pipelines_enabled` to be set to `true` to take effect."]
    pub fn set_merge_trains_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_trains_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror`.\nEnable project pull mirror."]
    pub fn set_mirror(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mirror = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_overwrites_diverged_branches`.\nEnable overwrite diverged branches for a mirrored project."]
    pub fn set_mirror_overwrites_diverged_branches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mirror_overwrites_diverged_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_trigger_builds`.\nEnable trigger builds on pushes for a mirrored project."]
    pub fn set_mirror_trigger_builds(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mirror_trigger_builds = Some(v.into());
        self
    }

    #[doc= "Set the field `monitor_access_level`.\nSet the monitor access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_monitor_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().monitor_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `mr_default_target_self`.\nFor forked projects, target merge requests to this project. If false, the target will be the upstream project."]
    pub fn set_mr_default_target_self(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mr_default_target_self = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace_id`.\nThe namespace (group or user) of the project. Defaults to your user."]
    pub fn set_namespace_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().namespace_id = Some(v.into());
        self
    }

    #[doc= "Set the field `only_allow_merge_if_all_discussions_are_resolved`.\nSet to true if you want allow merges only if all discussions are resolved."]
    pub fn set_only_allow_merge_if_all_discussions_are_resolved(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_allow_merge_if_all_discussions_are_resolved = Some(v.into());
        self
    }

    #[doc= "Set the field `only_allow_merge_if_pipeline_succeeds`.\nSet to true if you want allow merges only if a pipeline succeeds."]
    pub fn set_only_allow_merge_if_pipeline_succeeds(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_allow_merge_if_pipeline_succeeds = Some(v.into());
        self
    }

    #[doc= "Set the field `only_mirror_protected_branches`.\nEnable only mirror protected branches for a mirrored project."]
    pub fn set_only_mirror_protected_branches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_mirror_protected_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `packages_enabled`.\nEnable packages repository for the project."]
    pub fn set_packages_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().packages_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `pages_access_level`.\nEnable pages access control. Valid values are `public`, `private`, `enabled`, `disabled`."]
    pub fn set_pages_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pages_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\nThe path of the repository."]
    pub fn set_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path = Some(v.into());
        self
    }

    #[doc= "Set the field `pipelines_enabled`.\nEnable pipelines for the project. The `pipelines_enabled` field is being sent as `jobs_enabled` in the GitLab API calls."]
    pub fn set_pipelines_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pipelines_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `printing_merge_request_link_enabled`.\nShow link to create/view merge request when pushing from the command line"]
    pub fn set_printing_merge_request_link_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().printing_merge_request_link_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `public_builds`.\nIf true, jobs can be viewed by non-project members."]
    pub fn set_public_builds(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_builds = Some(v.into());
        self
    }

    #[doc= "Set the field `public_jobs`.\nIf true, jobs can be viewed by non-project members."]
    pub fn set_public_jobs(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_jobs = Some(v.into());
        self
    }

    #[doc= "Set the field `releases_access_level`.\nSet the releases access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_releases_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().releases_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `remove_source_branch_after_merge`.\nEnable `Delete source branch` option by default for all new merge requests."]
    pub fn set_remove_source_branch_after_merge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_source_branch_after_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_access_level`.\nSet the repository access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_repository_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().repository_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_storage`.\n\tWhich storage shard the repository is on. (administrator only)"]
    pub fn set_repository_storage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().repository_storage = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access_enabled`.\nAllow users to request member access."]
    pub fn set_request_access_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().request_access_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `requirements_access_level`.\nSet the requirements access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_requirements_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().requirements_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `resolve_outdated_diff_discussions`.\nAutomatically resolve merge request diffs discussions on lines changed with a push."]
    pub fn set_resolve_outdated_diff_discussions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().resolve_outdated_diff_discussions = Some(v.into());
        self
    }

    #[doc= "Set the field `restrict_user_defined_variables`.\nAllow only users with the Maintainer role to pass user-defined variables when triggering a pipeline."]
    pub fn set_restrict_user_defined_variables(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restrict_user_defined_variables = Some(v.into());
        self
    }

    #[doc= "Set the field `security_and_compliance_access_level`.\nSet the security and compliance access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_security_and_compliance_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_and_compliance_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_enabled`.\nEnable shared runners for this project."]
    pub fn set_shared_runners_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shared_runners_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_wait_for_default_branch_protection`.\nIf `true`, the default behavior to wait for the default branch protection to be created is skipped.\nThis is necessary if the current user is not an admin and the default branch protection is disabled on an instance-level.\nThere is currently no known way to determine if the default branch protection is disabled on an instance-level for non-admin users.\nThis attribute is only used during resource creation, thus changes are suppressed and the attribute cannot be imported.\n"]
    pub fn set_skip_wait_for_default_branch_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wait_for_default_branch_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `snippets_access_level`.\nSet the snippets access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_snippets_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snippets_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `snippets_enabled`.\nEnable snippets for the project."]
    pub fn set_snippets_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().snippets_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_commit_template`.\nTemplate used to create squash commit message in merge requests. (Introduced in GitLab 14.6.)"]
    pub fn set_squash_commit_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().squash_commit_template = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_option`.\nSquash commits when merge request. Valid values are `never`, `always`, `default_on`, or `default_off`. The default value is `default_off`. [GitLab >= 14.1]"]
    pub fn set_squash_option(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().squash_option = Some(v.into());
        self
    }

    #[doc= "Set the field `suggestion_commit_message`.\nThe commit message used to apply merge request suggestions."]
    pub fn set_suggestion_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().suggestion_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\nThe list of tags for a project; put array of tags, that should be finally assigned to a project. Use topics instead."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `template_name`.\nWhen used without use_custom_template, name of a built-in project template. When used with use_custom_template, name of a custom project template. This option is mutually exclusive with `template_project_id`."]
    pub fn set_template_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_name = Some(v.into());
        self
    }

    #[doc= "Set the field `template_project_id`.\nWhen used with use_custom_template, project ID of a custom project template. This is preferable to using template_name since template_name may be ambiguous (enterprise edition). This option is mutually exclusive with `template_name`. See `gitlab_group_project_file_template` to set a project as a template project. If a project has not been set as a template, using it here will result in an error."]
    pub fn set_template_project_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().template_project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `topics`.\nThe list of topics for the project."]
    pub fn set_topics(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().topics = Some(v.into());
        self
    }

    #[doc= "Set the field `use_custom_template`.\nUse either custom instance or group (with group_with_project_templates_id) project template (enterprise edition).\n\t\t~> When using a custom template, [Group Tokens won't work](https://docs.gitlab.com/15.7/ee/user/project/settings/import_export_troubleshooting.html#import-using-the-rest-api-fails-when-using-a-group-access-token). You must use a real user's Personal Access Token."]
    pub fn set_use_custom_template(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_custom_template = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility_level`.\nSet to `public` to create a public project. Valid values are `private`, `internal`, `public`."]
    pub fn set_visibility_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility_level = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_access_level`.\nSet the wiki access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn set_wiki_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().wiki_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_enabled`.\nEnable wiki for the project."]
    pub fn set_wiki_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wiki_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `container_expiration_policy`.\n"]
    pub fn set_container_expiration_policy(
        self,
        v: impl Into<BlockAssignable<ProjectContainerExpirationPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_expiration_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_expiration_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `push_rules`.\n"]
    pub fn set_push_rules(self, v: impl Into<BlockAssignable<ProjectPushRulesEl>>) -> Self {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ProjectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allow_merge_on_skipped_pipeline` after provisioning.\nSet to true if you want to treat skipped pipelines as if they finished with success."]
    pub fn allow_merge_on_skipped_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_on_skipped_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `analytics_access_level` after provisioning.\nSet the analytics access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn analytics_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approvals_before_merge` after provisioning.\nNumber of merge request approvals required for merging. Default is 0.\n  This field **does not** work well in combination with the `gitlab_project_approval_rule` resource\n  and is most likely gonna be deprecated in a future GitLab version (see [this upstream epic](https://gitlab.com/groups/gitlab-org/-/epics/7572)).\n  In the meantime we recommend against using this attribute and use `gitlab_project_approval_rule` instead.\n"]
    pub fn approvals_before_merge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_before_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archive_on_destroy` after provisioning.\nSet to `true` to archive the project instead of deleting on destroy. If set to `true` it will entire omit the `DELETE` operation."]
    pub fn archive_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nWhether the project is in read-only mode (archived). Repositories can be archived/unarchived by toggling this parameter."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_cancel_pending_pipelines` after provisioning.\nAuto-cancel pending pipelines. This isn’t a boolean, but enabled/disabled."]
    pub fn auto_cancel_pending_pipelines(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_cancel_pending_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_deploy_strategy` after provisioning.\nAuto Deploy strategy. Valid values are `continuous`, `manual`, `timed_incremental`."]
    pub fn auto_devops_deploy_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_deploy_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nEnable Auto DevOps for this project."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoclose_referenced_issues` after provisioning.\nSet whether auto-closing referenced issues on default branch."]
    pub fn autoclose_referenced_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoclose_referenced_issues", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `build_coverage_regex` after provisioning.\nTest coverage parsing for the project. This is deprecated feature in GitLab 15.0."]
    pub fn build_coverage_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_coverage_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_git_strategy` after provisioning.\nThe Git strategy. Defaults to fetch. Valid values are `clone`, `fetch`."]
    pub fn build_git_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_git_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_timeout` after provisioning.\nThe maximum amount of time, in seconds, that a job can run."]
    pub fn build_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `builds_access_level` after provisioning.\nSet the builds access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn builds_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.builds_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_config_path` after provisioning.\nCustom Path to CI config file."]
    pub fn ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_default_git_depth` after provisioning.\nDefault number of revisions for shallow cloning."]
    pub fn ci_default_git_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_default_git_depth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_forward_deployment_enabled` after provisioning.\nWhen a new deployment job starts, skip older deployment jobs that are still pending."]
    pub fn ci_forward_deployment_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_forward_deployment_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_restrict_pipeline_cancellation_role` after provisioning.\nThe role required to cancel a pipeline or job. Introduced in GitLab 16.8. Premium and Ultimate only. Valid values are `developer`, `maintainer`, `no one`"]
    pub fn ci_restrict_pipeline_cancellation_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ci_restrict_pipeline_cancellation_role", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `ci_separated_caches` after provisioning.\nUse separate caches for protected branches."]
    pub fn ci_separated_caches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_separated_caches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_access_level` after provisioning.\nSet visibility of container registry, for this project. Valid values are `disabled`, `private`, `enabled`."]
    pub fn container_registry_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_enabled` after provisioning.\nEnable container registry for the project."]
    pub fn container_registry_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\nThe default branch for the project."]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the project."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\nEnable email notifications."]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `empty_repo` after provisioning.\nWhether the project is empty."]
    pub fn empty_repo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.empty_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environments_access_level` after provisioning.\nSet the environments access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn environments_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environments_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_authorization_classification_label` after provisioning.\nThe classification label for the project."]
    pub fn external_authorization_classification_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_classification_label", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `feature_flags_access_level` after provisioning.\nSet the feature flags access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn feature_flags_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_flags_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forked_from_project_id` after provisioning.\nThe id of the project to fork. During create the project is forked and during an update the fork relation is changed."]
    pub fn forked_from_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.forked_from_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forking_access_level` after provisioning.\nSet the forking access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn forking_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forking_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_runners_enabled` after provisioning.\nEnable group runners for this project."]
    pub fn group_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_with_project_templates_id` after provisioning.\nFor group-level custom templates, specifies ID of group from which all the custom project templates are sourced. Leave empty for instance-level templates. Requires use_custom_template to be true (enterprise edition)."]
    pub fn group_with_project_templates_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_with_project_templates_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url` after provisioning.\nGit URL to a repository to be imported. Together with `mirror = true` it will setup a Pull Mirror. This can also be used together with `forked_from_project_id` to setup a Pull Mirror for a fork. The fork takes precedence over the import. Make sure to provide the credentials in `import_url_username` and `import_url_password`. GitLab never returns the credentials, thus the provider cannot detect configuration drift in the credentials. They can also not be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn import_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url_password` after provisioning.\nThe password for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn import_url_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url_username` after provisioning.\nThe username for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`.  See the examples section for how to properly use it."]
    pub fn import_url_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_access_level` after provisioning.\nSet the infrastructure access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn infrastructure_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initialize_with_readme` after provisioning.\nCreate main branch with first commit containing a README.md file. Must be set to `true` if importing an uninitialized project with a different `default_branch`."]
    pub fn initialize_with_readme(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.initialize_with_readme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_access_level` after provisioning.\nSet the issues access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn issues_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_enabled` after provisioning.\nEnable issue tracking for the project."]
    pub fn issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_template` after provisioning.\nSets the template for new issues in the project."]
    pub fn issues_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_latest_artifact` after provisioning.\nDisable or enable the ability to keep the latest artifact for this project."]
    pub fn keep_latest_artifact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_latest_artifact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nEnable LFS for the project."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_template` after provisioning.\nTemplate used to create merge commit message in merge requests. (Introduced in GitLab 14.5.)"]
    pub fn merge_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_method` after provisioning.\nSet the merge method. Valid values are `merge`, `rebase_merge`, `ff`."]
    pub fn merge_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_pipelines_enabled` after provisioning.\nEnable or disable merge pipelines."]
    pub fn merge_pipelines_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_pipelines_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_access_level` after provisioning.\nSet the merge requests access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn merge_requests_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_enabled` after provisioning.\nEnable merge requests for the project."]
    pub fn merge_requests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_template` after provisioning.\nSets the template for new merge requests in the project."]
    pub fn merge_requests_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_trains_enabled` after provisioning.\nEnable or disable merge trains. Requires `merge_pipelines_enabled` to be set to `true` to take effect."]
    pub fn merge_trains_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_trains_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror` after provisioning.\nEnable project pull mirror."]
    pub fn mirror(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_overwrites_diverged_branches` after provisioning.\nEnable overwrite diverged branches for a mirrored project."]
    pub fn mirror_overwrites_diverged_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_overwrites_diverged_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_trigger_builds` after provisioning.\nEnable trigger builds on pushes for a mirrored project."]
    pub fn mirror_trigger_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_trigger_builds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor_access_level` after provisioning.\nSet the monitor access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn monitor_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mr_default_target_self` after provisioning.\nFor forked projects, target merge requests to this project. If false, the target will be the upstream project."]
    pub fn mr_default_target_self(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mr_default_target_self", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe namespace (group or user) of the project. Defaults to your user."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_all_discussions_are_resolved` after provisioning.\nSet to true if you want allow merges only if all discussions are resolved."]
    pub fn only_allow_merge_if_all_discussions_are_resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_allow_merge_if_all_discussions_are_resolved", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_pipeline_succeeds` after provisioning.\nSet to true if you want allow merges only if a pipeline succeeds."]
    pub fn only_allow_merge_if_pipeline_succeeds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_allow_merge_if_pipeline_succeeds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_mirror_protected_branches` after provisioning.\nEnable only mirror protected branches for a mirrored project."]
    pub fn only_mirror_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_mirror_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `packages_enabled` after provisioning.\nEnable packages repository for the project."]
    pub fn packages_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.packages_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_access_level` after provisioning.\nEnable pages access control. Valid values are `public`, `private`, `enabled`, `disabled`."]
    pub fn pages_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pages_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the repository."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\nThe path of the repository with namespace."]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipelines_enabled` after provisioning.\nEnable pipelines for the project. The `pipelines_enabled` field is being sent as `jobs_enabled` in the GitLab API calls."]
    pub fn pipelines_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipelines_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `printing_merge_request_link_enabled` after provisioning.\nShow link to create/view merge request when pushing from the command line"]
    pub fn printing_merge_request_link_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.printing_merge_request_link_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_builds` after provisioning.\nIf true, jobs can be viewed by non-project members."]
    pub fn public_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_builds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_jobs` after provisioning.\nIf true, jobs can be viewed by non-project members."]
    pub fn public_jobs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_jobs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_access_level` after provisioning.\nSet the releases access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn releases_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_source_branch_after_merge` after provisioning.\nEnable `Delete source branch` option by default for all new merge requests."]
    pub fn remove_source_branch_after_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_source_branch_after_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_access_level` after provisioning.\nSet the repository access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn repository_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storage` after provisioning.\n\tWhich storage shard the repository is on. (administrator only)"]
    pub fn repository_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_storage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nAllow users to request member access."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `requirements_access_level` after provisioning.\nSet the requirements access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn requirements_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolve_outdated_diff_discussions` after provisioning.\nAutomatically resolve merge request diffs discussions on lines changed with a push."]
    pub fn resolve_outdated_diff_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_outdated_diff_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrict_user_defined_variables` after provisioning.\nAllow only users with the Maintainer role to pass user-defined variables when triggering a pipeline."]
    pub fn restrict_user_defined_variables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrict_user_defined_variables", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nRegistration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_and_compliance_access_level` after provisioning.\nSet the security and compliance access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn security_and_compliance_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_and_compliance_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_enabled` after provisioning.\nEnable shared runners for this project."]
    pub fn shared_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_wait_for_default_branch_protection` after provisioning.\nIf `true`, the default behavior to wait for the default branch protection to be created is skipped.\nThis is necessary if the current user is not an admin and the default branch protection is disabled on an instance-level.\nThere is currently no known way to determine if the default branch protection is disabled on an instance-level for non-admin users.\nThis attribute is only used during resource creation, thus changes are suppressed and the attribute cannot be imported.\n"]
    pub fn skip_wait_for_default_branch_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait_for_default_branch_protection", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `snippets_access_level` after provisioning.\nSet the snippets access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn snippets_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snippets_enabled` after provisioning.\nEnable snippets for the project."]
    pub fn snippets_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_commit_template` after provisioning.\nTemplate used to create squash commit message in merge requests. (Introduced in GitLab 14.6.)"]
    pub fn squash_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_commit_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_option` after provisioning.\nSquash commits when merge request. Valid values are `never`, `always`, `default_on`, or `default_off`. The default value is `default_off`. [GitLab >= 14.1]"]
    pub fn squash_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_option", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn ssh_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggestion_commit_message` after provisioning.\nThe commit message used to apply merge request suggestions."]
    pub fn suggestion_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggestion_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nThe list of tags for a project; put array of tags, that should be finally assigned to a project. Use topics instead."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_name` after provisioning.\nWhen used without use_custom_template, name of a built-in project template. When used with use_custom_template, name of a custom project template. This option is mutually exclusive with `template_project_id`."]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_project_id` after provisioning.\nWhen used with use_custom_template, project ID of a custom project template. This is preferable to using template_name since template_name may be ambiguous (enterprise edition). This option is mutually exclusive with `template_name`. See `gitlab_group_project_file_template` to set a project as a template project. If a project has not been set as a template, using it here will result in an error."]
    pub fn template_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics for the project."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_custom_template` after provisioning.\nUse either custom instance or group (with group_with_project_templates_id) project template (enterprise edition).\n\t\t~> When using a custom template, [Group Tokens won't work](https://docs.gitlab.com/15.7/ee/user/project/settings/import_export_troubleshooting.html#import-using-the-rest-api-fails-when-using-a-group-access-token). You must use a real user's Personal Access Token."]
    pub fn use_custom_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_custom_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nSet to `public` to create a public project. Valid values are `private`, `internal`, `public`."]
    pub fn visibility_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nURL that can be used to find the project in a browser."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\nSet the wiki access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_enabled` after provisioning.\nEnable wiki for the project."]
    pub fn wiki_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_expiration_policy` after provisioning.\n"]
    pub fn container_expiration_policy(&self) -> ListRef<ProjectContainerExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\n"]
    pub fn push_rules(&self) -> ListRef<ProjectPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ProjectTimeoutsElRef {
        ProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Project {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Project { }

impl ToListMappable for Project {
    type O = ListRef<ProjectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Project_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProject {
    pub tf_id: String,
    #[doc= "The name of the project."]
    pub name: PrimField<String>,
}

impl BuildProject {
    pub fn build(self, stack: &mut Stack) -> Project {
        let out = Project(Rc::new(Project_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_merge_on_skipped_pipeline: core::default::Default::default(),
                analytics_access_level: core::default::Default::default(),
                approvals_before_merge: core::default::Default::default(),
                archive_on_destroy: core::default::Default::default(),
                archived: core::default::Default::default(),
                auto_cancel_pending_pipelines: core::default::Default::default(),
                auto_devops_deploy_strategy: core::default::Default::default(),
                auto_devops_enabled: core::default::Default::default(),
                autoclose_referenced_issues: core::default::Default::default(),
                avatar: core::default::Default::default(),
                avatar_hash: core::default::Default::default(),
                build_coverage_regex: core::default::Default::default(),
                build_git_strategy: core::default::Default::default(),
                build_timeout: core::default::Default::default(),
                builds_access_level: core::default::Default::default(),
                ci_config_path: core::default::Default::default(),
                ci_default_git_depth: core::default::Default::default(),
                ci_forward_deployment_enabled: core::default::Default::default(),
                ci_restrict_pipeline_cancellation_role: core::default::Default::default(),
                ci_separated_caches: core::default::Default::default(),
                container_registry_access_level: core::default::Default::default(),
                container_registry_enabled: core::default::Default::default(),
                default_branch: core::default::Default::default(),
                description: core::default::Default::default(),
                emails_enabled: core::default::Default::default(),
                environments_access_level: core::default::Default::default(),
                external_authorization_classification_label: core::default::Default::default(),
                feature_flags_access_level: core::default::Default::default(),
                forked_from_project_id: core::default::Default::default(),
                forking_access_level: core::default::Default::default(),
                group_runners_enabled: core::default::Default::default(),
                group_with_project_templates_id: core::default::Default::default(),
                id: core::default::Default::default(),
                import_url: core::default::Default::default(),
                import_url_password: core::default::Default::default(),
                import_url_username: core::default::Default::default(),
                infrastructure_access_level: core::default::Default::default(),
                initialize_with_readme: core::default::Default::default(),
                issues_access_level: core::default::Default::default(),
                issues_enabled: core::default::Default::default(),
                issues_template: core::default::Default::default(),
                keep_latest_artifact: core::default::Default::default(),
                lfs_enabled: core::default::Default::default(),
                merge_commit_template: core::default::Default::default(),
                merge_method: core::default::Default::default(),
                merge_pipelines_enabled: core::default::Default::default(),
                merge_requests_access_level: core::default::Default::default(),
                merge_requests_enabled: core::default::Default::default(),
                merge_requests_template: core::default::Default::default(),
                merge_trains_enabled: core::default::Default::default(),
                mirror: core::default::Default::default(),
                mirror_overwrites_diverged_branches: core::default::Default::default(),
                mirror_trigger_builds: core::default::Default::default(),
                monitor_access_level: core::default::Default::default(),
                mr_default_target_self: core::default::Default::default(),
                name: self.name,
                namespace_id: core::default::Default::default(),
                only_allow_merge_if_all_discussions_are_resolved: core::default::Default::default(),
                only_allow_merge_if_pipeline_succeeds: core::default::Default::default(),
                only_mirror_protected_branches: core::default::Default::default(),
                packages_enabled: core::default::Default::default(),
                pages_access_level: core::default::Default::default(),
                path: core::default::Default::default(),
                pipelines_enabled: core::default::Default::default(),
                printing_merge_request_link_enabled: core::default::Default::default(),
                public_builds: core::default::Default::default(),
                public_jobs: core::default::Default::default(),
                releases_access_level: core::default::Default::default(),
                remove_source_branch_after_merge: core::default::Default::default(),
                repository_access_level: core::default::Default::default(),
                repository_storage: core::default::Default::default(),
                request_access_enabled: core::default::Default::default(),
                requirements_access_level: core::default::Default::default(),
                resolve_outdated_diff_discussions: core::default::Default::default(),
                restrict_user_defined_variables: core::default::Default::default(),
                security_and_compliance_access_level: core::default::Default::default(),
                shared_runners_enabled: core::default::Default::default(),
                skip_wait_for_default_branch_protection: core::default::Default::default(),
                snippets_access_level: core::default::Default::default(),
                snippets_enabled: core::default::Default::default(),
                squash_commit_template: core::default::Default::default(),
                squash_option: core::default::Default::default(),
                suggestion_commit_message: core::default::Default::default(),
                tags: core::default::Default::default(),
                template_name: core::default::Default::default(),
                template_project_id: core::default::Default::default(),
                topics: core::default::Default::default(),
                use_custom_template: core::default::Default::default(),
                visibility_level: core::default::Default::default(),
                wiki_access_level: core::default::Default::default(),
                wiki_enabled: core::default::Default::default(),
                container_expiration_policy: core::default::Default::default(),
                push_rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_merge_on_skipped_pipeline` after provisioning.\nSet to true if you want to treat skipped pipelines as if they finished with success."]
    pub fn allow_merge_on_skipped_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_on_skipped_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `analytics_access_level` after provisioning.\nSet the analytics access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn analytics_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approvals_before_merge` after provisioning.\nNumber of merge request approvals required for merging. Default is 0.\n  This field **does not** work well in combination with the `gitlab_project_approval_rule` resource\n  and is most likely gonna be deprecated in a future GitLab version (see [this upstream epic](https://gitlab.com/groups/gitlab-org/-/epics/7572)).\n  In the meantime we recommend against using this attribute and use `gitlab_project_approval_rule` instead.\n"]
    pub fn approvals_before_merge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_before_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archive_on_destroy` after provisioning.\nSet to `true` to archive the project instead of deleting on destroy. If set to `true` it will entire omit the `DELETE` operation."]
    pub fn archive_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nWhether the project is in read-only mode (archived). Repositories can be archived/unarchived by toggling this parameter."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_cancel_pending_pipelines` after provisioning.\nAuto-cancel pending pipelines. This isn’t a boolean, but enabled/disabled."]
    pub fn auto_cancel_pending_pipelines(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_cancel_pending_pipelines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_deploy_strategy` after provisioning.\nAuto Deploy strategy. Valid values are `continuous`, `manual`, `timed_incremental`."]
    pub fn auto_devops_deploy_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_deploy_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nEnable Auto DevOps for this project."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `autoclose_referenced_issues` after provisioning.\nSet whether auto-closing referenced issues on default branch."]
    pub fn autoclose_referenced_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoclose_referenced_issues", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `build_coverage_regex` after provisioning.\nTest coverage parsing for the project. This is deprecated feature in GitLab 15.0."]
    pub fn build_coverage_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_coverage_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_git_strategy` after provisioning.\nThe Git strategy. Defaults to fetch. Valid values are `clone`, `fetch`."]
    pub fn build_git_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_git_strategy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_timeout` after provisioning.\nThe maximum amount of time, in seconds, that a job can run."]
    pub fn build_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `builds_access_level` after provisioning.\nSet the builds access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn builds_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.builds_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_config_path` after provisioning.\nCustom Path to CI config file."]
    pub fn ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_default_git_depth` after provisioning.\nDefault number of revisions for shallow cloning."]
    pub fn ci_default_git_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_default_git_depth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_forward_deployment_enabled` after provisioning.\nWhen a new deployment job starts, skip older deployment jobs that are still pending."]
    pub fn ci_forward_deployment_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_forward_deployment_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_restrict_pipeline_cancellation_role` after provisioning.\nThe role required to cancel a pipeline or job. Introduced in GitLab 16.8. Premium and Ultimate only. Valid values are `developer`, `maintainer`, `no one`"]
    pub fn ci_restrict_pipeline_cancellation_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ci_restrict_pipeline_cancellation_role", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `ci_separated_caches` after provisioning.\nUse separate caches for protected branches."]
    pub fn ci_separated_caches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_separated_caches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_access_level` after provisioning.\nSet visibility of container registry, for this project. Valid values are `disabled`, `private`, `enabled`."]
    pub fn container_registry_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_enabled` after provisioning.\nEnable container registry for the project."]
    pub fn container_registry_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\nThe default branch for the project."]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the project."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\nEnable email notifications."]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `empty_repo` after provisioning.\nWhether the project is empty."]
    pub fn empty_repo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.empty_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environments_access_level` after provisioning.\nSet the environments access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn environments_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environments_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_authorization_classification_label` after provisioning.\nThe classification label for the project."]
    pub fn external_authorization_classification_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_classification_label", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `feature_flags_access_level` after provisioning.\nSet the feature flags access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn feature_flags_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_flags_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forked_from_project_id` after provisioning.\nThe id of the project to fork. During create the project is forked and during an update the fork relation is changed."]
    pub fn forked_from_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.forked_from_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forking_access_level` after provisioning.\nSet the forking access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn forking_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forking_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_runners_enabled` after provisioning.\nEnable group runners for this project."]
    pub fn group_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_with_project_templates_id` after provisioning.\nFor group-level custom templates, specifies ID of group from which all the custom project templates are sourced. Leave empty for instance-level templates. Requires use_custom_template to be true (enterprise edition)."]
    pub fn group_with_project_templates_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_with_project_templates_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url` after provisioning.\nGit URL to a repository to be imported. Together with `mirror = true` it will setup a Pull Mirror. This can also be used together with `forked_from_project_id` to setup a Pull Mirror for a fork. The fork takes precedence over the import. Make sure to provide the credentials in `import_url_username` and `import_url_password`. GitLab never returns the credentials, thus the provider cannot detect configuration drift in the credentials. They can also not be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn import_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url_password` after provisioning.\nThe password for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`. See the examples section for how to properly use it."]
    pub fn import_url_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url_username` after provisioning.\nThe username for the `import_url`. The value of this field is used to construct a valid `import_url` and is only related to the provider. This field cannot be imported using `terraform import`.  See the examples section for how to properly use it."]
    pub fn import_url_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_access_level` after provisioning.\nSet the infrastructure access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn infrastructure_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `initialize_with_readme` after provisioning.\nCreate main branch with first commit containing a README.md file. Must be set to `true` if importing an uninitialized project with a different `default_branch`."]
    pub fn initialize_with_readme(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.initialize_with_readme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_access_level` after provisioning.\nSet the issues access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn issues_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_enabled` after provisioning.\nEnable issue tracking for the project."]
    pub fn issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_template` after provisioning.\nSets the template for new issues in the project."]
    pub fn issues_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_latest_artifact` after provisioning.\nDisable or enable the ability to keep the latest artifact for this project."]
    pub fn keep_latest_artifact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_latest_artifact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\nEnable LFS for the project."]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_template` after provisioning.\nTemplate used to create merge commit message in merge requests. (Introduced in GitLab 14.5.)"]
    pub fn merge_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_method` after provisioning.\nSet the merge method. Valid values are `merge`, `rebase_merge`, `ff`."]
    pub fn merge_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_method", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_pipelines_enabled` after provisioning.\nEnable or disable merge pipelines."]
    pub fn merge_pipelines_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_pipelines_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_access_level` after provisioning.\nSet the merge requests access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn merge_requests_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_enabled` after provisioning.\nEnable merge requests for the project."]
    pub fn merge_requests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_template` after provisioning.\nSets the template for new merge requests in the project."]
    pub fn merge_requests_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_trains_enabled` after provisioning.\nEnable or disable merge trains. Requires `merge_pipelines_enabled` to be set to `true` to take effect."]
    pub fn merge_trains_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_trains_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror` after provisioning.\nEnable project pull mirror."]
    pub fn mirror(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_overwrites_diverged_branches` after provisioning.\nEnable overwrite diverged branches for a mirrored project."]
    pub fn mirror_overwrites_diverged_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_overwrites_diverged_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_trigger_builds` after provisioning.\nEnable trigger builds on pushes for a mirrored project."]
    pub fn mirror_trigger_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_trigger_builds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor_access_level` after provisioning.\nSet the monitor access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn monitor_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mr_default_target_self` after provisioning.\nFor forked projects, target merge requests to this project. If false, the target will be the upstream project."]
    pub fn mr_default_target_self(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mr_default_target_self", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe namespace (group or user) of the project. Defaults to your user."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_all_discussions_are_resolved` after provisioning.\nSet to true if you want allow merges only if all discussions are resolved."]
    pub fn only_allow_merge_if_all_discussions_are_resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_allow_merge_if_all_discussions_are_resolved", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_pipeline_succeeds` after provisioning.\nSet to true if you want allow merges only if a pipeline succeeds."]
    pub fn only_allow_merge_if_pipeline_succeeds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_allow_merge_if_pipeline_succeeds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_mirror_protected_branches` after provisioning.\nEnable only mirror protected branches for a mirrored project."]
    pub fn only_mirror_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_mirror_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `packages_enabled` after provisioning.\nEnable packages repository for the project."]
    pub fn packages_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.packages_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_access_level` after provisioning.\nEnable pages access control. Valid values are `public`, `private`, `enabled`, `disabled`."]
    pub fn pages_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pages_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the repository."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\nThe path of the repository with namespace."]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipelines_enabled` after provisioning.\nEnable pipelines for the project. The `pipelines_enabled` field is being sent as `jobs_enabled` in the GitLab API calls."]
    pub fn pipelines_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipelines_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `printing_merge_request_link_enabled` after provisioning.\nShow link to create/view merge request when pushing from the command line"]
    pub fn printing_merge_request_link_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.printing_merge_request_link_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_builds` after provisioning.\nIf true, jobs can be viewed by non-project members."]
    pub fn public_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_builds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_jobs` after provisioning.\nIf true, jobs can be viewed by non-project members."]
    pub fn public_jobs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_jobs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_access_level` after provisioning.\nSet the releases access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn releases_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_source_branch_after_merge` after provisioning.\nEnable `Delete source branch` option by default for all new merge requests."]
    pub fn remove_source_branch_after_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_source_branch_after_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_access_level` after provisioning.\nSet the repository access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn repository_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storage` after provisioning.\n\tWhich storage shard the repository is on. (administrator only)"]
    pub fn repository_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_storage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\nAllow users to request member access."]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `requirements_access_level` after provisioning.\nSet the requirements access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn requirements_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resolve_outdated_diff_discussions` after provisioning.\nAutomatically resolve merge request diffs discussions on lines changed with a push."]
    pub fn resolve_outdated_diff_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_outdated_diff_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrict_user_defined_variables` after provisioning.\nAllow only users with the Maintainer role to pass user-defined variables when triggering a pipeline."]
    pub fn restrict_user_defined_variables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrict_user_defined_variables", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\nRegistration token to use during runner setup."]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_and_compliance_access_level` after provisioning.\nSet the security and compliance access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn security_and_compliance_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_and_compliance_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_enabled` after provisioning.\nEnable shared runners for this project."]
    pub fn shared_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_wait_for_default_branch_protection` after provisioning.\nIf `true`, the default behavior to wait for the default branch protection to be created is skipped.\nThis is necessary if the current user is not an admin and the default branch protection is disabled on an instance-level.\nThere is currently no known way to determine if the default branch protection is disabled on an instance-level for non-admin users.\nThis attribute is only used during resource creation, thus changes are suppressed and the attribute cannot be imported.\n"]
    pub fn skip_wait_for_default_branch_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_wait_for_default_branch_protection", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `snippets_access_level` after provisioning.\nSet the snippets access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn snippets_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snippets_enabled` after provisioning.\nEnable snippets for the project."]
    pub fn snippets_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_commit_template` after provisioning.\nTemplate used to create squash commit message in merge requests. (Introduced in GitLab 14.6.)"]
    pub fn squash_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_commit_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_option` after provisioning.\nSquash commits when merge request. Valid values are `never`, `always`, `default_on`, or `default_off`. The default value is `default_off`. [GitLab >= 14.1]"]
    pub fn squash_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_option", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn ssh_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggestion_commit_message` after provisioning.\nThe commit message used to apply merge request suggestions."]
    pub fn suggestion_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggestion_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nThe list of tags for a project; put array of tags, that should be finally assigned to a project. Use topics instead."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_name` after provisioning.\nWhen used without use_custom_template, name of a built-in project template. When used with use_custom_template, name of a custom project template. This option is mutually exclusive with `template_project_id`."]
    pub fn template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template_project_id` after provisioning.\nWhen used with use_custom_template, project ID of a custom project template. This is preferable to using template_name since template_name may be ambiguous (enterprise edition). This option is mutually exclusive with `template_name`. See `gitlab_group_project_file_template` to set a project as a template project. If a project has not been set as a template, using it here will result in an error."]
    pub fn template_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics for the project."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_custom_template` after provisioning.\nUse either custom instance or group (with group_with_project_templates_id) project template (enterprise edition).\n\t\t~> When using a custom template, [Group Tokens won't work](https://docs.gitlab.com/15.7/ee/user/project/settings/import_export_troubleshooting.html#import-using-the-rest-api-fails-when-using-a-group-access-token). You must use a real user's Personal Access Token."]
    pub fn use_custom_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_custom_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nSet to `public` to create a public project. Valid values are `private`, `internal`, `public`."]
    pub fn visibility_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nURL that can be used to find the project in a browser."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\nSet the wiki access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_enabled` after provisioning.\nEnable wiki for the project."]
    pub fn wiki_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_expiration_policy` after provisioning.\n"]
    pub fn container_expiration_policy(&self) -> ListRef<ProjectContainerExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\n"]
    pub fn push_rules(&self) -> ListRef<ProjectPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ProjectTimeoutsElRef {
        ProjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ProjectContainerExpirationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_n: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex_delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex_keep: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    older_than: Option<PrimField<String>>,
}

impl ProjectContainerExpirationPolicyEl {
    #[doc= "Set the field `cadence`.\nThe cadence of the policy. Valid values are: `1d`, `7d`, `14d`, `1month`, `3month`."]
    pub fn set_cadence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cadence = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nIf true, the policy is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_n`.\nThe number of images to keep."]
    pub fn set_keep_n(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keep_n = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex`.\nThe regular expression to match image names to delete."]
    pub fn set_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_delete`.\nThe regular expression to match image names to delete."]
    pub fn set_name_regex_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_keep`.\nThe regular expression to match image names to keep."]
    pub fn set_name_regex_keep(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_keep = Some(v.into());
        self
    }

    #[doc= "Set the field `older_than`.\nThe number of days to keep images."]
    pub fn set_older_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.older_than = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectContainerExpirationPolicyEl {
    type O = BlockAssignable<ProjectContainerExpirationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectContainerExpirationPolicyEl {}

impl BuildProjectContainerExpirationPolicyEl {
    pub fn build(self) -> ProjectContainerExpirationPolicyEl {
        ProjectContainerExpirationPolicyEl {
            cadence: core::default::Default::default(),
            enabled: core::default::Default::default(),
            keep_n: core::default::Default::default(),
            name_regex: core::default::Default::default(),
            name_regex_delete: core::default::Default::default(),
            name_regex_keep: core::default::Default::default(),
            older_than: core::default::Default::default(),
        }
    }
}

pub struct ProjectContainerExpirationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectContainerExpirationPolicyElRef {
    fn new(shared: StackShared, base: String) -> ProjectContainerExpirationPolicyElRef {
        ProjectContainerExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectContainerExpirationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cadence` after provisioning.\nThe cadence of the policy. Valid values are: `1d`, `7d`, `14d`, `1month`, `3month`."]
    pub fn cadence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cadence", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf true, the policy is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `keep_n` after provisioning.\nThe number of images to keep."]
    pub fn keep_n(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_n", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex` after provisioning.\nThe regular expression to match image names to delete."]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_delete` after provisioning.\nThe regular expression to match image names to delete."]
    pub fn name_regex_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_keep` after provisioning.\nThe regular expression to match image names to keep."]
    pub fn name_regex_keep(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_keep", self.base))
    }

    #[doc= "Get a reference to the value of field `next_run_at` after provisioning.\nThe next time the policy will run."]
    pub fn next_run_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_run_at", self.base))
    }

    #[doc= "Get a reference to the value of field `older_than` after provisioning.\nThe number of days to keep images."]
    pub fn older_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectPushRulesEl {
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

impl ProjectPushRulesEl {
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

    #[doc= "Set the field `commit_committer_check`.\nUsers can only push commits to this repository that were committed with one of their own verified emails."]
    pub fn set_commit_committer_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_name_check`.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn set_commit_committer_name_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_name_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_negative_regex`.\nNo commit message is allowed to match this regex, e.g. `ssh\\:\\/\\/`."]
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

    #[doc= "Set the field `file_name_regex`.\nAll committed filenames must not match this regex, e.g. `(jar|exe)$`."]
    pub fn set_file_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `max_file_size`.\nMaximum file size (MB)."]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `member_check`.\nRestrict commits by author (email) to existing GitLab users."]
    pub fn set_member_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.member_check = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_secrets`.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn set_prevent_secrets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `reject_unsigned_commits`.\nReject commit when it’s not signed through GPG."]
    pub fn set_reject_unsigned_commits(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reject_unsigned_commits = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectPushRulesEl {
    type O = BlockAssignable<ProjectPushRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectPushRulesEl {}

impl BuildProjectPushRulesEl {
    pub fn build(self) -> ProjectPushRulesEl {
        ProjectPushRulesEl {
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

pub struct ProjectPushRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectPushRulesElRef {
    fn new(shared: StackShared, base: String) -> ProjectPushRulesElRef {
        ProjectPushRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectPushRulesElRef {
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

    #[doc= "Get a reference to the value of field `commit_committer_check` after provisioning.\nUsers can only push commits to this repository that were committed with one of their own verified emails."]
    pub fn commit_committer_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_committer_name_check` after provisioning.\nUsers can only push commits to this repository if the commit author name is consistent with their GitLab account name."]
    pub fn commit_committer_name_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_name_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_negative_regex` after provisioning.\nNo commit message is allowed to match this regex, e.g. `ssh\\:\\/\\/`."]
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

    #[doc= "Get a reference to the value of field `file_name_regex` after provisioning.\nAll committed filenames must not match this regex, e.g. `(jar|exe)$`."]
    pub fn file_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\nMaximum file size (MB)."]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc= "Get a reference to the value of field `member_check` after provisioning.\nRestrict commits by author (email) to existing GitLab users."]
    pub fn member_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_check", self.base))
    }

    #[doc= "Get a reference to the value of field `prevent_secrets` after provisioning.\nGitLab will reject any files that are likely to contain secrets."]
    pub fn prevent_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `reject_unsigned_commits` after provisioning.\nReject commit when it’s not signed through GPG."]
    pub fn reject_unsigned_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reject_unsigned_commits", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ProjectTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectTimeoutsEl {
    type O = BlockAssignable<ProjectTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectTimeoutsEl {}

impl BuildProjectTimeoutsEl {
    pub fn build(self) -> ProjectTimeoutsEl {
        ProjectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ProjectTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ProjectTimeoutsElRef {
        ProjectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProjectDynamic {
    container_expiration_policy: Option<DynamicBlock<ProjectContainerExpirationPolicyEl>>,
    push_rules: Option<DynamicBlock<ProjectPushRulesEl>>,
}

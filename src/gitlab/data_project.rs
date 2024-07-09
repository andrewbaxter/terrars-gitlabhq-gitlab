use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_default_git_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_builds: Option<PrimField<bool>>,
}

struct DataProject_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectData>,
}

#[derive(Clone)]
pub struct DataProject(Rc<DataProject_>);

impl DataProject {
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

    #[doc= "Set the field `ci_default_git_depth`.\nDefault number of revisions for shallow cloning."]
    pub fn set_ci_default_git_depth(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ci_default_git_depth = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nThe integer that uniquely identifies the project within the gitlab install."]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `path_with_namespace`.\nThe path of the repository with namespace."]
    pub fn set_path_with_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().path_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `public_builds`.\nIf true, jobs can be viewed by non-project members."]
    pub fn set_public_builds(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_builds = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `analytics_access_level` after provisioning.\nSet the analytics access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn analytics_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nWhether the project is in read-only mode (archived)."]
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

    #[doc= "Get a reference to the value of field `build_git_strategy` after provisioning.\nThe Git strategy. Defaults to fetch."]
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

    #[doc= "Get a reference to the value of field `ci_config_path` after provisioning.\nCI config file path for the project."]
    pub fn ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_default_git_depth` after provisioning.\nDefault number of revisions for shallow cloning."]
    pub fn ci_default_git_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_default_git_depth", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `container_expiration_policy` after provisioning.\nSet the image cleanup policy for this project. **Note**: this field is sometimes named `container_expiration_policy_attributes` in the GitLab Upstream API."]
    pub fn container_expiration_policy(&self) -> ListRef<DataProjectContainerExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_access_level` after provisioning.\nSet visibility of container registry, for this project. Valid values are `disabled`, `private`, `enabled`."]
    pub fn container_registry_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_access_level", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `forking_access_level` after provisioning.\nSet the forking access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn forking_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forking_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe integer that uniquely identifies the project within the gitlab install."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url` after provisioning.\nURL the project was imported from."]
    pub fn import_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_access_level` after provisioning.\nSet the infrastructure access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn infrastructure_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_access_level` after provisioning.\nSet the issues access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn issues_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_enabled` after provisioning.\nEnable issue tracking for the project."]
    pub fn issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_enabled", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `merge_trains_enabled` after provisioning.\nEnable or disable merge trains."]
    pub fn merge_trains_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_trains_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor_access_level` after provisioning.\nSet the monitor access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn monitor_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe namespace (group or user) of the project. Defaults to your user."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the repository."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\nThe path of the repository with namespace."]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipelines_enabled` after provisioning.\nEnable pipelines for the project."]
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

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\nPush rules for the project. Push rules are only available on Enterprise plans and if the authenticated has permissions to read them."]
    pub fn push_rules(&self) -> ListRef<DataProjectPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_access_level` after provisioning.\nSet the releases access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn releases_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_source_branch_after_merge` after provisioning.\nEnable `Delete source branch` option by default for all new merge requests"]
    pub fn remove_source_branch_after_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_source_branch_after_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_access_level` after provisioning.\nSet the repository access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn repository_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storage` after provisioning.\nWhich storage shard the repository is on. (administrator only)"]
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

    #[doc= "Get a reference to the value of field `shared_with_groups` after provisioning.\nDescribes groups which have access shared to this project."]
    pub fn shared_with_groups(&self) -> ListRef<DataProjectSharedWithGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shared_with_groups", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `ssh_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn ssh_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggestion_commit_message` after provisioning.\nThe commit message used to apply merge request suggestions."]
    pub fn suggestion_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggestion_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics for the project."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nRepositories are created as private by default."]
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
}

impl Referable for DataProject {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProject { }

impl ToListMappable for DataProject {
    type O = ListRef<DataProjectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProject_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProject {
    pub tf_id: String,
}

impl BuildDataProject {
    pub fn build(self, stack: &mut Stack) -> DataProject {
        let out = DataProject(Rc::new(DataProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                ci_default_git_depth: core::default::Default::default(),
                id: core::default::Default::default(),
                path_with_namespace: core::default::Default::default(),
                public_builds: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `analytics_access_level` after provisioning.\nSet the analytics access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn analytics_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nWhether the project is in read-only mode (archived)."]
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

    #[doc= "Get a reference to the value of field `build_git_strategy` after provisioning.\nThe Git strategy. Defaults to fetch."]
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

    #[doc= "Get a reference to the value of field `ci_config_path` after provisioning.\nCI config file path for the project."]
    pub fn ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ci_default_git_depth` after provisioning.\nDefault number of revisions for shallow cloning."]
    pub fn ci_default_git_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_default_git_depth", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `container_expiration_policy` after provisioning.\nSet the image cleanup policy for this project. **Note**: this field is sometimes named `container_expiration_policy_attributes` in the GitLab Upstream API."]
    pub fn container_expiration_policy(&self) -> ListRef<DataProjectContainerExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_registry_access_level` after provisioning.\nSet visibility of container registry, for this project. Valid values are `disabled`, `private`, `enabled`."]
    pub fn container_registry_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_access_level", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `forking_access_level` after provisioning.\nSet the forking access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn forking_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forking_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe integer that uniquely identifies the project within the gitlab install."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_url` after provisioning.\nURL the project was imported from."]
    pub fn import_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_access_level` after provisioning.\nSet the infrastructure access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn infrastructure_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_access_level` after provisioning.\nSet the issues access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn issues_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_enabled` after provisioning.\nEnable issue tracking for the project."]
    pub fn issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_enabled", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `merge_trains_enabled` after provisioning.\nEnable or disable merge trains."]
    pub fn merge_trains_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_trains_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitor_access_level` after provisioning.\nSet the monitor access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn monitor_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe namespace (group or user) of the project. Defaults to your user."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe path of the repository."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\nThe path of the repository with namespace."]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipelines_enabled` after provisioning.\nEnable pipelines for the project."]
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

    #[doc= "Get a reference to the value of field `push_rules` after provisioning.\nPush rules for the project. Push rules are only available on Enterprise plans and if the authenticated has permissions to read them."]
    pub fn push_rules(&self) -> ListRef<DataProjectPushRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_access_level` after provisioning.\nSet the releases access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn releases_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_source_branch_after_merge` after provisioning.\nEnable `Delete source branch` option by default for all new merge requests"]
    pub fn remove_source_branch_after_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_source_branch_after_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_access_level` after provisioning.\nSet the repository access level. Valid values are `disabled`, `private`, `enabled`."]
    pub fn repository_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storage` after provisioning.\nWhich storage shard the repository is on. (administrator only)"]
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

    #[doc= "Get a reference to the value of field `shared_with_groups` after provisioning.\nDescribes groups which have access shared to this project."]
    pub fn shared_with_groups(&self) -> ListRef<DataProjectSharedWithGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shared_with_groups", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `ssh_url_to_repo` after provisioning.\nURL that can be provided to `git clone` to clone the"]
    pub fn ssh_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_url_to_repo", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggestion_commit_message` after provisioning.\nThe commit message used to apply merge request suggestions."]
    pub fn suggestion_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggestion_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics for the project."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility_level` after provisioning.\nRepositories are created as private by default."]
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
}

#[derive(Serialize)]
pub struct DataProjectContainerExpirationPolicyEl {
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
    next_run_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    older_than: Option<PrimField<String>>,
}

impl DataProjectContainerExpirationPolicyEl {
    #[doc= "Set the field `cadence`.\n"]
    pub fn set_cadence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cadence = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_n`.\n"]
    pub fn set_keep_n(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keep_n = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex`.\n"]
    pub fn set_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_delete`.\n"]
    pub fn set_name_regex_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_keep`.\n"]
    pub fn set_name_regex_keep(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_keep = Some(v.into());
        self
    }

    #[doc= "Set the field `next_run_at`.\n"]
    pub fn set_next_run_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_run_at = Some(v.into());
        self
    }

    #[doc= "Set the field `older_than`.\n"]
    pub fn set_older_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.older_than = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectContainerExpirationPolicyEl {
    type O = BlockAssignable<DataProjectContainerExpirationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectContainerExpirationPolicyEl {}

impl BuildDataProjectContainerExpirationPolicyEl {
    pub fn build(self) -> DataProjectContainerExpirationPolicyEl {
        DataProjectContainerExpirationPolicyEl {
            cadence: core::default::Default::default(),
            enabled: core::default::Default::default(),
            keep_n: core::default::Default::default(),
            name_regex: core::default::Default::default(),
            name_regex_delete: core::default::Default::default(),
            name_regex_keep: core::default::Default::default(),
            next_run_at: core::default::Default::default(),
            older_than: core::default::Default::default(),
        }
    }
}

pub struct DataProjectContainerExpirationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectContainerExpirationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataProjectContainerExpirationPolicyElRef {
        DataProjectContainerExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectContainerExpirationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cadence` after provisioning.\n"]
    pub fn cadence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cadence", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `keep_n` after provisioning.\n"]
    pub fn keep_n(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_n", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_delete` after provisioning.\n"]
    pub fn name_regex_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_keep` after provisioning.\n"]
    pub fn name_regex_keep(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_keep", self.base))
    }

    #[doc= "Get a reference to the value of field `next_run_at` after provisioning.\n"]
    pub fn next_run_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_run_at", self.base))
    }

    #[doc= "Get a reference to the value of field `older_than` after provisioning.\n"]
    pub fn older_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectPushRulesEl {
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

impl DataProjectPushRulesEl {
    #[doc= "Set the field `author_email_regex`.\n"]
    pub fn set_author_email_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.author_email_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `branch_name_regex`.\n"]
    pub fn set_branch_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_check`.\n"]
    pub fn set_commit_committer_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_committer_name_check`.\n"]
    pub fn set_commit_committer_name_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.commit_committer_name_check = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_negative_regex`.\n"]
    pub fn set_commit_message_negative_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_message_negative_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message_regex`.\n"]
    pub fn set_commit_message_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_message_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `deny_delete_tag`.\n"]
    pub fn set_deny_delete_tag(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deny_delete_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `file_name_regex`.\n"]
    pub fn set_file_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_file_size = Some(v.into());
        self
    }

    #[doc= "Set the field `member_check`.\n"]
    pub fn set_member_check(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.member_check = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_secrets`.\n"]
    pub fn set_prevent_secrets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `reject_unsigned_commits`.\n"]
    pub fn set_reject_unsigned_commits(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reject_unsigned_commits = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectPushRulesEl {
    type O = BlockAssignable<DataProjectPushRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectPushRulesEl {}

impl BuildDataProjectPushRulesEl {
    pub fn build(self) -> DataProjectPushRulesEl {
        DataProjectPushRulesEl {
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

pub struct DataProjectPushRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectPushRulesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectPushRulesElRef {
        DataProjectPushRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectPushRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `author_email_regex` after provisioning.\n"]
    pub fn author_email_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `branch_name_regex` after provisioning.\n"]
    pub fn branch_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_committer_check` after provisioning.\n"]
    pub fn commit_committer_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_committer_name_check` after provisioning.\n"]
    pub fn commit_committer_name_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_committer_name_check", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_negative_regex` after provisioning.\n"]
    pub fn commit_message_negative_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_negative_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_regex` after provisioning.\n"]
    pub fn commit_message_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `deny_delete_tag` after provisioning.\n"]
    pub fn deny_delete_tag(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deny_delete_tag", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name_regex` after provisioning.\n"]
    pub fn file_name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.base))
    }

    #[doc= "Get a reference to the value of field `member_check` after provisioning.\n"]
    pub fn member_check(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_check", self.base))
    }

    #[doc= "Get a reference to the value of field `prevent_secrets` after provisioning.\n"]
    pub fn prevent_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `reject_unsigned_commits` after provisioning.\n"]
    pub fn reject_unsigned_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reject_unsigned_commits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectSharedWithGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_access_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataProjectSharedWithGroupsEl {
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

impl ToListMappable for DataProjectSharedWithGroupsEl {
    type O = BlockAssignable<DataProjectSharedWithGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectSharedWithGroupsEl {}

impl BuildDataProjectSharedWithGroupsEl {
    pub fn build(self) -> DataProjectSharedWithGroupsEl {
        DataProjectSharedWithGroupsEl {
            group_access_level: core::default::Default::default(),
            group_full_path: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
        }
    }
}

pub struct DataProjectSharedWithGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectSharedWithGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectSharedWithGroupsElRef {
        DataProjectSharedWithGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectSharedWithGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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

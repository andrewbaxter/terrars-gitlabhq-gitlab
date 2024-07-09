use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct RepositoryFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<PrimField<String>>,
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<PrimField<String>>,
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_filemode: Option<PrimField<bool>>,
    file_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_on_create: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RepositoryFileTimeoutsEl>,
}

struct RepositoryFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryFileData>,
}

#[derive(Clone)]
pub struct RepositoryFile(Rc<RepositoryFile_>);

impl RepositoryFile {
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

    #[doc= "Set the field `author_email`.\nEmail of the commit author."]
    pub fn set_author_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().author_email = Some(v.into());
        self
    }

    #[doc= "Set the field `author_name`.\nName of the commit author."]
    pub fn set_author_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().author_name = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message`.\nCommit message."]
    pub fn set_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `create_commit_message`.\nCreate commit message."]
    pub fn set_create_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().create_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_commit_message`.\nDelete Commit message."]
    pub fn set_delete_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delete_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `encoding`.\nThe file content encoding. Default value is `base64`. Valid values are: `base64`, `text`."]
    pub fn set_encoding(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encoding = Some(v.into());
        self
    }

    #[doc= "Set the field `execute_filemode`.\nEnables or disables the execute flag on the file. **Note**: requires GitLab 14.10 or newer."]
    pub fn set_execute_filemode(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().execute_filemode = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `overwrite_on_create`.\nEnable overwriting existing files, defaults to `false`. This attribute is only used during `create` and must be use carefully. We suggest to use `imports` whenever possible and limit the use of this attribute for when the project was imported on the same `apply`. This attribute is not supported during a resource import."]
    pub fn set_overwrite_on_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().overwrite_on_create = Some(v.into());
        self
    }

    #[doc= "Set the field `start_branch`.\nName of the branch to start the new commit from."]
    pub fn set_start_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `update_commit_message`.\nUpdate commit message."]
    pub fn set_update_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().update_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RepositoryFileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `author_email` after provisioning.\nEmail of the commit author."]
    pub fn author_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_name` after provisioning.\nName of the commit author."]
    pub fn author_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blob_id` after provisioning.\nThe blob id."]
    pub fn blob_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nName of the branch to which to commit to."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_id` after provisioning.\nThe commit id."]
    pub fn commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nCommit message."]
    pub fn commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nFile content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha256` after provisioning.\nFile content sha256 digest."]
    pub fn content_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_commit_message` after provisioning.\nCreate commit message."]
    pub fn create_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_commit_message` after provisioning.\nDelete Commit message."]
    pub fn delete_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe file content encoding. Default value is `base64`. Valid values are: `base64`, `text`."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `execute_filemode` after provisioning.\nEnables or disables the execute flag on the file. **Note**: requires GitLab 14.10 or newer."]
    pub fn execute_filemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.execute_filemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe filename."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_path` after provisioning.\nThe full path of the file. It must be relative to the root of the project without a leading slash `/` or `./`."]
    pub fn file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_commit_id` after provisioning.\nThe last known commit id."]
    pub fn last_commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `overwrite_on_create` after provisioning.\nEnable overwriting existing files, defaults to `false`. This attribute is only used during `create` and must be use carefully. We suggest to use `imports` whenever possible and limit the use of this attribute for when the project was imported on the same `apply`. This attribute is not supported during a resource import."]
    pub fn overwrite_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of branch, tag or commit."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nThe file size."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_branch` after provisioning.\nName of the branch to start the new commit from."]
    pub fn start_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_commit_message` after provisioning.\nUpdate commit message."]
    pub fn update_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RepositoryFileTimeoutsElRef {
        RepositoryFileTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for RepositoryFile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryFile { }

impl ToListMappable for RepositoryFile {
    type O = ListRef<RepositoryFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryFile_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_repository_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryFile {
    pub tf_id: String,
    #[doc= "Name of the branch to which to commit to."]
    pub branch: PrimField<String>,
    #[doc= "File content."]
    pub content: PrimField<String>,
    #[doc= "The full path of the file. It must be relative to the root of the project without a leading slash `/` or `./`."]
    pub file_path: PrimField<String>,
    #[doc= "The name or ID of the project."]
    pub project: PrimField<String>,
}

impl BuildRepositoryFile {
    pub fn build(self, stack: &mut Stack) -> RepositoryFile {
        let out = RepositoryFile(Rc::new(RepositoryFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                author_email: core::default::Default::default(),
                author_name: core::default::Default::default(),
                branch: self.branch,
                commit_message: core::default::Default::default(),
                content: self.content,
                create_commit_message: core::default::Default::default(),
                delete_commit_message: core::default::Default::default(),
                encoding: core::default::Default::default(),
                execute_filemode: core::default::Default::default(),
                file_path: self.file_path,
                id: core::default::Default::default(),
                overwrite_on_create: core::default::Default::default(),
                project: self.project,
                start_branch: core::default::Default::default(),
                update_commit_message: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryFileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `author_email` after provisioning.\nEmail of the commit author."]
    pub fn author_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_name` after provisioning.\nName of the commit author."]
    pub fn author_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blob_id` after provisioning.\nThe blob id."]
    pub fn blob_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nName of the branch to which to commit to."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_id` after provisioning.\nThe commit id."]
    pub fn commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nCommit message."]
    pub fn commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nFile content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha256` after provisioning.\nFile content sha256 digest."]
    pub fn content_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_commit_message` after provisioning.\nCreate commit message."]
    pub fn create_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_commit_message` after provisioning.\nDelete Commit message."]
    pub fn delete_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe file content encoding. Default value is `base64`. Valid values are: `base64`, `text`."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `execute_filemode` after provisioning.\nEnables or disables the execute flag on the file. **Note**: requires GitLab 14.10 or newer."]
    pub fn execute_filemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.execute_filemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe filename."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_path` after provisioning.\nThe full path of the file. It must be relative to the root of the project without a leading slash `/` or `./`."]
    pub fn file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_commit_id` after provisioning.\nThe last known commit id."]
    pub fn last_commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `overwrite_on_create` after provisioning.\nEnable overwriting existing files, defaults to `false`. This attribute is only used during `create` and must be use carefully. We suggest to use `imports` whenever possible and limit the use of this attribute for when the project was imported on the same `apply`. This attribute is not supported during a resource import."]
    pub fn overwrite_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of branch, tag or commit."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nThe file size."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_branch` after provisioning.\nName of the branch to start the new commit from."]
    pub fn start_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_commit_message` after provisioning.\nUpdate commit message."]
    pub fn update_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RepositoryFileTimeoutsElRef {
        RepositoryFileTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RepositoryFileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RepositoryFileTimeoutsEl {
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

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryFileTimeoutsEl {
    type O = BlockAssignable<RepositoryFileTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryFileTimeoutsEl {}

impl BuildRepositoryFileTimeoutsEl {
    pub fn build(self) -> RepositoryFileTimeoutsEl {
        RepositoryFileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RepositoryFileTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryFileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RepositoryFileTimeoutsElRef {
        RepositoryFileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryFileTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

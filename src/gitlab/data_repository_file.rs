use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataRepositoryFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    file_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(rename = "ref")]
    ref_: PrimField<String>,
}

struct DataRepositoryFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryFileData>,
}

#[derive(Clone)]
pub struct DataRepositoryFile(Rc<DataRepositoryFile_>);

impl DataRepositoryFile {
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

    #[doc= "Get a reference to the value of field `blob_id` after provisioning.\nThe blob id."]
    pub fn blob_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_id` after provisioning.\nThe commit id."]
    pub fn commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nFile content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha256` after provisioning.\nFile content sha256 digest."]
    pub fn content_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe file content encoding."]
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
}

impl Referable for DataRepositoryFile {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryFile { }

impl ToListMappable for DataRepositoryFile {
    type O = ListRef<DataRepositoryFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryFile_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_repository_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryFile {
    pub tf_id: String,
    #[doc= "The full path of the file. It must be relative to the root of the project without a leading slash `/` or `./`."]
    pub file_path: PrimField<String>,
    #[doc= "The name or ID of the project."]
    pub project: PrimField<String>,
    #[doc= "The name of branch, tag or commit."]
    pub ref_: PrimField<String>,
}

impl BuildDataRepositoryFile {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryFile {
        let out = DataRepositoryFile(Rc::new(DataRepositoryFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                file_path: self.file_path,
                id: core::default::Default::default(),
                project: self.project,
                ref_: self.ref_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryFileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `blob_id` after provisioning.\nThe blob id."]
    pub fn blob_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_id` after provisioning.\nThe commit id."]
    pub fn commit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nFile content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha256` after provisioning.\nFile content sha256 digest."]
    pub fn content_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha256", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe file content encoding."]
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
}

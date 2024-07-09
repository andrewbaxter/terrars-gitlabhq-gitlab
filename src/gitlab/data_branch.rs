use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataBranchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    project: PrimField<String>,
}

struct DataBranch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBranchData>,
}

#[derive(Clone)]
pub struct DataBranch(Rc<DataBranch_>);

impl DataBranch {
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

    #[doc= "Get a reference to the value of field `can_push` after provisioning.\nBool, true if you can push to the branch."]
    pub fn can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit` after provisioning.\nThe commit associated with the branch ref."]
    pub fn commit(&self) -> SetRef<DataBranchCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nBool, true if branch is the default branch for the project."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `developer_can_merge` after provisioning.\nBool, true if developer level access allows to merge branch."]
    pub fn developer_can_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_can_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `developer_can_push` after provisioning.\nBool, true if developer level access allows git push."]
    pub fn developer_can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_can_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merged` after provisioning.\nBool, true if the branch has been merged into it's parent."]
    pub fn merged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the branch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe full path or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nBool, true if branch has branch protection."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe url of the created branch (https.)"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }
}

impl Referable for DataBranch {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBranch { }

impl ToListMappable for DataBranch {
    type O = ListRef<DataBranchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBranch_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_branch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBranch {
    pub tf_id: String,
    #[doc= "The name of the branch."]
    pub name: PrimField<String>,
    #[doc= "The full path or id of the project."]
    pub project: PrimField<String>,
}

impl BuildDataBranch {
    pub fn build(self, stack: &mut Stack) -> DataBranch {
        let out = DataBranch(Rc::new(DataBranch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBranchData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBranchRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBranchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBranchRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `can_push` after provisioning.\nBool, true if you can push to the branch."]
    pub fn can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit` after provisioning.\nThe commit associated with the branch ref."]
    pub fn commit(&self) -> SetRef<DataBranchCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nBool, true if branch is the default branch for the project."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `developer_can_merge` after provisioning.\nBool, true if developer level access allows to merge branch."]
    pub fn developer_can_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_can_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `developer_can_push` after provisioning.\nBool, true if developer level access allows git push."]
    pub fn developer_can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developer_can_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merged` after provisioning.\nBool, true if the branch has been merged into it's parent."]
    pub fn merged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the branch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe full path or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nBool, true if branch has branch protection."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe url of the created branch (https.)"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBranchCommitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    author_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authored_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committed_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

impl DataBranchCommitEl {
    #[doc= "Set the field `author_email`.\n"]
    pub fn set_author_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.author_email = Some(v.into());
        self
    }

    #[doc= "Set the field `author_name`.\n"]
    pub fn set_author_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.author_name = Some(v.into());
        self
    }

    #[doc= "Set the field `authored_date`.\n"]
    pub fn set_authored_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authored_date = Some(v.into());
        self
    }

    #[doc= "Set the field `committed_date`.\n"]
    pub fn set_committed_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.committed_date = Some(v.into());
        self
    }

    #[doc= "Set the field `committer_email`.\n"]
    pub fn set_committer_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.committer_email = Some(v.into());
        self
    }

    #[doc= "Set the field `committer_name`.\n"]
    pub fn set_committer_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.committer_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_ids`.\n"]
    pub fn set_parent_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.parent_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `short_id`.\n"]
    pub fn set_short_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.short_id = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl ToListMappable for DataBranchCommitEl {
    type O = BlockAssignable<DataBranchCommitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBranchCommitEl {}

impl BuildDataBranchCommitEl {
    pub fn build(self) -> DataBranchCommitEl {
        DataBranchCommitEl {
            author_email: core::default::Default::default(),
            author_name: core::default::Default::default(),
            authored_date: core::default::Default::default(),
            committed_date: core::default::Default::default(),
            committer_email: core::default::Default::default(),
            committer_name: core::default::Default::default(),
            id: core::default::Default::default(),
            message: core::default::Default::default(),
            parent_ids: core::default::Default::default(),
            short_id: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct DataBranchCommitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBranchCommitElRef {
    fn new(shared: StackShared, base: String) -> DataBranchCommitElRef {
        DataBranchCommitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBranchCommitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `author_email` after provisioning.\n"]
    pub fn author_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_email", self.base))
    }

    #[doc= "Get a reference to the value of field `author_name` after provisioning.\n"]
    pub fn author_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_name", self.base))
    }

    #[doc= "Get a reference to the value of field `authored_date` after provisioning.\n"]
    pub fn authored_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authored_date", self.base))
    }

    #[doc= "Get a reference to the value of field `committed_date` after provisioning.\n"]
    pub fn committed_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.committed_date", self.base))
    }

    #[doc= "Get a reference to the value of field `committer_email` after provisioning.\n"]
    pub fn committer_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.committer_email", self.base))
    }

    #[doc= "Get a reference to the value of field `committer_name` after provisioning.\n"]
    pub fn committer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.committer_name", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc= "Get a reference to the value of field `parent_ids` after provisioning.\n"]
    pub fn parent_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.parent_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `short_id` after provisioning.\n"]
    pub fn short_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_id", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

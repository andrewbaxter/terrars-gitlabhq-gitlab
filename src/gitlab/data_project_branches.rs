use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectBranchesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct DataProjectBranches_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectBranchesData>,
}

#[derive(Clone)]
pub struct DataProjectBranches(Rc<DataProjectBranches_>);

impl DataProjectBranches {
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

    #[doc= "Get a reference to the value of field `branches` after provisioning.\nThe list of branches of the project, as defined below."]
    pub fn branches(&self) -> ListRef<DataProjectBranchesBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for DataProjectBranches {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectBranches { }

impl ToListMappable for DataProjectBranches {
    type O = ListRef<DataProjectBranchesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectBranches_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_branches".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectBranches {
    pub tf_id: String,
    #[doc= "ID or URL-encoded path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildDataProjectBranches {
    pub fn build(self, stack: &mut Stack) -> DataProjectBranches {
        let out = DataProjectBranches(Rc::new(DataProjectBranches_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectBranchesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectBranchesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectBranchesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectBranchesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `branches` after provisioning.\nThe list of branches of the project, as defined below."]
    pub fn branches(&self) -> ListRef<DataProjectBranchesBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectBranchesBranchesElCommitEl {
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

impl DataProjectBranchesBranchesElCommitEl {
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

impl ToListMappable for DataProjectBranchesBranchesElCommitEl {
    type O = BlockAssignable<DataProjectBranchesBranchesElCommitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectBranchesBranchesElCommitEl {}

impl BuildDataProjectBranchesBranchesElCommitEl {
    pub fn build(self) -> DataProjectBranchesBranchesElCommitEl {
        DataProjectBranchesBranchesElCommitEl {
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

pub struct DataProjectBranchesBranchesElCommitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectBranchesBranchesElCommitElRef {
    fn new(shared: StackShared, base: String) -> DataProjectBranchesBranchesElCommitElRef {
        DataProjectBranchesBranchesElCommitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectBranchesBranchesElCommitElRef {
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

#[derive(Serialize)]
pub struct DataProjectBranchesBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    can_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit: Option<SetField<DataProjectBranchesBranchesElCommitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developers_can_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developers_can_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
}

impl DataProjectBranchesBranchesEl {
    #[doc= "Set the field `can_push`.\n"]
    pub fn set_can_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.can_push = Some(v.into());
        self
    }

    #[doc= "Set the field `commit`.\n"]
    pub fn set_commit(mut self, v: impl Into<SetField<DataProjectBranchesBranchesElCommitEl>>) -> Self {
        self.commit = Some(v.into());
        self
    }

    #[doc= "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default = Some(v.into());
        self
    }

    #[doc= "Set the field `developers_can_merge`.\n"]
    pub fn set_developers_can_merge(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.developers_can_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `developers_can_push`.\n"]
    pub fn set_developers_can_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.developers_can_push = Some(v.into());
        self
    }

    #[doc= "Set the field `merged`.\n"]
    pub fn set_merged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.merged = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `protected`.\n"]
    pub fn set_protected(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protected = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectBranchesBranchesEl {
    type O = BlockAssignable<DataProjectBranchesBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectBranchesBranchesEl {}

impl BuildDataProjectBranchesBranchesEl {
    pub fn build(self) -> DataProjectBranchesBranchesEl {
        DataProjectBranchesBranchesEl {
            can_push: core::default::Default::default(),
            commit: core::default::Default::default(),
            default: core::default::Default::default(),
            developers_can_merge: core::default::Default::default(),
            developers_can_push: core::default::Default::default(),
            merged: core::default::Default::default(),
            name: core::default::Default::default(),
            protected: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct DataProjectBranchesBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectBranchesBranchesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectBranchesBranchesElRef {
        DataProjectBranchesBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectBranchesBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `can_push` after provisioning.\n"]
    pub fn can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_push", self.base))
    }

    #[doc= "Get a reference to the value of field `commit` after provisioning.\n"]
    pub fn commit(&self) -> SetRef<DataProjectBranchesBranchesElCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.base))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc= "Get a reference to the value of field `developers_can_merge` after provisioning.\n"]
    pub fn developers_can_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developers_can_merge", self.base))
    }

    #[doc= "Get a reference to the value of field `developers_can_push` after provisioning.\n"]
    pub fn developers_can_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.developers_can_push", self.base))
    }

    #[doc= "Get a reference to the value of field `merged` after provisioning.\n"]
    pub fn merged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merged", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\n"]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}

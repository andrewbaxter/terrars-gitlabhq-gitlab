use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectTagData {
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

struct DataProjectTag_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectTagData>,
}

#[derive(Clone)]
pub struct DataProjectTag(Rc<DataProjectTag_>);

impl DataProjectTag {
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

    #[doc= "Get a reference to the value of field `commit` after provisioning.\nThe commit associated with the tag."]
    pub fn commit(&self) -> SetRef<DataProjectTagCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\nThe message of the annotated tag."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of a tag."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nBool, true if tag has tag protection."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release` after provisioning.\nThe release associated with the tag."]
    pub fn release(&self) -> SetRef<DataProjectTagReleaseElRef> {
        SetRef::new(self.shared().clone(), format!("{}.release", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nThe unique id assigned to the commit by Gitlab."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

impl Referable for DataProjectTag {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectTag { }

impl ToListMappable for DataProjectTag {
    type O = ListRef<DataProjectTagRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectTag_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_tag".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectTag {
    pub tf_id: String,
    #[doc= "The name of a tag."]
    pub name: PrimField<String>,
    #[doc= "The ID or URL-encoded path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildDataProjectTag {
    pub fn build(self, stack: &mut Stack) -> DataProjectTag {
        let out = DataProjectTag(Rc::new(DataProjectTag_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectTagData {
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

pub struct DataProjectTagRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectTagRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `commit` after provisioning.\nThe commit associated with the tag."]
    pub fn commit(&self) -> SetRef<DataProjectTagCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\nThe message of the annotated tag."]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of a tag."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nBool, true if tag has tag protection."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release` after provisioning.\nThe release associated with the tag."]
    pub fn release(&self) -> SetRef<DataProjectTagReleaseElRef> {
        SetRef::new(self.shared().clone(), format!("{}.release", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nThe unique id assigned to the commit by Gitlab."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectTagCommitEl {
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

impl DataProjectTagCommitEl {
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

impl ToListMappable for DataProjectTagCommitEl {
    type O = BlockAssignable<DataProjectTagCommitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectTagCommitEl {}

impl BuildDataProjectTagCommitEl {
    pub fn build(self) -> DataProjectTagCommitEl {
        DataProjectTagCommitEl {
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

pub struct DataProjectTagCommitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagCommitElRef {
    fn new(shared: StackShared, base: String) -> DataProjectTagCommitElRef {
        DataProjectTagCommitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectTagCommitElRef {
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
pub struct DataProjectTagReleaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}

impl DataProjectTagReleaseEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectTagReleaseEl {
    type O = BlockAssignable<DataProjectTagReleaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectTagReleaseEl {}

impl BuildDataProjectTagReleaseEl {
    pub fn build(self) -> DataProjectTagReleaseEl {
        DataProjectTagReleaseEl {
            description: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

pub struct DataProjectTagReleaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagReleaseElRef {
    fn new(shared: StackShared, base: String) -> DataProjectTagReleaseElRef {
        DataProjectTagReleaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectTagReleaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

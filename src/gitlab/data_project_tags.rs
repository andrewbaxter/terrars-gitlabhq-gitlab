use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectTagsData {
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
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
}

struct DataProjectTags_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectTagsData>,
}

#[derive(Clone)]
pub struct DataProjectTags(Rc<DataProjectTags_>);

impl DataProjectTags {
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

    #[doc= "Set the field `order_by`.\nReturn tags ordered by `name` or `updated` fields. Default is `updated`."]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nReturn list of tags matching the search criteria. You can use `^term` and `term$` to find tags that begin and end with `term` respectively. No other regular expressions are supported."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nReturn tags sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn tags ordered by `name` or `updated` fields. Default is `updated`."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn list of tags matching the search criteria. You can use `^term` and `term$` to find tags that begin and end with `term` respectively. No other regular expressions are supported."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn tags sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nList of repository tags from a project."]
    pub fn tags(&self) -> ListRef<DataProjectTagsTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataProjectTags {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectTags { }

impl ToListMappable for DataProjectTags {
    type O = ListRef<DataProjectTagsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectTags_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_tags".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectTags {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildDataProjectTags {
    pub fn build(self, stack: &mut Stack) -> DataProjectTags {
        let out = DataProjectTags(Rc::new(DataProjectTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectTagsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                order_by: core::default::Default::default(),
                project: self.project,
                search: core::default::Default::default(),
                sort: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectTagsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectTagsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn tags ordered by `name` or `updated` fields. Default is `updated`."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn list of tags matching the search criteria. You can use `^term` and `term$` to find tags that begin and end with `term` respectively. No other regular expressions are supported."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn tags sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nList of repository tags from a project."]
    pub fn tags(&self) -> ListRef<DataProjectTagsTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectTagsTagsElCommitEl {
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

impl DataProjectTagsTagsElCommitEl {
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

impl ToListMappable for DataProjectTagsTagsElCommitEl {
    type O = BlockAssignable<DataProjectTagsTagsElCommitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectTagsTagsElCommitEl {}

impl BuildDataProjectTagsTagsElCommitEl {
    pub fn build(self) -> DataProjectTagsTagsElCommitEl {
        DataProjectTagsTagsElCommitEl {
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

pub struct DataProjectTagsTagsElCommitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagsTagsElCommitElRef {
    fn new(shared: StackShared, base: String) -> DataProjectTagsTagsElCommitElRef {
        DataProjectTagsTagsElCommitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectTagsTagsElCommitElRef {
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
pub struct DataProjectTagsTagsElReleaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}

impl DataProjectTagsTagsElReleaseEl {
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

impl ToListMappable for DataProjectTagsTagsElReleaseEl {
    type O = BlockAssignable<DataProjectTagsTagsElReleaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectTagsTagsElReleaseEl {}

impl BuildDataProjectTagsTagsElReleaseEl {
    pub fn build(self) -> DataProjectTagsTagsElReleaseEl {
        DataProjectTagsTagsElReleaseEl {
            description: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

pub struct DataProjectTagsTagsElReleaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagsTagsElReleaseElRef {
    fn new(shared: StackShared, base: String) -> DataProjectTagsTagsElReleaseElRef {
        DataProjectTagsTagsElReleaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectTagsTagsElReleaseElRef {
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

#[derive(Serialize)]
pub struct DataProjectTagsTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit: Option<SetField<DataProjectTagsTagsElCommitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release: Option<SetField<DataProjectTagsTagsElReleaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
}

impl DataProjectTagsTagsEl {
    #[doc= "Set the field `commit`.\n"]
    pub fn set_commit(mut self, v: impl Into<SetField<DataProjectTagsTagsElCommitEl>>) -> Self {
        self.commit = Some(v.into());
        self
    }

    #[doc= "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
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

    #[doc= "Set the field `release`.\n"]
    pub fn set_release(mut self, v: impl Into<SetField<DataProjectTagsTagsElReleaseEl>>) -> Self {
        self.release = Some(v.into());
        self
    }

    #[doc= "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectTagsTagsEl {
    type O = BlockAssignable<DataProjectTagsTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectTagsTagsEl {}

impl BuildDataProjectTagsTagsEl {
    pub fn build(self) -> DataProjectTagsTagsEl {
        DataProjectTagsTagsEl {
            commit: core::default::Default::default(),
            message: core::default::Default::default(),
            name: core::default::Default::default(),
            protected: core::default::Default::default(),
            release: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

pub struct DataProjectTagsTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectTagsTagsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectTagsTagsElRef {
        DataProjectTagsTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectTagsTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `commit` after provisioning.\n"]
    pub fn commit(&self) -> SetRef<DataProjectTagsTagsElCommitElRef> {
        SetRef::new(self.shared().clone(), format!("{}.commit", self.base))
    }

    #[doc= "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\n"]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.base))
    }

    #[doc= "Get a reference to the value of field `release` after provisioning.\n"]
    pub fn release(&self) -> SetRef<DataProjectTagsTagsElReleaseElRef> {
        SetRef::new(self.shared().clone(), format!("{}.release", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

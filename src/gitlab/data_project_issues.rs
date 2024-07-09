use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectIssuesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iids: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    my_reaction_emoji: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_assignee_id: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_author_id: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_labels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_milestone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_my_reaction_emoji: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_labels_details: Option<PrimField<bool>>,
}

struct DataProjectIssues_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectIssuesData>,
}

#[derive(Clone)]
pub struct DataProjectIssues(Rc<DataProjectIssues_>);

impl DataProjectIssues {
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

    #[doc= "Set the field `assignee_id`.\nReturn issues assigned to the given user id. Mutually exclusive with assignee_username. None returns unassigned issues. Any returns issues with an assignee."]
    pub fn set_assignee_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().assignee_id = Some(v.into());
        self
    }

    #[doc= "Set the field `assignee_username`.\nReturn issues assigned to the given username. Similar to assignee_id and mutually exclusive with assignee_id. In GitLab CE, the assignee_username array should only contain a single value. Otherwise, an invalid parameter error is returned."]
    pub fn set_assignee_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().assignee_username = Some(v.into());
        self
    }

    #[doc= "Set the field `author_id`.\nReturn issues created by the given user id. Combine with scope=all or scope=assigned_to_me."]
    pub fn set_author_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().author_id = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential`.\nFilter confidential or public issues."]
    pub fn set_confidential(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential = Some(v.into());
        self
    }

    #[doc= "Set the field `created_after`.\nReturn issues created on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn set_created_after(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_after = Some(v.into());
        self
    }

    #[doc= "Set the field `created_before`.\nReturn issues created on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn set_created_before(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_before = Some(v.into());
        self
    }

    #[doc= "Set the field `due_date`.\nReturn issues that have no due date, are overdue, or whose due date is this week, this month, or between two weeks ago and next month. Accepts: 0 (no due date), any, today, tomorrow, overdue, week, month, next_month_and_previous_two_weeks."]
    pub fn set_due_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().due_date = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `iids`.\nReturn only the issues having the given iid"]
    pub fn set_iids(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().iids = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_type`.\nFilter to a given type of issue. Valid values are [issue incident test_case]. (Introduced in GitLab 13.12)"]
    pub fn set_issue_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().issue_type = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nReturn issues with labels. Issues must have all labels to be returned. None lists all issues with no labels. Any lists all issues with at least one label. No+Label (Deprecated) lists all issues with no labels. Predefined names are case-insensitive."]
    pub fn set_labels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone`.\nThe milestone title. None lists all issues with no milestone. Any lists all issues that have an assigned milestone."]
    pub fn set_milestone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().milestone = Some(v.into());
        self
    }

    #[doc= "Set the field `my_reaction_emoji`.\nReturn issues reacted by the authenticated user by the given emoji. None returns issues not given a reaction. Any returns issues given at least one reaction."]
    pub fn set_my_reaction_emoji(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().my_reaction_emoji = Some(v.into());
        self
    }

    #[doc= "Set the field `not_assignee_id`.\nReturn issues that do not match the assignee id."]
    pub fn set_not_assignee_id(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().not_assignee_id = Some(v.into());
        self
    }

    #[doc= "Set the field `not_author_id`.\nReturn issues that do not match the author id."]
    pub fn set_not_author_id(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().not_author_id = Some(v.into());
        self
    }

    #[doc= "Set the field `not_labels`.\nReturn issues that do not match the labels."]
    pub fn set_not_labels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().not_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `not_milestone`.\nReturn issues that do not match the milestone."]
    pub fn set_not_milestone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().not_milestone = Some(v.into());
        self
    }

    #[doc= "Set the field `not_my_reaction_emoji`.\nReturn issues not reacted by the authenticated user by the given emoji."]
    pub fn set_not_my_reaction_emoji(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().not_my_reaction_emoji = Some(v.into());
        self
    }

    #[doc= "Set the field `order_by`.\nReturn issues ordered by. Valid values are `created_at`, `updated_at`, `priority`, `due_date`, `relative_position`, `label_priority`, `milestone_due`, `popularity`, `weight`. Default is created_at"]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nReturn issues for the given scope. Valid values are `created_by_me`, `assigned_to_me`, `all`. Defaults to all."]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nSearch project issues against their title and description"]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nReturn issues sorted in asc or desc order. Default is desc"]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_after`.\nReturn issues updated on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn set_updated_after(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().updated_after = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_before`.\nReturn issues updated on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn set_updated_before(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().updated_before = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nReturn issues with the specified weight. None returns issues with no weight assigned. Any returns issues with a weight assigned."]
    pub fn set_weight(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().weight = Some(v.into());
        self
    }

    #[doc= "Set the field `with_labels_details`.\nIf true, the response returns more details for each label in labels field: :name, :color, :description, :description_html, :text_color. Default is false. description_html was introduced in GitLab 12.7"]
    pub fn set_with_labels_details(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_labels_details = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `assignee_id` after provisioning.\nReturn issues assigned to the given user id. Mutually exclusive with assignee_username. None returns unassigned issues. Any returns issues with an assignee."]
    pub fn assignee_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assignee_username` after provisioning.\nReturn issues assigned to the given username. Similar to assignee_id and mutually exclusive with assignee_id. In GitLab CE, the assignee_username array should only contain a single value. Otherwise, an invalid parameter error is returned."]
    pub fn assignee_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_id` after provisioning.\nReturn issues created by the given user id. Combine with scope=all or scope=assigned_to_me."]
    pub fn author_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nFilter confidential or public issues."]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_after` after provisioning.\nReturn issues created on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_before` after provisioning.\nReturn issues created on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nReturn issues that have no due date, are overdue, or whose due date is this week, this month, or between two weeks ago and next month. Accepts: 0 (no due date), any, today, tomorrow, overdue, week, month, next_month_and_previous_two_weeks."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iids` after provisioning.\nReturn only the issues having the given iid"]
    pub fn iids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.iids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_type` after provisioning.\nFilter to a given type of issue. Valid values are [issue incident test_case]. (Introduced in GitLab 13.12)"]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues` after provisioning.\nThe list of issues returned by the search."]
    pub fn issues(&self) -> ListRef<DataProjectIssuesIssuesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nReturn issues with labels. Issues must have all labels to be returned. None lists all issues with no labels. Any lists all issues with at least one label. No+Label (Deprecated) lists all issues with no labels. Predefined names are case-insensitive."]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone` after provisioning.\nThe milestone title. None lists all issues with no milestone. Any lists all issues that have an assigned milestone."]
    pub fn milestone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `my_reaction_emoji` after provisioning.\nReturn issues reacted by the authenticated user by the given emoji. None returns issues not given a reaction. Any returns issues given at least one reaction."]
    pub fn my_reaction_emoji(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.my_reaction_emoji", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_assignee_id` after provisioning.\nReturn issues that do not match the assignee id."]
    pub fn not_assignee_id(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.not_assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_author_id` after provisioning.\nReturn issues that do not match the author id."]
    pub fn not_author_id(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.not_author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_labels` after provisioning.\nReturn issues that do not match the labels."]
    pub fn not_labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_milestone` after provisioning.\nReturn issues that do not match the milestone."]
    pub fn not_milestone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_milestone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_my_reaction_emoji` after provisioning.\nReturn issues not reacted by the authenticated user by the given emoji."]
    pub fn not_my_reaction_emoji(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_my_reaction_emoji", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn issues ordered by. Valid values are `created_at`, `updated_at`, `priority`, `due_date`, `relative_position`, `label_priority`, `milestone_due`, `popularity`, `weight`. Default is created_at"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nReturn issues for the given scope. Valid values are `created_by_me`, `assigned_to_me`, `all`. Defaults to all."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch project issues against their title and description"]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn issues sorted in asc or desc order. Default is desc"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_after` after provisioning.\nReturn issues updated on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn updated_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_before` after provisioning.\nReturn issues updated on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn updated_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nReturn issues with the specified weight. None returns issues with no weight assigned. Any returns issues with a weight assigned."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_labels_details` after provisioning.\nIf true, the response returns more details for each label in labels field: :name, :color, :description, :description_html, :text_color. Default is false. description_html was introduced in GitLab 12.7"]
    pub fn with_labels_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_labels_details", self.extract_ref()))
    }
}

impl Referable for DataProjectIssues {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectIssues { }

impl ToListMappable for DataProjectIssues {
    type O = ListRef<DataProjectIssuesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectIssues_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_issues".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectIssues {
    pub tf_id: String,
    #[doc= "The name or id of the project."]
    pub project: PrimField<String>,
}

impl BuildDataProjectIssues {
    pub fn build(self, stack: &mut Stack) -> DataProjectIssues {
        let out = DataProjectIssues(Rc::new(DataProjectIssues_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectIssuesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                assignee_id: core::default::Default::default(),
                assignee_username: core::default::Default::default(),
                author_id: core::default::Default::default(),
                confidential: core::default::Default::default(),
                created_after: core::default::Default::default(),
                created_before: core::default::Default::default(),
                due_date: core::default::Default::default(),
                id: core::default::Default::default(),
                iids: core::default::Default::default(),
                issue_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                milestone: core::default::Default::default(),
                my_reaction_emoji: core::default::Default::default(),
                not_assignee_id: core::default::Default::default(),
                not_author_id: core::default::Default::default(),
                not_labels: core::default::Default::default(),
                not_milestone: core::default::Default::default(),
                not_my_reaction_emoji: core::default::Default::default(),
                order_by: core::default::Default::default(),
                project: self.project,
                scope: core::default::Default::default(),
                search: core::default::Default::default(),
                sort: core::default::Default::default(),
                updated_after: core::default::Default::default(),
                updated_before: core::default::Default::default(),
                weight: core::default::Default::default(),
                with_labels_details: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectIssuesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectIssuesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectIssuesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `assignee_id` after provisioning.\nReturn issues assigned to the given user id. Mutually exclusive with assignee_username. None returns unassigned issues. Any returns issues with an assignee."]
    pub fn assignee_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assignee_username` after provisioning.\nReturn issues assigned to the given username. Similar to assignee_id and mutually exclusive with assignee_id. In GitLab CE, the assignee_username array should only contain a single value. Otherwise, an invalid parameter error is returned."]
    pub fn assignee_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_id` after provisioning.\nReturn issues created by the given user id. Combine with scope=all or scope=assigned_to_me."]
    pub fn author_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nFilter confidential or public issues."]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_after` after provisioning.\nReturn issues created on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_before` after provisioning.\nReturn issues created on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nReturn issues that have no due date, are overdue, or whose due date is this week, this month, or between two weeks ago and next month. Accepts: 0 (no due date), any, today, tomorrow, overdue, week, month, next_month_and_previous_two_weeks."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iids` after provisioning.\nReturn only the issues having the given iid"]
    pub fn iids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.iids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_type` after provisioning.\nFilter to a given type of issue. Valid values are [issue incident test_case]. (Introduced in GitLab 13.12)"]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues` after provisioning.\nThe list of issues returned by the search."]
    pub fn issues(&self) -> ListRef<DataProjectIssuesIssuesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nReturn issues with labels. Issues must have all labels to be returned. None lists all issues with no labels. Any lists all issues with at least one label. No+Label (Deprecated) lists all issues with no labels. Predefined names are case-insensitive."]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone` after provisioning.\nThe milestone title. None lists all issues with no milestone. Any lists all issues that have an assigned milestone."]
    pub fn milestone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `my_reaction_emoji` after provisioning.\nReturn issues reacted by the authenticated user by the given emoji. None returns issues not given a reaction. Any returns issues given at least one reaction."]
    pub fn my_reaction_emoji(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.my_reaction_emoji", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_assignee_id` after provisioning.\nReturn issues that do not match the assignee id."]
    pub fn not_assignee_id(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.not_assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_author_id` after provisioning.\nReturn issues that do not match the author id."]
    pub fn not_author_id(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.not_author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_labels` after provisioning.\nReturn issues that do not match the labels."]
    pub fn not_labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_milestone` after provisioning.\nReturn issues that do not match the milestone."]
    pub fn not_milestone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_milestone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_my_reaction_emoji` after provisioning.\nReturn issues not reacted by the authenticated user by the given emoji."]
    pub fn not_my_reaction_emoji(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_my_reaction_emoji", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn issues ordered by. Valid values are `created_at`, `updated_at`, `priority`, `due_date`, `relative_position`, `label_priority`, `milestone_due`, `popularity`, `weight`. Default is created_at"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nReturn issues for the given scope. Valid values are `created_by_me`, `assigned_to_me`, `all`. Defaults to all."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch project issues against their title and description"]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn issues sorted in asc or desc order. Default is desc"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_after` after provisioning.\nReturn issues updated on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn updated_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_before` after provisioning.\nReturn issues updated on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"]
    pub fn updated_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nReturn issues with the specified weight. None returns issues with no weight assigned. Any returns issues with a weight assigned."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_labels_details` after provisioning.\nIf true, the response returns more details for each label in labels field: :name, :color, :description, :description_html, :text_color. Default is false. description_html was introduced in GitLab 12.7"]
    pub fn with_labels_details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_labels_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectIssuesIssuesElTaskCompletionStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}

impl DataProjectIssuesIssuesElTaskCompletionStatusEl {
    #[doc= "Set the field `completed_count`.\n"]
    pub fn set_completed_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.completed_count = Some(v.into());
        self
    }

    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectIssuesIssuesElTaskCompletionStatusEl {
    type O = BlockAssignable<DataProjectIssuesIssuesElTaskCompletionStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectIssuesIssuesElTaskCompletionStatusEl {}

impl BuildDataProjectIssuesIssuesElTaskCompletionStatusEl {
    pub fn build(self) -> DataProjectIssuesIssuesElTaskCompletionStatusEl {
        DataProjectIssuesIssuesElTaskCompletionStatusEl {
            completed_count: core::default::Default::default(),
            count: core::default::Default::default(),
        }
    }
}

pub struct DataProjectIssuesIssuesElTaskCompletionStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectIssuesIssuesElTaskCompletionStatusElRef {
    fn new(shared: StackShared, base: String) -> DataProjectIssuesIssuesElTaskCompletionStatusElRef {
        DataProjectIssuesIssuesElTaskCompletionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectIssuesIssuesElTaskCompletionStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `completed_count` after provisioning.\n"]
    pub fn completed_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.completed_count", self.base))
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectIssuesIssuesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closed_by_user_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_to_resolve: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downvotes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epic_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epic_issue_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_time_estimate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    human_total_time_spent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_link_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_request_to_resolve_discussions_of: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    moved_to_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_completion_status: Option<ListField<DataProjectIssuesIssuesElTaskCompletionStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_estimate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_time_spent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upvotes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_notes_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataProjectIssuesIssuesEl {
    #[doc= "Set the field `assignee_ids`.\n"]
    pub fn set_assignee_ids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.assignee_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `author_id`.\n"]
    pub fn set_author_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.author_id = Some(v.into());
        self
    }

    #[doc= "Set the field `closed_at`.\n"]
    pub fn set_closed_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.closed_at = Some(v.into());
        self
    }

    #[doc= "Set the field `closed_by_user_id`.\n"]
    pub fn set_closed_by_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.closed_by_user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential`.\n"]
    pub fn set_confidential(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.confidential = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `discussion_locked`.\n"]
    pub fn set_discussion_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.discussion_locked = Some(v.into());
        self
    }

    #[doc= "Set the field `discussion_to_resolve`.\n"]
    pub fn set_discussion_to_resolve(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discussion_to_resolve = Some(v.into());
        self
    }

    #[doc= "Set the field `downvotes`.\n"]
    pub fn set_downvotes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.downvotes = Some(v.into());
        self
    }

    #[doc= "Set the field `due_date`.\n"]
    pub fn set_due_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.due_date = Some(v.into());
        self
    }

    #[doc= "Set the field `epic_id`.\n"]
    pub fn set_epic_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.epic_id = Some(v.into());
        self
    }

    #[doc= "Set the field `epic_issue_id`.\n"]
    pub fn set_epic_issue_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.epic_issue_id = Some(v.into());
        self
    }

    #[doc= "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_id = Some(v.into());
        self
    }

    #[doc= "Set the field `human_time_estimate`.\n"]
    pub fn set_human_time_estimate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.human_time_estimate = Some(v.into());
        self
    }

    #[doc= "Set the field `human_total_time_spent`.\n"]
    pub fn set_human_total_time_spent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.human_total_time_spent = Some(v.into());
        self
    }

    #[doc= "Set the field `iid`.\n"]
    pub fn set_iid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iid = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_id`.\n"]
    pub fn set_issue_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.issue_id = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_link_id`.\n"]
    pub fn set_issue_link_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.issue_link_id = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_type`.\n"]
    pub fn set_issue_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issue_type = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `links`.\n"]
    pub fn set_links(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.links = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_request_to_resolve_discussions_of`.\n"]
    pub fn set_merge_request_to_resolve_discussions_of(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.merge_request_to_resolve_discussions_of = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_count`.\n"]
    pub fn set_merge_requests_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.merge_requests_count = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_id`.\n"]
    pub fn set_milestone_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.milestone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `moved_to_id`.\n"]
    pub fn set_moved_to_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.moved_to_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `references`.\n"]
    pub fn set_references(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.references = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `subscribed`.\n"]
    pub fn set_subscribed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.subscribed = Some(v.into());
        self
    }

    #[doc= "Set the field `task_completion_status`.\n"]
    pub fn set_task_completion_status(
        mut self,
        v: impl Into<ListField<DataProjectIssuesIssuesElTaskCompletionStatusEl>>,
    ) -> Self {
        self.task_completion_status = Some(v.into());
        self
    }

    #[doc= "Set the field `time_estimate`.\n"]
    pub fn set_time_estimate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.time_estimate = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `total_time_spent`.\n"]
    pub fn set_total_time_spent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_time_spent = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }

    #[doc= "Set the field `upvotes`.\n"]
    pub fn set_upvotes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.upvotes = Some(v.into());
        self
    }

    #[doc= "Set the field `user_notes_count`.\n"]
    pub fn set_user_notes_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_notes_count = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectIssuesIssuesEl {
    type O = BlockAssignable<DataProjectIssuesIssuesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectIssuesIssuesEl {}

impl BuildDataProjectIssuesIssuesEl {
    pub fn build(self) -> DataProjectIssuesIssuesEl {
        DataProjectIssuesIssuesEl {
            assignee_ids: core::default::Default::default(),
            author_id: core::default::Default::default(),
            closed_at: core::default::Default::default(),
            closed_by_user_id: core::default::Default::default(),
            confidential: core::default::Default::default(),
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            discussion_locked: core::default::Default::default(),
            discussion_to_resolve: core::default::Default::default(),
            downvotes: core::default::Default::default(),
            due_date: core::default::Default::default(),
            epic_id: core::default::Default::default(),
            epic_issue_id: core::default::Default::default(),
            external_id: core::default::Default::default(),
            human_time_estimate: core::default::Default::default(),
            human_total_time_spent: core::default::Default::default(),
            iid: core::default::Default::default(),
            issue_id: core::default::Default::default(),
            issue_link_id: core::default::Default::default(),
            issue_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            links: core::default::Default::default(),
            merge_request_to_resolve_discussions_of: core::default::Default::default(),
            merge_requests_count: core::default::Default::default(),
            milestone_id: core::default::Default::default(),
            moved_to_id: core::default::Default::default(),
            project: core::default::Default::default(),
            references: core::default::Default::default(),
            state: core::default::Default::default(),
            subscribed: core::default::Default::default(),
            task_completion_status: core::default::Default::default(),
            time_estimate: core::default::Default::default(),
            title: core::default::Default::default(),
            total_time_spent: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            upvotes: core::default::Default::default(),
            user_notes_count: core::default::Default::default(),
            web_url: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataProjectIssuesIssuesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectIssuesIssuesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectIssuesIssuesElRef {
        DataProjectIssuesIssuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectIssuesIssuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `assignee_ids` after provisioning.\n"]
    pub fn assignee_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.assignee_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `author_id` after provisioning.\n"]
    pub fn author_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.base))
    }

    #[doc= "Get a reference to the value of field `closed_at` after provisioning.\n"]
    pub fn closed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_at", self.base))
    }

    #[doc= "Get a reference to the value of field `closed_by_user_id` after provisioning.\n"]
    pub fn closed_by_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_by_user_id", self.base))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\n"]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `discussion_locked` after provisioning.\n"]
    pub fn discussion_locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_locked", self.base))
    }

    #[doc= "Get a reference to the value of field `discussion_to_resolve` after provisioning.\n"]
    pub fn discussion_to_resolve(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_to_resolve", self.base))
    }

    #[doc= "Get a reference to the value of field `downvotes` after provisioning.\n"]
    pub fn downvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downvotes", self.base))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\n"]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.base))
    }

    #[doc= "Get a reference to the value of field `epic_id` after provisioning.\n"]
    pub fn epic_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_id", self.base))
    }

    #[doc= "Get a reference to the value of field `epic_issue_id` after provisioning.\n"]
    pub fn epic_issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_issue_id", self.base))
    }

    #[doc= "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc= "Get a reference to the value of field `human_time_estimate` after provisioning.\n"]
    pub fn human_time_estimate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_time_estimate", self.base))
    }

    #[doc= "Get a reference to the value of field `human_total_time_spent` after provisioning.\n"]
    pub fn human_total_time_spent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_total_time_spent", self.base))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\n"]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.base))
    }

    #[doc= "Get a reference to the value of field `issue_id` after provisioning.\n"]
    pub fn issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_id", self.base))
    }

    #[doc= "Get a reference to the value of field `issue_link_id` after provisioning.\n"]
    pub fn issue_link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_link_id", self.base))
    }

    #[doc= "Get a reference to the value of field `issue_type` after provisioning.\n"]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.links", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_request_to_resolve_discussions_of` after provisioning.\n"]
    pub fn merge_request_to_resolve_discussions_of(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_request_to_resolve_discussions_of", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_requests_count` after provisioning.\n"]
    pub fn merge_requests_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_count", self.base))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\n"]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.base))
    }

    #[doc= "Get a reference to the value of field `moved_to_id` after provisioning.\n"]
    pub fn moved_to_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.moved_to_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `references` after provisioning.\n"]
    pub fn references(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.references", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `subscribed` after provisioning.\n"]
    pub fn subscribed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscribed", self.base))
    }

    #[doc= "Get a reference to the value of field `task_completion_status` after provisioning.\n"]
    pub fn task_completion_status(&self) -> ListRef<DataProjectIssuesIssuesElTaskCompletionStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_completion_status", self.base))
    }

    #[doc= "Get a reference to the value of field `time_estimate` after provisioning.\n"]
    pub fn time_estimate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_estimate", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `total_time_spent` after provisioning.\n"]
    pub fn total_time_spent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_time_spent", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc= "Get a reference to the value of field `upvotes` after provisioning.\n"]
    pub fn upvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upvotes", self.base))
    }

    #[doc= "Get a reference to the value of field `user_notes_count` after provisioning.\n"]
    pub fn user_notes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_notes_count", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename = "job")]
pub struct Job {
    #[serde(rename = "recipeSet", default)]
    pub recipe_sets: Vec<RecipeSet>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct RecipeSet {
    #[serde(rename = "recipe", default)]
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Recipe {
    #[serde(rename = "@id", default)]
    pub id: Option<String>,
    #[serde(rename = "@whiteboard", default)]
    pub whiteboard: Option<String>,
    #[serde(rename = "@role", default)]
    pub role: Option<String>,
    #[serde(rename = "@owner", default)]
    pub owner: Option<String>,
    #[serde(rename = "@family", default)]
    pub family: Option<String>,
    #[serde(rename = "@job_id", default)]
    pub job_id: Option<String>,
    #[serde(rename = "params", default)]
    pub params: Option<Params>,
    #[serde(rename = "task", default)]
    pub tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Task {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@role", default)]
    pub role: Option<String>,
    #[serde(rename = "@keepchanges", default)]
    pub keepchanges: Option<String>,
    #[serde(rename = "fetch", default)]
    pub fetch: Option<Fetch>,
    #[serde(rename = "rpm", default)]
    pub rpm: Option<Rpm>,
    #[serde(rename = "params", default)]
    pub params: Option<Params>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Params {
    #[serde(rename = "param", default)]
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Fetch {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "@ssl_verify", default)]
    pub ssl_verify: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Rpm {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@path")]
    pub path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEMPLATE_WITH_CHROME: &str = r#"<job>
    <recipeSet>
        <recipe whiteboard="example" owner="me@example.com" family="RHEL9">
            <autopick random="false"/>
            <watchdog panic="None"/>
            <packages>
                <package name="vim"/>
            </packages>
            <repos>
                <repo name="extras" url="http://example.com/repo"/>
            </repos>
            <params>
                <param name="GLOBAL" value="foo"/>
            </params>
            <distroRequires>
                <distro_family op="=" value="RHEL9"/>
            </distroRequires>
            <task name="/example/git-task" role="SERVERS">
                <fetch url="git://example.com/repo#path/to/task"/>
                <params>
                    <param name="TASK_PARAM" value="bar"/>
                </params>
            </task>
            <task name="/example/rpm-task">
                <rpm name="task-rpm" path="/mnt/tests/task-rpm"/>
            </task>
        </recipe>
    </recipeSet>
</job>"#;

    #[test]
    fn parses_typed_fields_and_ignores_chrome() {
        let job: Job = quick_xml::de::from_str(TEMPLATE_WITH_CHROME).expect("parse");
        assert_eq!(job.recipe_sets.len(), 1);
        let recipe = &job.recipe_sets[0].recipes[0];

        assert_eq!(recipe.id, None);
        assert_eq!(recipe.whiteboard.as_deref(), Some("example"));
        assert_eq!(recipe.owner.as_deref(), Some("me@example.com"));
        assert_eq!(recipe.family.as_deref(), Some("RHEL9"));

        let rparams = recipe.params.as_ref().expect("recipe params");
        assert_eq!(rparams.params.len(), 1);
        assert_eq!(rparams.params[0].name, "GLOBAL");
        assert_eq!(rparams.params[0].value, "foo");

        assert_eq!(recipe.tasks.len(), 2);

        let git_task = &recipe.tasks[0];
        assert_eq!(git_task.name, "/example/git-task");
        assert_eq!(git_task.role.as_deref(), Some("SERVERS"));
        let fetch = git_task.fetch.as_ref().expect("fetch");
        assert_eq!(fetch.url, "git://example.com/repo#path/to/task");
        assert!(git_task.rpm.is_none());
        let tparams = git_task.params.as_ref().expect("task params");
        assert_eq!(tparams.params[0].name, "TASK_PARAM");

        let rpm_task = &recipe.tasks[1];
        assert_eq!(rpm_task.name, "/example/rpm-task");
        let rpm = rpm_task.rpm.as_ref().expect("rpm");
        assert_eq!(rpm.name, "task-rpm");
        assert_eq!(rpm.path, "/mnt/tests/task-rpm");
        assert!(rpm_task.fetch.is_none());
    }
}

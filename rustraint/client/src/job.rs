use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "job")]
pub struct Job {
    #[serde(rename = "recipeSet", default)]
    pub recipe_sets: Vec<RecipeSet>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RecipeSet {
    #[serde(rename = "recipe", default)]
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(
        rename = "@whiteboard",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub whiteboard: Option<String>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none", default)]
    pub role: Option<String>,
    #[serde(rename = "@owner", skip_serializing_if = "Option::is_none", default)]
    pub owner: Option<String>,
    #[serde(rename = "@family", skip_serializing_if = "Option::is_none", default)]
    pub family: Option<String>,
    #[serde(rename = "@job_id", skip_serializing_if = "Option::is_none", default)]
    pub job_id: Option<String>,
    #[serde(rename = "@status")]
    pub status: String,
    #[serde(rename = "@result")]
    pub result: String,
    #[serde(
        rename = "@checkpoint_file",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub checkpoint_file: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<Params>,
    #[serde(rename = "task", default)]
    pub tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none", default)]
    pub role: Option<String>,
    #[serde(
        rename = "@keepchanges",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub keepchanges: Option<String>,
    #[serde(rename = "@status")]
    pub status: String,
    #[serde(rename = "@result")]
    pub result: String,
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none", default)]
    pub version: Option<String>,
    #[serde(
        rename = "@start_time",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub start_time: Option<String>,
    #[serde(rename = "@end_time", skip_serializing_if = "Option::is_none", default)]
    pub end_time: Option<String>,
    #[serde(rename = "@duration", skip_serializing_if = "Option::is_none", default)]
    pub duration: Option<String>,
    #[serde(rename = "fetch", skip_serializing_if = "Option::is_none", default)]
    pub fetch: Option<Fetch>,
    #[serde(rename = "rpm", skip_serializing_if = "Option::is_none", default)]
    pub rpm: Option<Rpm>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none", default)]
    pub params: Option<Params>,
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none", default)]
    pub logs: Option<Logs>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none", default)]
    pub results: Option<Results>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Params {
    #[serde(rename = "param", default)]
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Fetch {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(
        rename = "@ssl_verify",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ssl_verify: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Rpm {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@path")]
    pub path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Logs {
    #[serde(rename = "log", default)]
    pub logs: Vec<Log>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@filename")]
    pub filename: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Results {
    #[serde(rename = "result", default)]
    pub results: Vec<TaskResult>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskResult {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@result")]
    pub result: String,
    #[serde(rename = "@score", skip_serializing_if = "Option::is_none", default)]
    pub score: Option<String>,
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none", default)]
    pub logs: Option<Logs>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN_JOB: &str = r#"<job>
    <recipeSet>
        <recipe id="1" status="Running" result="None" whiteboard="example">
            <params>
                <param name="GLOBAL" value="foo"/>
            </params>
            <task id="1" name="/example/task" status="Completed" result="Pass" start_time="2025-01-01T00:00:00Z" end_time="2025-01-01T00:05:00Z" duration="300">
                <params>
                    <param name="TASK_PARAM" value="bar"/>
                </params>
                <logs>
                    <log path="/recipes/1/tasks/1/logs/taskout.log" filename="taskout.log"/>
                </logs>
                <results>
                    <result id="1" path="/example/task" result="Pass" score="100">
                        <logs>
                            <log path="/recipes/1/tasks/1/results/1/logs/result.log" filename="result.log"/>
                        </logs>
                    </result>
                </results>
            </task>
        </recipe>
    </recipeSet>
</job>"#;

    #[test]
    fn parses_full_run_state() {
        let job: Job = quick_xml::de::from_str(RUN_JOB).expect("parse");
        let recipe = &job.recipe_sets[0].recipes[0];
        assert_eq!(recipe.id, "1");
        assert_eq!(recipe.status, "Running");
        assert_eq!(recipe.result, "None");

        let task = &recipe.tasks[0];
        assert_eq!(task.id, "1");
        assert_eq!(task.status, "Completed");
        assert_eq!(task.result, "Pass");
        assert_eq!(task.duration.as_deref(), Some("300"));

        let logs = task.logs.as_ref().expect("logs");
        assert_eq!(logs.logs.len(), 1);
        assert_eq!(logs.logs[0].filename, "taskout.log");

        let results = task.results.as_ref().expect("results");
        assert_eq!(results.results.len(), 1);
        let result = &results.results[0];
        assert_eq!(result.id, "1");
        assert_eq!(result.score.as_deref(), Some("100"));
        let result_logs = result.logs.as_ref().expect("result logs");
        assert_eq!(result_logs.logs[0].filename, "result.log");
    }

    #[test]
    fn round_trips_lossless_through_typed_model() {
        let job: Job = quick_xml::de::from_str(RUN_JOB).expect("parse");
        let xml = quick_xml::se::to_string(&job).expect("serialize");
        let again: Job = quick_xml::de::from_str(&xml).expect("re-parse");
        assert_eq!(job, again);
    }
}

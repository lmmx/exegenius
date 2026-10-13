//! Scratch harness for the brief pilot: renders an audit task for a written page with
//! exegenius's own `audit_task`, and checks an audit answer with `apply_audit`, on a project
//! whose other pages are unwritten. Nothing is saved to the project.
use exegenius_core::audit::{AUDIT_INSTRUCTIONS, AuditAnswer, apply_audit, audit_task, levels};
use exegenius_core::project::Project;
use std::path::Path;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (cmd, dir, page) = (&a[1], Path::new(&a[2]), &a[3]);
    let project = Project::load(dir).unwrap();
    let mut graph = project.load_graph().unwrap().unwrap();
    let plan = exegenius_core::plan::plan(&graph, &project.sources).unwrap();
    let i = plan.order.iter().position(|p| p == page).unwrap();
    let body = project.page_body(page).unwrap().expect("page written");
    match cmd.as_str() {
        "task" => {
            let out = &a[4];
            let schema = serde_json::to_string_pretty(&schemars::schema_for!(AuditAnswer)).unwrap();
            print!(
                "# audit of `{page}`\n\n{AUDIT_INSTRUCTIONS}\n\n{}\n## Answer\n\nThe answer is one JSON object following this schema:\n\n```json\n{schema}\n```\n\nWrite the answer as JSON to `{out}` (an absolute path). Do not run any other command.\n",
                audit_task(&project, &graph, &plan, i, &body)
            );
        }
        "check" => {
            let answer: AuditAnswer =
                serde_json::from_str(&std::fs::read_to_string(&a[4]).unwrap()).unwrap();
            let argument = exegenius_core::argument::load(&project).unwrap();
            let lv = levels(argument.as_ref(), page);
            match apply_audit(&mut graph, &plan, i, &body, answer.clone(), lv) {
                Ok(r) => println!("accepted: {r:?}"),
                Err(e) => println!("rejected: {e}"),
            }
        }
        _ => panic!("task|check"),
    }
}

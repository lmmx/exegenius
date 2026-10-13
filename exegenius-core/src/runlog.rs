//! The run log, `exegenius/log.jsonl`: one JSON object per line for each task handed out
//! and each submit, with its outcome, so a run can be read back without the agent's
//! transcripts.
//!
//! Every event has `at` (milliseconds since the Unix epoch), `version` (of exegenius),
//! `event` and `task` (the task id). The events:
//!
//! - `issued`: a task was handed out. Also `kind` (the task kind, as in task ids) and
//!   `round` (the audit round).
//! - `submit`: `exegenius submit` ran on a task. Also `kind` (absent if the id was not a
//!   pending task), `attempt` (this task's submits so far, this one included), `outcome`
//!   (`accepted`, `rejected` or `error`), `message` (for a rejection or error, what submit
//!   printed), `warnings` (for an acceptance, the warnings), `check` (for a rejection by a
//!   check on the stage's output, which: `coverage`), and, for extract and argument
//!   tasks whose answer parses, `coverage` (see [`crate::coverage`]): where the answer's
//!   quotes fall in each source, recorded whether or not the answer was accepted.
//!
//! The log records; the one decision that reads it is how many times the coverage check has
//! sent a task's answer back ([`coverage_rejections`]). Writing it is best effort: a
//! failure to write it never changes the outcome of `next` or `submit`.

use std::io::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::coverage::Coverage;
use crate::project::Project;

/// One line of the run log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Milliseconds since the Unix epoch.
    pub at: u64,
    /// The version of exegenius that wrote the event.
    pub version: String,
    #[serde(flatten)]
    pub what: What,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum What {
    Issued {
        task: String,
        kind: String,
        round: u32,
    },
    Submit {
        task: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<String>,
        attempt: usize,
        outcome: Outcome,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        warnings: Vec<String>,
        /// For a rejection by a check on the stage's output rather than by the answer's own
        /// checks: which check (`coverage`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        check: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        coverage: Option<Vec<Coverage>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Accepted,
    /// The answer was checked and rejected; the task stays pending.
    Rejected,
    /// No answer was checked: the id was not a pending task, an answer file was missing,
    /// or a file could not be read or written.
    Error,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

/// Append an event to the run log, ignoring failures.
pub fn append(project: &Project, what: What) {
    let event = Event {
        at: now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        what,
    };
    let Ok(line) = serde_json::to_string(&event) else {
        return;
    };
    let path = project.log_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(file, "{line}");
    }
}

/// The run log's events, skipping lines that do not parse. Empty if there is no log.
pub fn load(project: &Project) -> Vec<Event> {
    std::fs::read_to_string(project.log_path())
        .map(|text| {
            text.lines()
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// Submits of `task` recorded so far.
pub fn submits(events: &[Event], task: &str) -> usize {
    events
        .iter()
        .filter(|e| matches!(&e.what, What::Submit { task: t, .. } if t == task))
        .count()
}

/// Rejections of `task` by the coverage check recorded so far.
pub fn coverage_rejections(events: &[Event], task: &str) -> usize {
    events
        .iter()
        .filter(|e| {
            matches!(&e.what, What::Submit { task: t, check: Some(c), .. }
                if t == task && c == "coverage")
        })
        .count()
}

/// For one task kind: tasks handed out, submits by outcome, and tasks accepted at their
/// first submit.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct KindSummary {
    pub kind: String,
    pub issued: usize,
    pub accepted: usize,
    pub rejected: usize,
    /// Rejections by the coverage check, counted among `rejected`.
    pub coverage: usize,
    pub errors: usize,
    pub first_time: usize,
}

/// Per task kind, in the order kinds were first handed out.
pub fn summarise(events: &[Event]) -> Vec<KindSummary> {
    let mut out: Vec<KindSummary> = Vec::new();
    fn entry<'a>(out: &'a mut Vec<KindSummary>, kind: &str) -> &'a mut KindSummary {
        match out.iter().position(|s| s.kind == kind) {
            Some(i) => &mut out[i],
            None => {
                out.push(KindSummary {
                    kind: kind.to_string(),
                    ..KindSummary::default()
                });
                out.last_mut().expect("just pushed")
            }
        }
    }
    for e in events {
        match &e.what {
            What::Issued { kind, .. } => entry(&mut out, kind).issued += 1,
            What::Submit {
                kind: Some(kind),
                attempt,
                outcome,
                check,
                ..
            } => {
                let s = entry(&mut out, kind);
                if check.as_deref() == Some("coverage") {
                    s.coverage += 1;
                }
                match outcome {
                    Outcome::Accepted => {
                        s.accepted += 1;
                        if *attempt == 1 {
                            s.first_time += 1;
                        }
                    }
                    Outcome::Rejected => s.rejected += 1,
                    Outcome::Error => s.errors += 1,
                }
            }
            What::Submit { kind: None, .. } => {}
        }
    }
    out
}

/// The coverage recorded at the last accepted submit of a task of `kind`.
pub fn last_coverage<'a>(events: &'a [Event], kind: &str) -> Option<&'a [Coverage]> {
    events.iter().rev().find_map(|e| match &e.what {
        What::Submit {
            kind: Some(k),
            outcome: Outcome::Accepted,
            coverage: Some(c),
            ..
        } if k == kind => Some(c.as_slice()),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn submit(task: &str, attempt: usize, outcome: Outcome) -> Event {
        Event {
            at: 0,
            version: "0".into(),
            what: What::Submit {
                task: task.into(),
                kind: Some("write".into()),
                attempt,
                outcome,
                message: None,
                warnings: Vec::new(),
                check: None,
                coverage: None,
            },
        }
    }

    #[test]
    fn events_round_trip_as_tagged_lines() {
        let e = Event {
            at: 5,
            version: "0.6.0".into(),
            what: What::Issued {
                task: "0001-extract".into(),
                kind: "extract".into(),
                round: 1,
            },
        };
        let line = serde_json::to_string(&e).unwrap();
        assert_eq!(
            line,
            r#"{"at":5,"version":"0.6.0","event":"issued","task":"0001-extract","kind":"extract","round":1}"#
        );
        assert_eq!(serde_json::from_str::<Event>(&line).unwrap(), e);
    }

    #[test]
    fn summaries_count_first_time_acceptances() {
        let issued = |task: &str| Event {
            at: 0,
            version: "0".into(),
            what: What::Issued {
                task: task.into(),
                kind: "write".into(),
                round: 1,
            },
        };
        let events = vec![
            issued("0001-write-a"),
            issued("0002-write-b"),
            submit("0001-write-a", 1, Outcome::Accepted),
            submit("0002-write-b", 1, Outcome::Rejected),
            submit("0002-write-b", 2, Outcome::Accepted),
        ];
        assert_eq!(submits(&events, "0002-write-b"), 2);
        assert_eq!(
            summarise(&events),
            [KindSummary {
                kind: "write".into(),
                issued: 2,
                accepted: 2,
                rejected: 1,
                coverage: 0,
                errors: 0,
                first_time: 1,
            }]
        );
    }
}

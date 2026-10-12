//! Support checks: does each cited quotation support the claim it is attached to?
//!
//! Submit already checks that a `<cite>` mark's text is in its source word for word
//! ([`crate::cite::check`]). That shows the quotation exists, not that it supports the claim
//! before it. Once the pages are final, a support task per page labels each mark, after
//! ARC's supported, missing and contradicted labels: `supported`, `partial`, `unsupported`
//! or `contradicted`, judged from the quotation alone. Unsupported and contradicted marks
//! become rewrite notes; partial ones go to the report.
//!
//! The judge is an agent, so its labels are evidence, not proof. Two controls bound that:
//! marks whose quotation repeats the claim are skipped, since they always pass; and the
//! first page checked is labelled twice, by independent tasks, so that disagreement between
//! judges can be reported. A page is checked at most twice, so a writer who cannot find
//! support does not loop forever: after that, its marks go to the report only.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Result, read_json, write_json};
use crate::graph::Graph;
use crate::plan::Plan;
use crate::project::Project;
use crate::simulate::rejected;
use crate::text::normalise;

/// Most support checks per page.
const CHECKS: u32 = 2;

/// How a rewrite note for an unsupported or contradicted citation begins.
pub const SUPPORT_NOTE: &str = "The quotation does not support the claim it is attached to";

pub const SUPPORT_INSTRUCTIONS: &str = "\
Check whether each quotation on a page of an explanatory wiki supports the claim it is \
attached to. Writers back claims taken from the sources with a quotation copied from them; a \
quotation can exist in the source and still not support what it is attached to.

- Judge each claim against its quotation alone, not against what you know or what the \
source may say elsewhere.
- `supported`: the quotation states the claim or directly implies it.
- `partial`: the quotation supports part of the claim, or a weaker version of it.
- `unsupported`: the quotation does not address what the claim asserts.
- `contradicted`: the quotation says something incompatible with the claim.
- For every label but `supported`, say in one line what is missing or wrong.
- Label every quotation listed, each once.";

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    /// The quotation states the claim or directly implies it.
    Supported,
    /// The quotation supports part of the claim, or a weaker version.
    Partial,
    /// The quotation does not address what the claim asserts.
    Unsupported,
    /// The quotation says something incompatible with the claim.
    Contradicted,
}

impl Verdict {
    pub fn name(self) -> &'static str {
        match self {
            Verdict::Supported => "supported",
            Verdict::Partial => "partial",
            Verdict::Unsupported => "unsupported",
            Verdict::Contradicted => "contradicted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Label {
    /// Number of the quotation, as listed.
    pub mark: usize,
    pub label: Verdict,
    /// What is missing or wrong; may be empty for `supported`.
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SupportAnswer {
    pub labels: Vec<Label>,
}

/// A citation mark in context: the claim it follows, its quotation, and its paragraph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mark {
    pub page: String,
    pub source: String,
    pub claim: String,
    pub quote: String,
    pub paragraph: String,
}

/// A judge's label on a mark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Judgment {
    pub mark: Mark,
    pub label: Verdict,
    pub reason: String,
}

/// Every label so far, and how many checks each page has had.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Support {
    pub judged: Vec<Judgment>,
    /// Labels from the second, independent judge of the sampled page.
    #[serde(default)]
    pub second: Vec<Judgment>,
    #[serde(default)]
    pub checks: BTreeMap<String, u32>,
}

impl Support {
    fn judgment(&self, mark: &Mark) -> Option<&Judgment> {
        self.judged.iter().find(|j| same(&j.mark, mark))
    }

    /// Marks labelled by both judges, and how many of them got the same label.
    pub fn agreement(&self) -> (usize, usize) {
        let both: Vec<(&Judgment, &Judgment)> = self
            .second
            .iter()
            .filter_map(|s| Some((self.judgment(&s.mark)?, s)))
            .collect();
        let agree = both.iter().filter(|(a, b)| a.label == b.label).count();
        (both.len(), agree)
    }
}

fn same(a: &Mark, b: &Mark) -> bool {
    a.page == b.page
        && normalise(&a.claim) == normalise(&b.claim)
        && normalise(&a.quote) == normalise(&b.quote)
}

pub fn load(project: &Project) -> Result<Support> {
    let path = project.support_path();
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Support::default())
    }
}

/// The marks on a page body, each with the claim before it and its paragraph. Marks whose
/// quotation repeats the claim are left out: they pass by construction.
pub fn marks(page: &str, body: &str) -> Vec<Mark> {
    let (found, _) = crate::cite::marks(body);
    let mut out = Vec::new();
    for mark in found {
        let before = &body[..mark.range.start];
        let paragraph_start = before.rfind("\n\n").map_or(0, |i| i + 2);
        let paragraph_end = body[mark.range.end..]
            .find("\n\n")
            .map_or(body.len(), |i| mark.range.end + i);
        // The claim: from the end of the previous sentence or mark to this mark.
        let claim_start = [". ", "? ", "! ", "</cite>"]
            .iter()
            .filter_map(|stop| {
                before[paragraph_start..]
                    .rfind(stop)
                    .map(|i| i + stop.len())
            })
            .max()
            .map_or(paragraph_start, |i| paragraph_start + i);
        let claim = crate::cite::strip(&before[claim_start..])
            .trim()
            .to_string();
        let paragraph = crate::cite::strip(&body[paragraph_start..paragraph_end])
            .trim()
            .to_string();
        if claim.is_empty() || normalise(&claim) == normalise(&mark.quote) {
            continue;
        }
        out.push(Mark {
            page: page.to_string(),
            source: mark.source,
            claim,
            quote: mark.quote,
            paragraph,
        });
    }
    out
}

/// Pages with marks no judge has labelled, that have had fewer than the most checks, in
/// reading order, with those marks.
pub fn unjudged(project: &Project, plan: &Plan) -> Result<Vec<(String, Vec<Mark>)>> {
    let support = load(project)?;
    let mut pages = Vec::new();
    for id in &plan.order {
        if support.checks.get(id).copied().unwrap_or(0) >= CHECKS {
            continue;
        }
        let Some(body) = project.page_body(id)? else {
            continue;
        };
        let marks: Vec<Mark> = marks(id, &body)
            .into_iter()
            .filter(|m| support.judgment(m).is_none())
            .collect();
        if !marks.is_empty() {
            pages.push((id.clone(), marks));
        }
    }
    Ok(pages)
}

/// Whether the second judge should also label `page`: the first page ever checked.
pub fn sampled(project: &Project, page: &str) -> Result<bool> {
    let support = load(project)?;
    Ok(support.checks.is_empty() && support.second.is_empty() && !page.is_empty())
}

pub fn support_task(project: &Project, marks: &[Mark]) -> String {
    let mut out = String::from("## Quotations\n");
    for (i, m) in marks.iter().enumerate() {
        let source = project
            .sources
            .iter()
            .find(|s| s.id == m.source)
            .map_or(m.source.clone(), |s| s.title.clone());
        out.push_str(&format!(
            "\n### {}\n\nClaim: {}\n\nQuotation, from {source}: \"{}\"\n\nParagraph:\n\n> {}\n",
            i + 1,
            m.claim,
            m.quote,
            m.paragraph.replace('\n', "\n> ")
        ));
    }
    out
}

/// Check the labels on `marks` and store them. Labels from the first judge queue a rewrite
/// for each unsupported or contradicted mark; labels from the second judge are kept for the
/// agreement figure only.
pub fn apply_support(
    project: &Project,
    graph: &mut Graph,
    page: &str,
    marks: &[Mark],
    second: bool,
    answer: SupportAnswer,
) -> Result<()> {
    let mut errors = Vec::new();
    let mut labels: Vec<Option<&Label>> = vec![None; marks.len()];
    for l in &answer.labels {
        match labels.get_mut(l.mark.wrapping_sub(1)) {
            None => errors.push(format!("quotation {} does not exist", l.mark)),
            Some(Some(_)) => errors.push(format!("quotation {} is labelled twice", l.mark)),
            Some(slot) => {
                if l.label != Verdict::Supported && l.reason.trim().is_empty() {
                    errors.push(format!(
                        "quotation {}: say what is missing or wrong",
                        l.mark
                    ));
                }
                *slot = Some(l);
            }
        }
    }
    for (i, l) in labels.iter().enumerate() {
        if l.is_none() {
            errors.push(format!("no label for quotation {}", i + 1));
        }
    }
    if !errors.is_empty() {
        return Err(rejected(errors));
    }
    let mut support = load(project)?;
    for (m, l) in marks.iter().zip(labels) {
        let l = l.expect("every mark is labelled");
        let judgment = Judgment {
            mark: m.clone(),
            label: l.label,
            reason: l.reason.trim().to_string(),
        };
        if second {
            support.second.push(judgment);
            continue;
        }
        if matches!(l.label, Verdict::Unsupported | Verdict::Contradicted) {
            graph.queue_rewrite(
                page,
                Some(format!(
                    "{SUPPORT_NOTE} ({}): \"{}\" cites \"{}\": {} Say only what the source \
                     supports, or cite the passage that supports the claim.",
                    l.label.name(),
                    m.claim,
                    m.quote,
                    judgment.reason
                )),
            );
        }
        support.judged.push(judgment);
    }
    if !second {
        *support.checks.entry(page.to_string()).or_default() += 1;
    }
    write_json(&project.support_path(), &support)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_carry_their_claim_and_skip_restatements() {
        let body = "Intro. The grid is resampled <cite data-source=\"p\">We resample the \
                    grid.</cite> and then rounded <cite data-source=\"p\">rounded</cite>.\n\n\
                    It is exact <cite data-source=\"p\">It is exact</cite>.";
        let found = marks("page", body);
        assert_eq!(found.len(), 2, "{found:?}");
        assert_eq!(found[0].claim, "The grid is resampled");
        assert_eq!(found[1].claim, "and then rounded");
        assert_eq!(
            found[0].paragraph,
            "Intro. The grid is resampled and then rounded."
        );
    }
}

# 2026-10-11: Citation support

This entry implements the seventh research entry, 2026-10-11-evidence-layers (proposal I-93, extending I-36). That entry put evidence in four layers: citation integrity, quotation integrity, claim support, and explanatory adequacy. Exegenius checked the second, and the simulation addresses the fourth. This entry adds the third and a metadata check for the first.

## What was built

### Layer 3: a support task per page

Once the pages are final, a `support` task is handed out for each page with citation marks no judge has labelled yet, in the same batch as the outline and overloads tasks (exegenius-core/src/support.rs). This answers the research entry's open question 1: the check runs once the pages are final, not after every write.

For each mark the task gives:
- **the claim**: the text from the end of the previous sentence, or the previous mark, up to the mark;
- **the quotation** and the title of its source;
- **the paragraph** around the claim.

The judge labels each mark `supported`, `partial`, `unsupported` or `contradicted`, after ARC's supported, missing and contradicted labels. The judgement is made from the quotation alone, not from what the judge knows or what the source says elsewhere, since a claim should be supported by what it cites. Submit rejects a missing or doubled label, and any label but `supported` without a reason.

What the labels do:
- **`unsupported` and `contradicted`** become rewrite notes on the page, quoting the claim, the quotation and the reason, and asking the writer to say only what the source supports or to cite the passage that does.
- **`partial`** goes to the report only. This answers open question 2.
- **A rewritten claim is a new mark,** so the page is checked again. A page is checked at most twice. After that its marks go to the report only, so a writer who cannot find support does not loop.

The research entry's two controls are built:
- **No-op marks are skipped.** A mark whose quotation repeats its claim passes by construction, so it is left out of the task.
- **Judge noise is measured.** The first page checked gets a second, independent support task. Its labels raise no rewrite notes. `status` and the report give how many marks both judges labelled and how many got the same label.

The report's "Citation support" section lists every mark not labelled `supported`, with its label, claim, quotation and reason, and the judges' agreement. `status` counts the labels.

### Layer 1: source metadata

`status` names the sources that have no URL, saying that a DOI, arXiv or stable URL lets readers check them. The template's comment on `url` says the same. This answers open question 4. Checking that a URL resolves needs the network, and the core stays deterministic, so it is not done.

## Where the build departs from the research entry

1. **The sample for double labelling is one page,** the first one checked, not a fraction of all pages. It sets a floor on cost. A larger sample would need a configuration setting, and one page already yields an agreement figure.
2. **Concept evidence quotes are not checked for support** (open question 3). They back concepts, not sentences, so there is no claim to judge them against.
3. **Selecting passages before writing is not built.** The research entry deferred it until the support check showed how often citations fail.

## How it was tested

The end-to-end test gives the chirp page one citation mark whose claim, "So $nk$ is exact", is not what its quotation says. The outline batch then carries two support tasks, the second judge's included. A label with no reason is rejected. The first judge's `unsupported` becomes a rewrite note that the next write task shows. The rewritten claim is checked a second time and labelled `partial`, and the stored labels keep the second judge's disagreement. A unit test checks mark extraction, including that a mark restating its claim is skipped; another checks the report section.

There was no demo check. The demo repository's pages carry no citation marks: the marks from the repair round described in 2026-10-10-simulated-readers are not in its published commit.

## Current State
- `support::marks` extracts each citation mark's claim, quotation and paragraph from a page body and skips marks whose quotation repeats the claim (exegenius-core/src/support.rs:150)
- `next` issues a support task per page with unjudged marks and fewer than two checks, in the batch with the outline, and a second task for the first page ever checked (exegenius-core/src/task.rs, exegenius-core/src/support.rs:191, exegenius-core/src/support.rs:213)
- `apply_support` requires one label per mark and a reason for every label but `supported`, queues a rewrite note for unsupported and contradicted marks, counts the page's checks, and stores labels in `exegenius/support.json`, keeping the second judge's labels apart (exegenius-core/src/support.rs:240)
- `status` counts support labels, reports the judges' agreement, and names sources without a URL; the report lists marks not labelled supported with the agreement (exegenius-cli/src/main.rs, exegenius-core/src/report.rs)

## Missing
- No support check of concept evidence quotes
- No passage selection before writing
- No double labelling beyond the first page checked
- No check that a source URL resolves

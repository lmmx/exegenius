# 2026-10-13: The coverage check rejects by default

**The decision.** After the recovery experiment and its replication (2026-10-13-recovery-experiment), the author approved making `[checks] coverage = "reject"` the default.

**The evidence behind it,** over eleven clean planning runs:
- the planners stopped at the introduction in seven;
- the four of those whose answers were not sent back produced arguments that anchor the paper's central derivations only in its introduction;
- the three that were sent back read the body sections and produced arguments anchored where the derivations are, at under two minutes per rejection.

**The change.** On branch `131026-coverage-default`, kept apart from the feature branch until it is reviewed:
- `CoverageCheck::Reject` is the default;
- the configuration template's comment and the README say so;
- `coverage = "report"` keeps the old behaviour.

**What the default does not cover:**
- an extraction that quotes the opening of most sections but none of their results passes the check, as B6's did in the replication;
- the planners in those runs were orchestrated by the runner session's model, not Haiku;
- the evidence is from one paper.

## Current State
- `exegenius submit` sends back an extract or argument answer that leaves more than half of a marked-up source's top-level sections developing results unquoted, unless `exegenius.toml` sets `[checks] coverage = "report"` (`CoverageCheck` in exegenius-core/src/config.rs)
- The end-to-end test `the_coverage_check_rejects_by_default_and_can_only_report` checks both settings on an introduction-only extraction answer (exegenius-core/tests/task_loop.rs)

## Missing
- No full run (writing and auditing) with the coverage check rejecting
- No evidence from a source other than the n log n paper

I think the important shift is this: the next generation of frontier models may become much better at deciding what is worth doing, not just at doing what they've been asked to do.

For Exegenius, that could be more consequential than a jump in raw coding ability. You're already trying to build a system that can detect when its output is inadequate, work out why, and choose a useful recovery. The next step is a system that can also recognise that its original approach to the problem was wrong, discover a better framing, and make a targeted change to its own methods.

That is what I would investigate under the label research taste. But I'd distinguish the demonstrated capabilities of current models from the more speculative possibilities of future ones.

## What the evidence says about “research taste”

There are already some interesting results, and they suggest a more nuanced future than “models are becoming better researchers.”

1\. Choosing the right direction is a distinct capability.

A September 2026 study, The Tasteful Agent, evaluates decisions at forks in long-running engineering and research trajectories. The best frontier models answered only 59.7% of its taste questions correctly; more reasoning budget didn't improve accuracy. Yet training a model on outcome-informed judgments improved decisions on unseen tasks.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://arcxiv.org\&sz=32)

ArcXiv



The implication: a model can be brilliant at executing a plan and still choose a poor plan. Conversely, judgment may be improved through learning from past decisions and their consequences.

2\. Research judgment may improve unevenly.

Anthropic's August 2026 TASTE evaluation found Fable 5 at 60% agreement with experienced researchers' preferences on its selected proposal pairs, against an estimated human agreement level of 77%. The benchmark deliberately focuses on cases where researchers agreed strongly. So even a frontier model with strong general capabilities remained imperfect at this particular kind of research judgment.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://alignment.anthropic.com\&sz=32)

alignment.anthropic.com



This matters because judging which idea is promising, which problem is important, and which experiment is worth running are related but different skills.

3\. In some bounded research settings, the advance may be much larger.

A recent TasteVal study reports that Opus 5.5 exceeded its expert baseline on experimental research taste, measured by the compute efficiency of designing experiments and interpreting their outcomes. It reports a 2.3× compute multiplier, with a wide 95% confidence interval of 1.15–4.37, across eight challenging tasks. That's promising but not evidence of general research superiority.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://arcxiv.org\&sz=32)

ArcXiv



The important development is that researchers are starting to measure the quality of decisions over time, not just final answers or coding success.

These findings don't all measure the same thing, and their apparent differences are informative. A model might be excellent at experiment selection, weaker at judging abstract proposals, and poor at deciding which architectural detour is worthwhile. We should expect a profile of research capabilities, not a single universal research-taste score.

## What could models soon be able to do?

My forecast is that the biggest near-term change will be from task execution towards iterative, evidence-guided problem solving.

| Emerging capability            | What it could mean in practice                                                                                    |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------- |
| Diagnose failures              | Identify not only that a wiki is weak, but the missing reasoning step or faulty system assumption that caused it. |
| Find high-value interventions  | Distinguish a consequential architectural weakness from a minor prompt or style issue.                            |
| Design informative experiments | Choose tests that discriminate between competing explanations, rather than accumulate more examples.              |
| Learn from outcomes            | Reuse knowledge about which interventions succeeded, failed, or only appeared to succeed.                         |
| Coordinate long-horizon work   | Maintain a coherent research programme across code, experiments, documentation and model updates.                 |
| Recognise when to stop         | Know when evidence is sufficient, when an approach has stalled, and when human judgement is genuinely needed.     |

The strongest systems will combine these capabilities with tools, persistent records and reliable verification. A more capable model doesn't automatically acquire a reliable memory of what happened in your repo, or a calibrated understanding of what counts as success. Those properties have to be engineered.

## What this changes about the Exegenius task

The task we just drafted asks Claude to read an article, find a weakness, and implement an improvement. That's a good test of the emerging capability—but it is still a one-off.

The bigger opportunity is to turn it into a repeatable research-and-intervention loop that gets more valuable as models improve.

The distinction is important:

- Today: We give a capable model a bounded research task and inspect what it produces.
- Next: We give a model access to the project's goals, history, tests and previous interventions, and ask it to identify the highest-value next move.
- Further ahead: The model maintains a portfolio of hypotheses about how to improve the system, runs discriminating experiments, learns from the outcomes, and updates its priorities—within explicit resource and authority limits.

The last two are not capabilities we should simply assume will work. They're the capabilities we should build a way to evaluate.

## The intervention I'd make to Exegenius's development process

I would introduce a reusable Research-to-Intervention Loop, initially as a documented protocol and lightweight set of tools, rather than a new autonomous subsystem.

1\. Persistent research state

Goals, known failures, previous experiments, decisions, unresolved hypotheses and resource constraints.

2\. Find the highest-value opportunity

Inspect the actual system. Identify a consequential weakness, its likely cause and competing explanations.

3\. Choose the most informative intervention

Prefer the smallest test or change that distinguishes plausible explanations and could improve the system.

4\. Test and record the outcome

Measure improvement, regressions, cost, uncertainty and whether the original hypothesis survived.

5\. Update the research state

Retain what was learned, revise priorities and choose the next opportunity.

Repeat at meaningful milestones, not on a fixed schedule regardless of evidence.

The crucial element is step five. Without it, each new model starts another clever investigation. With it, every investigation can build on the last.

### The key engineering idea: record decision forks, not just final outcomes

This is where the recent research on taste becomes especially relevant.

Suppose a previous development session encountered a failure in mathematical explanations. It had several choices:

- Add another prompt instruction.
- Increase reasoning effort.
- Change the planner.
- Build a check for a missing derivation.
- Investigate whether the evaluation itself is measuring the wrong thing.

The final commit alone doesn't tell a future model which alternative was worth taking, which evidence was available at the time, or why the other choices were rejected.

I'd therefore preserve a compact record of consequential decisions:

- Situation: What weakness or uncertainty prompted the decision?
- Options: Which plausible interventions were considered?
- Evidence available then: What justified the choice?
- Prediction: What should happen if the hypothesis is right?
- Outcome: What actually happened, including cost and side effects?
- Lesson: What should a future development session do differently?

This makes the project's history a potential training and evaluation resource for research judgment, rather than just a record of code changes.

The Tasteful Agent work suggests that outcome-informed judgments at decision forks can be useful for training and evaluation, although it does not establish that this particular record format will improve Exegenius.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://arcxiv.org\&sz=32)

ArcXiv



## How to make this repeatable across future models

I'd create three reusable assets.

A. Intervention benchmark

A small collection of historical Exegenius problems with the repository state at the time, available evidence, competing interventions and known outcomes. Give the model the information available before the original decision, then evaluate its choice without revealing the outcome.

This tests whether newer models choose better directions—not merely whether they can explain past decisions once the answer is known.

B. Intervention protocol

A standard procedure for investigating a problem, selecting a change, implementing it, testing the result and recording the lesson. Include resource limits and boundaries for autonomous action.

This is what makes the article-inspired task reproducible. The article becomes one input to the process, not the reason the process exists.

C. Model capability record

For each model and relevant version, record the quality of its diagnoses, intervention choices, experimental designs, implementation results and resource use. Keep these dimensions separate.

When a new model arrives, replay the same historical problems and compare it with the previous model. Then give it a fresh problem to check whether improvements transfer.

The benchmark should avoid turning into a set of answer keys that models memorise. Use a mixture of historical cases, hidden decision forks and fresh problems. Also compare models under comparable tool access, time and resource budgets.

This is how we can discover whether Fable—or its successor—is genuinely better at research judgment, rather than just more persuasive in its reports.

## What I'd do now, versus later

A proposed sequence, not a request to interrupt the current E3 work.

1. Now: Finish the current article-to-intervention task. Let it produce one tested improvement. Record the decision and its evidence using the format above.
2. Next: Extract a few existing development episodes from Exegenius's history. Choose cases with meaningful alternatives and outcomes, not merely successful coding tasks. Turn these into the first decision-fork evaluations.
3. When Fable becomes available: Run the same protocol and historical cases through it. Compare its choices, the quality of the resulting interventions and the evidence it uses to justify them.
4. Only after that: Decide whether to build a persistent autonomous research agent, a model-assisted development workflow, or simply a better repeatable process. The results should determine the architecture.

## The deeper opportunity

The most interesting possibility is not just that future models will be better at research. It is that research judgment may become an experimentally improvable and measurable part of our software-development process.

That changes the role of the human, too. Rather than selecting every experiment and correcting every methodological choice, you establish the objective, preserve the evidence, set the boundaries and review the consequential decisions. The model handles more of the search for what to do next.

But there is an important distinction between a model that looks like it has research taste and one that repeatedly makes decisions that lead to better outcomes under comparable constraints. The latter is what we should measure.

My recommendation is to make the Research-to-Intervention Loop the reusable unit of work, and the decision-fork benchmark the mechanism for testing whether each new model is better at it. That gives us something we can use immediately, something that becomes more valuable with each development cycle, and a way to exploit future model improvements without having to redesign the whole project every time a new frontier model arrives.
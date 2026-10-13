# 3. Verification inside the loop

## Checking a stage before the next one uses it

When you build a chatbot as an agent, you do not take each user message and act on it directly. You first check whether the message suits what the agent is for. An airline's booking assistant should not be usable for writing a Python calculator. That check is part of what separates early agent designs from later ones.

**The same applies between stages here.** After the extraction, a step could ask whether the extraction was actually done: whether it drew on the document, rather than on one part of it. If the evidence were not spread through a 70-page document, that would show the document was not fully processed. The run would learn this immediately, instead of the next eight hours of tasks building on it. The quote spread that exposed this failure was found by analysis after the run. It could have been a check during the run.

**Status:** proposal.

**A caution from ChatGPT:** a rule such as "quotes must reach past the first N per cent" would be brittle. Some papers develop their main result early. Coverage would have to be judged against what the target depends on, and where the source develops it, not against an even spread for its own sake.

## Fail fast as something to intensify

"Fail fast" is underappreciated in agent design. But my real thought is that it is not on or off; it is a matter of how far it goes. A check can always be made deeper.

One way to put it: picture the agent as having already failed at the task, in some ridiculous way you have not thought of. Then, rather than planning recovery after the run, add many checks during it, and make the verification intense. Only an intense verification catches the qualitative failures that would otherwise slip through and propagate. Reading only the introduction is one: every structural check passed.

## Recovery before failure

The aim is not that every check aborts the run. The better version of the idea:
1. Move the checks we now do as analysis after a run into the generation process itself, as a detailed system of checks.
2. When a check finds a nuanced failure, recover from it where possible: retry, read the missing sections, split the work.
3. Fail only if it cannot be recovered.

ChatGPT spelled this out as stages:
- **detection:** is this intermediate result suspect;
- **diagnosis:** what is wrong with it: unread sections, a poor extraction, an incomplete plan;
- **recovery:** a targeted retry or supplement;
- **escalation:** when to stop or to continue with a result marked partial;
- **verification:** of the recovery itself.

That matches what I meant. Which conditions are hard failures, which are warnings and which trigger a recovery is not decided.

**Status:** proposal; the design is open.

## A preflight check of the setup

**Something like a dry run.** The tool could report what a run would do. The agent could then check whether the setup can succeed before spending tokens on the run. Not only "what would happen", but whether the configuration is well formed for what it asks: whether the subagent definitions suit the tasks and the source.

**The example I tried did not hold.** I asked whether Haiku has enough context for a 5,000-line LaTeX file. ChatGPT said its context window is a million tokens, far more than the file needs, so that check would have found nothing. I am not sure which checks belong in such a mode.

**Status:** open question. The need is to validate feasibility and configuration before an expensive run. A preflight command is one possible form of it, not a conclusion. A dry run may use model tokens: the tool itself spends none, but the agent can use the tool's output to check its own setup.

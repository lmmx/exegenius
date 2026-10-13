# 8. How I want to work

## Taking myself out of the loop

I have spent this time reading one long message from the Claude Code session, aloud and slowly, rather than passing it straight to ChatGPT and accepting the prompt it wrote back. ChatGPT responded at once with how I should respond. I wanted to think about it more.

Some would say that putting a person in the loop slows things down. What I am doing is trying to get myself out of the loop by thinking about it differently. I have now drawn out what went wrong far enough to hand the work to an autonomous agent session that can drive it.

**What I want next:**
- **The session.** An agent session with both repositories: exegenius, and the paper's repository. It turns out a repository can be added to an existing Claude Code cloud session, so no new session is needed. The wiki repository has since been added to this session, for reading.
- **The experiments.** That session runs the planning experiments itself, inspects the artifacts and logs, compares outcomes and iterates, rather than asking me to pull out transcripts and set up runs.
- **What I keep:** oversight of the conceptual direction. I still want to do some manual runs with working software.

This is not a licence to spend tokens without thought. The experiments should be reproducible, and their conclusions should show their evidence.

**Status:** decision in part. The repository is added. How autonomous the next session is, and its budget, are open.

## Reading output written for agents

The analysis was hard for me to follow. It reads as written for an agent:
- terms such as "chain", "owned", "cite or cut" and `relied_on`, used without explanation;
- dense tables of counts;
- the main point arriving late.

It took a lot of words to reach a fairly simple conclusion:
- we did not test what we thought we were testing;
- planning did not cover the paper;
- the earlier model comparison was confounded;
- the handoff barely got a test.

**What I want from reports to me:** the conclusion first, the terms explained, and the detail after.

## Engaging with the detail

ChatGPT's reply opened, in bold, with "don't introduce Opus yet". I read that as a safeguarding move: protecting me from a decision I might regret, in money or time. That is my reaction to how the advice was presented, not a claim about anyone's intent. It reads as a caring way to write, but I do not need the softly-softly approach. Today I have the time to go through this properly, and I do not want to throw tokens at a problem and expect it to be solved.

## Recording my reasoning, not my approvals

Accepting prompts that one model wrote for another, with what amounts to "looks good to me", does not record my own reasoning. I want the documentation to keep:
- why I agree;
- what I think the evidence implies;
- which assumptions I question;
- which ideas are still exploratory.

**Where it goes:**
- **Long-form notes** like these, in a directory named after a journal entry: an outline, then numbered sections. One markdown file would not suit a discussion that went through many phases.
- **The journal entry** stays the structured summary. It links the notes, and draws the actions and open questions up out of them rather than being written in one shot.
- **Style:** written prose, not a transcript, and free of mannered prose. Anthropic's prompt-engineering guidance describes mannered prose, and an agent writing such notes could refer to it.
- **Decision records:** the rationale for an architectural change, such as the switch to restricted subagents, deserves a record of its own, like an ADR. I do not want to start reworking the journal's format now, but the information architecture of this tooling is something I want to improve.

**Status:** decision, for these notes; the decision-record format is open.

## Beyond this paper

The approach of turning a long technical source into an explained, checkable structure could apply to many other things than LaTeX papers. That is not a task now, only a note on why the design questions matter beyond this paper.

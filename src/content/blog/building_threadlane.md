## A workspace around the work

An AI coding session is more than a prompt and a reply. There is a repository to understand, a plan to follow, commands to run, changes to review, and often more than one line of investigation. A useful workspace should make those pieces easy to find without making the conversation harder to follow.

[Threadlane](https://github.com/wheregmis/threadlane) brings that work into a native desktop application built with Rust and GPUI. It combines project-scoped sessions, branching conversations, persistent terminals, and an agent runtime behind one interface.

![Threadlane desktop workspace showing project sessions, an agent conversation, and Git review](/images/threadlane-workspace.jpg)

*The Threadlane workspace, from the project's public README.*

## Native at the interface, deliberate underneath

A coding workspace spends much of its life in motion: text streams in, a tool finishes, a diff changes, another task needs attention. The interface needs to make that activity legible without overwhelming the person using it.

Threadlane uses GPUI for its GPU-accelerated desktop interface. Its workspace includes rendered Markdown, code diffs, tool activity, and keyboard-oriented command discovery.

The design principle is straightforward: keep the work close to its context. A terminal is more useful when its project is clear. A diff is more useful when it sits beside the discussion that led to it. A plan is more useful when it belongs to the session it describes.

## A conversation can have more than one lane

Some investigations belong beside the main conversation rather than inside it. Exploring a dependency, comparing approaches, or tracing a failure can all produce useful detail without needing to interrupt the main thread.

Threadlane's runtime represents foreground work and delegated work in separate lanes. Its conversation trees preserve branches, while the trajectory inspector exposes execution records for closer inspection.

That separation makes an important distinction visible: the answer someone reads is not the entire history of the work. The interface can stay focused while the execution history remains available.

## Record intent before acting

Agent software has to deal with interruption. An application might close after requesting a tool operation but before receiving its result. When the session opens again, what actually happened?

Threadlane's Harness V2 records execution intent before dispatching provider or tool actions. An append-only JSONL history supports reconstruction of session state and interruption handling.

A simplified view of the design is:

```text
Record intent → Dispatch operation → Record outcome
       ↓                                  ↓
       └──────── Session history ──────────┘
```

The useful question is not simply whether a session can reopen. It is whether reopening it can distinguish completed work from work whose outcome is uncertain. Retrying a read and repeating a write are different decisions; recovery should preserve that distinction.

## Extensions with clear boundaries

Threadlane connects to MCP servers and external ACP agents, and supports sandboxed WASI extensions. These are different integration paths, not interchangeable names for the same capability.

Keeping those boundaries explicit helps the workspace grow without forcing every tool into the desktop interface itself. The UI can concentrate on presenting the work; integrations can concentrate on their own responsibilities.

## What this project explores

Threadlane sits at an intersection that interests me: expressive interfaces and reliable systems. A good interface helps someone understand what is happening. A durable runtime helps that understanding survive an interruption.

Neither layer can substitute for the other. A polished transcript is not a recovery strategy, and a detailed execution log is not automatically a good reading experience. The opportunity is to design them together.

The project is open source. Visit the [Threadlane repository](https://github.com/wheregmis/threadlane) for source code, setup instructions, and the current architecture. The [README](https://github.com/wheregmis/threadlane#readme) is the reference for the implementation details described here.

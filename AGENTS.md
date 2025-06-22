# Contributor Guide

## Tasks

- When asked to complete a task, or when asked to complete the next task, **ALWAYS** check `/tasks/process-task-list-codex.md` and follow that process.
- When asked to work fully autonomously, use the following process:
  1. **Begin at the top of the task list and complete sub-tasks one by one in order, following all instructions in `/tasks/process-task-list-codex.md` unless otherwise directed here.**
  2. **After each sub-task:**
     - Mark the sub-task `[x]` and update any relevant documentation.
     - If all subtasks under a parent are complete, mark the parent task `[x]`.
     - Check for the following _stopping conditions_:
       - Project is finished (no sub-tasks remaining).
       - Any issues arise, including:
         - Project scope increases by more than ~3 sub-tasks in a cycle,
         - Tooling is not functioning properly,
         - The agent is unable to figure out how to implement a sub-task,
         - The user needs to make a decision or provide input.
     - **If any stopping condition is met, STOP working and ask the user how to proceed.**
  3. **If no stopping conditions are met, proceed to the next sub-task and repeat step 2.**

# MindsHub Demo Prompt

Copy and paste this exact prompt into MindsHub during your recording:

---

> Look at the open issues in my Rust repo, find the 3 most common problems
> people are hitting, and write a brief status update I can add to the README.

---

## What MindsHub Should Do

1. Connect to GitHub and read open issues
2. Group issues by theme (async, borrow checker, build issues, docs, etc.)
3. Identify top 3 most common pain points
4. Write a polished markdown status update

## Expected Output Example

```markdown
## Project Status — Open Issues Summary

**Top issues the community is running into:**

1. **Async + borrow checker conflicts** (4 open issues) — Users hitting E0502
   when passing references into async closures. Workaround: wrap in Arc<Mutex<>>.
   Fix planned for v0.4.

2. **Windows/MSVC build failures** (2 open issues) — Compilation fails on
   Windows with MSVC toolchain due to cc crate linking. Linux/macOS unaffected.
   Investigation in progress.

3. **Tokio runtime context panics** (3 open issues) — tokio::spawn called
   outside runtime in library contexts. Added example to docs; deeper fix TBD.
```

---

## Tips for the Recording

- Make sure your GitHub account is connected in MindsHub before starting
- Use a repo with at least 5 open issues for a realistic demo
- If using demo data, point MindsHub at `demo-data/sample-issues.json`

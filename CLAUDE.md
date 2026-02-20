# CLAUDE.md

## Commit Rules

- Always use [Conventional Commits](https://www.conventionalcommits.org/) format: `<type>(<scope>): <description>`
- Only include a concise header — no body, no footer
- Common types: `feat`, `fix`, `chore`, `refactor`, `docs`, `test`, `style`, `perf`

**Examples:**
```
feat(ui): add scrollable pane navigation
fix(input): handle escape key in modal
chore: update dependencies
```

## Pull Request Rules

- MUST read this file before raising a PR
- Use the commit message as the PR title (verbatim)
- Do NOT paraphrase or reword — copy the commit header exactly

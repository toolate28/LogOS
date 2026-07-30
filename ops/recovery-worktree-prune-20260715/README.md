# Worktree prune — 2026-07-15 (post F: crash recovery)

## Removed
- C: Grok worktree `C:\Users\toolated\.grok\worktrees\matthew-ruhnau-logos\2026-06-29-7ca42856` (detached `bd6f8712`)
- F: linked worktree `LogOS.worktrees\master` (branch `main` @ `995463f4`) — unregistered from git
- Non-git leftovers under `LogOS.worktrees\` (agent-tools, mcps, terminals, coherence-mcp)
- `coherence-mcp\.claude\worktrees\` nested copies (~1.1 GB; not real git worktrees)

## Preserved
- Branch `main` still exists in LogOS primary repo @ `995463f4`
- Branch `master` primary @ `7f9443fb`
- Patches from dirty F: worktree: `*.diff` / status lists in this directory

## Primary checkout
`F:\Users\Matthew Ruhnau\LogOS` only (`git worktree list` → single entry)

---
name: git-branch
description: Manage Git branches following Git Flow strategy with consistent naming conventions. Use when creating, switching, merging branches, or when the user asks about branch naming, branch strategy, or Git Flow workflow. (Git Flow 전략과 일관된 명명 규칙에 따라 브랜치를 관리합니다. 브랜치 생성, 전환, 병합 또는 브랜치 명명법, 전략에 대한 질문 시 사용합니다.)
---

# Git Branch Management

Git Flow 전략 기반 브랜치 관리 가이드.

## Branch Types

| Branch      | Purpose               | Base      | Merge To          |
| ----------- | --------------------- | --------- | ----------------- |
| `main`      | Production-ready code | -         | -                 |
| `develop`   | Integration branch    | `main`    | `main`            |
| `feature/*` | New features          | `develop` | `develop`         |
| `release/*` | Release preparation   | `develop` | `main`, `develop` |
| `hotfix/*`  | Production bug fixes  | `main`    | `main`, `develop` |

## Naming Convention

Pattern: `type/description`

- Use lowercase
- Use hyphens for spaces
- Keep description concise but descriptive

### Examples

```
feature/add-recipe-sharing
feature/youtube-url-validation
fix/recipe-parsing-error
release/1.0.0
hotfix/login-crash
```

### Type Prefixes

| Prefix      | When to Use             |
| ----------- | ----------------------- |
| `feature/`  | New functionality       |
| `fix/`      | Bug fixes (on develop)  |
| `hotfix/`   | Urgent production fixes |
| `release/`  | Release preparation     |
| `refactor/` | Code restructuring      |
| `chore/`    | Maintenance tasks       |

## Common Workflows

### Start a New Feature

```bash
# From develop branch (develop 브랜치에서)
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name
```

### Finish a Feature

```bash
# Update and merge (업데이트 및 병합)
git checkout develop
git pull origin develop
git merge feature/your-feature-name

# Delete feature branch (기능 브랜치 삭제)
git branch -d feature/your-feature-name
```

### Create a Release

```bash
# From develop (develop에서)
git checkout develop
git checkout -b release/1.0.0

# After testing, merge to main and develop (테스트 후 main과 develop에 병합)
git checkout main
git merge release/1.0.0
git tag -a v1.0.0 -m "Release 1.0.0"

git checkout develop
git merge release/1.0.0
git branch -d release/1.0.0
```

### Hotfix Production Issue

```bash
# From main (main에서)
git checkout main
git checkout -b hotfix/critical-bug

# After fix, merge to both main and develop (수정 후 main과 develop 모두에 병합)
git checkout main
git merge hotfix/critical-bug
git tag -a v1.0.1 -m "Hotfix 1.0.1"

git checkout develop
git merge hotfix/critical-bug
git branch -d hotfix/critical-bug
```

## Project-Specific Scopes

For this monorepo, consider including scope in branch names when appropriate:

```
feature/mobile-recipe-detail
feature/api-auth-endpoint
fix/spaghetti-worker-timeout
```

## Quick Reference

| Action          | Command                                |
| --------------- | -------------------------------------- |
| List branches   | `git branch -a`                        |
| Create & switch | `git checkout -b type/name`            |
| Switch branch   | `git checkout branch-name`             |
| Delete local    | `git branch -d branch-name`            |
| Delete remote   | `git push origin --delete branch-name` |
| Rename current  | `git branch -m new-name`               |
| Show current    | `git branch --show-current`            |

## Best Practices

1. **Keep feature branches short-lived** - Merge frequently to avoid conflicts
2. **Always branch from correct base** - Features from `develop`, hotfixes from `main`
3. **Pull before branching** - Ensure you have the latest changes
4. **Delete merged branches** - Keep branch list clean
5. **Use descriptive names** - Branch name should indicate purpose

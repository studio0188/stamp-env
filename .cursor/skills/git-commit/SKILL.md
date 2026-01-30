---
name: git-commit
description: Generate commit messages following Conventional Commits format. Use when the user asks to commit changes, write a commit message, or review staged changes for committing. (Conventional Commits 형식에 따라 커밋 메시지를 생성합니다. 커밋 변경, 커밋 메시지 작성, 스테이징된 변경 사항 검토 요청 시 사용합니다.)
---

# Git Commit Message

Generate commit messages following the Conventional Commits specification.

## Format

```
<type>[(scope)]: <description>

[body]

[footer]
```

## Types

| Type       | Purpose                                                 |
| ---------- | ------------------------------------------------------- |
| `feat`     | New feature                                             |
| `fix`      | Bug fix                                                 |
| `docs`     | Documentation only                                      |
| `style`    | Code style (formatting, semicolons, etc.)               |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf`     | Performance improvement                                 |
| `test`     | Adding or updating tests                                |
| `build`    | Build system or external dependencies                   |
| `ci`       | CI configuration                                        |
| `chore`    | Other changes (deps update, config, etc.)               |

## Rules

1. **Type**: Required. Use lowercase.
2. **Scope**: Optional. Use when change is specific to a module/area.
   - Examples: `feat(auth)`, `fix(mobile)`, `docs(api)`
3. **Description**: Required. Use imperative mood, lowercase, no period.
   - ✅ `add user authentication`
   - ❌ `Added user authentication.`
4. **Body**: Optional. Explain "why" not "what".
5. **Footer**: Optional. For breaking changes or issue references.

## Scopes for This Project

| Scope       | Area                    |
| ----------- | ----------------------- |
| `mobile`    | React Native / Expo app |
| `api`       | Fastify backend         |
| `spaghetti` | Background worker       |
| `shared`    | Shared packages         |

## Examples

**Simple feature (간단한 기능):**

```
feat: add youtube url validation
```

**Feature with scope (범위 포함 기능):**

```
feat(mobile): implement recipe detail view
```

**Bug fix with body (본문 포함 버그 수정):**

```
fix(api): handle empty video response

YouTube API returns null for deleted videos, causing JSON parse error
```

**Breaking change (주요 변경 사항):**

```
feat(api)!: change recipe response format

BREAKING CHANGE: recipe.steps is now an array of objects instead of strings
```

## Workflow

1. Run `git diff --staged` to analyze staged changes
2. Identify the type based on the nature of changes
3. Determine scope if changes are localized to one area
4. Write a concise description in imperative mood
5. Add body if the "why" needs explanation

## Commit Command

Use HEREDOC for multi-line messages:

```bash
git commit -m "$(cat <<'EOF'
feat(mobile): add recipe sharing functionality

Enable users to share recipes via native share sheet
EOF
)"
```

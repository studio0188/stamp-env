# stamp.env

프리셋 기반 폴더/파일 구조를 심링크로 빠르게 배포하는 Rust CLI 도구.

## 핵심 명령어

- `stamp link <preset> [target]` - 프리셋을 대상 경로에 심링크로 생성
- `stamp unlink [target]` - 생성된 심링크 제거
- `stamp commit <name> [-p <patterns>...]` - 현재 구조를 프리셋으로 저장 (glob 패턴 필터 지원)
- `stamp list` - 저장된 프리셋 목록
- `stamp show <preset>` - 프리셋 내용 확인

## 프로젝트 구조

```
src/
├── main.rs            # CLI 진입점 (clap 사용)
├── commands/          # CLI 명령어 핸들러
│   ├── mod.rs
│   ├── link.rs        # link 명령 구현
│   ├── unlink.rs      # unlink 명령 구현
│   ├── commit.rs      # commit 명령 구현
│   └── list.rs        # list/show 명령 구현
├── preset/            # 프리셋 관리 로직
│   ├── mod.rs
│   └── manager.rs     # 프리셋 저장/로드
└── symlink/           # 심링크 생성 로직
    ├── mod.rs
    └── builder.rs
```

## 코딩 규칙

- **에러 처리**: `anyhow` 크레이트 사용
- **프리셋 형식**: TOML 직렬화 (`serde`, `toml`)
- **심링크 충돌**: 기존 파일 존재 시 사용자 확인 필요
- **CLI 파싱**: `clap` derive 매크로 사용

## 의존성

| 크레이트 | 용도               |
| -------- | ------------------ |
| clap     | CLI 인자 파싱      |
| anyhow   | 에러 처리          |
| serde    | 직렬화/역직렬화    |
| toml     | 프리셋 파일 형식   |
| walkdir  | 디렉토리 재귀 탐색 |
| glob     | 파일 패턴 매칭     |

## 프리셋 저장 위치

`~/.config/stamp.env/presets/` 디렉토리에 TOML 파일로 저장됨.

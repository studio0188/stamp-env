# stamp-env

프리셋 기반 폴더/파일 구조를 심링크로 빠르게 배포하는 Rust CLI 도구.

## 설치

### Homebrew (macOS/Linux)

```bash
brew install studio0188/tap/stamp
```

### Shell (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/studio0188/stamp-env/releases/latest/download/stamp-env-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -c "irm https://github.com/studio0188/stamp-env/releases/latest/download/stamp-env-installer.ps1 | iex"
```

### Cargo

```bash
cargo install stamp-env
```

## 사용법

### 핵심 명령어

- `stamp link <preset> [target]` - 프리셋을 대상 경로에 심링크로 생성
- `stamp unlink [target]` - 생성된 심링크 제거
- `stamp commit <name> [-p <patterns>...]` - 현재 구조를 프리셋으로 저장 (glob 패턴 필터 지원)
- `stamp list` - 저장된 프리셋 목록
- `stamp show <preset>` - 프리셋 내용 확인

### 예제

```bash
# 현재 디렉토리 구조를 'my-preset' 프리셋으로 저장
stamp commit my-preset

# 특정 패턴의 파일만 프리셋으로 저장
stamp commit my-preset -p "*.rs" -p "Cargo.toml"

# 프리셋을 대상 경로에 심링크로 생성
stamp link my-preset ./target-dir

# 생성된 심링크 제거
stamp unlink ./target-dir
```

## 프리셋 저장 위치

`~/.config/stamp.env/presets/` 디렉토리에 TOML 파일로 저장됩니다.

## 라이선스

MIT License

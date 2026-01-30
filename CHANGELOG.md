# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-31

### Added

- Initial release
- `stamp link` - 프리셋을 대상 경로에 심링크로 생성
- `stamp unlink` - 생성된 심링크 제거
- `stamp commit` - 현재 구조를 프리셋으로 저장 (glob 패턴 필터 지원)
- `stamp list` - 저장된 프리셋 목록
- `stamp show` - 프리셋 내용 확인
- `--sync` 플래그로 link 트래킹 및 자동 동기화 지원

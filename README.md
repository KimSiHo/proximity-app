# proximity-app

DeepStream과 연동되는 Rust 기반 이벤트 처리 애플리케이션입니다.

객체 추적 결과를 분석하여 이벤트를 판단하고, UDS를 통해 DeepStream의 Smart Record를 제어합니다.

## 동작 흐름

```
DeepStream (ds app)
      │
      ▼
Detection Packet (ds app -> rust app, 오브젝트 detection 정보 UDS 로 전송)
      │
      ▼
Tracking (rust app, 링 버퍼 자료구조로 최근 n 초간 detection 정보 저장)
      │
      ▼
Event Detection (rust app, n 초간 detection 정보 기반으로 로직 판단)
      │
      ▼
Command (START / STOP) (rust app -> ds app, 명령 UDS 로 전송)
      │
      ▼
DeepStream Smart Record (smart recoding, 동영상 저장)
```

## 개발 방식

- DeepStream과 UDS를 통해 Detection 정보를 수신합니다.
- 객체 추적 이력을 기반으로 이벤트를 판단합니다.
- 이벤트 발생 시 Smart Record 시작/종료 명령을 전송합니다.
- 공통 프로토콜은 C 헤더를 기준으로 관리하며, `build.rs`에서 `bindgen`을 사용해 Rust Binding을 자동 생성합니다.

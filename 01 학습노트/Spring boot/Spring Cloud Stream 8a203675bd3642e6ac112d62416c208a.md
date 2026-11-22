# Spring Cloud Stream

Pub/Sub 패턴을 기반으로 하는 메시징 방식 스트리밍 추상화를 제공

Apache Kafka와 RabbitMQ를 기본 지원

- Message : 메시징 시스템과 주고받는 데이터를 설명하는 데이터 구조
- Publisher : 메시징 시스템에 메시지를 보냄
- Subscriber : 메시징 시스템에서 메시지를 받음
- Channel : 메시징 시스템과 통신하는데 사용
- Binder : 특정 메시징 시스템과 통합 기능을 제공

![Untitled](Spring%20Cloud%20Stream/Untitled.png)

기본입력 채널 (Sink)과 출력채널 (Source)를 제공
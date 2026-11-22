# Micro Service Architecture (MSA)

> 출처: [Notion 페이지](https://app.notion.com/p/9dbbf3389e5d4476b760edc77085a6b1) · 동기화: 2026-11-22

# 마이크로 서비스의 정의

일체형 어플리케이션을 작은 컴포넌트로 나누는 것, 무엇을 위해?

- 빠르게 개발해 지속적으로 배포할 수 있어야 한다.
- 수동 혹은 자동으로 쉽게 스케일링 할 수 있어야 한다.

## 독립된 컴포넌트로 동작하기 위한 기준

- 컴포넌트 간 아무것도 공유되지 않는 아키텍처여야 함
- 명확한 인터페이스를 통해서만 통신
- 개별적인 런타임 프로세스로 배포되어야 함
- 마이크로서비스 인스턴스는 상태가 없다. (Stateless)

# 프로젝트 구성

프로젝트는 마이크로서비스, 배치 서비스, Presentation Layer를 담당하는 통합 Web서비스로 구성

시스템의 구분은 MS군-MS-아키텍처 조합으로 구분

MS는 Spring Boot를 기반으로 구성

## Layer 구성요구

- Controller : 특정 업무의 비즈니스 로직에 대한 Flow와 외부 인터페이스를 제어
  - 외부 시스템 접근은 API Gateway로 일원화
  - MS간의 인터페이스를 위한 접점으로 활용
- Consumer : 컨트롤 컴포넌트에서 재사용되는 단위로 식별되는 비즈니스 로직 Layer
  - DB I/O를 위한 DTO ↔ Entity변환 및 IO Mapper 호출
  - 외부 인터페이스 제어
  - Message Broker를 통한 비동기 통신의 호출 접점 담당
- REST Interface : 다른 MS의 API를 동기 호출을 위한 선언 (Outbound)
- Producer : 비동기 통신 담당 (Outbound)
- Mapper Interface : 데이터의 조회/생성/삭제/변경을 담당하는 데이터 처리 인터페이스
  - SQL의 결과 Layout을 Dataset으로 처리
  - SQL은 Mybatis기반의 XML로 작성
- DTO : Contoller와 Service간의 데이터 전달시에 사용하며 로직을 포함하지 않음
- Entity : DTO와 구분되며 DB의 테이블과 동일하게 매핑, lombok을 사용하여 별도 메소드 작성안함

## 연관 라이브러리

- lombok : 코드 간결성을 유지
- swagger : 조사필요

## Rest API

URI 구조

- 서버 주소 : https://{서버주소}/{시스템코드}/{도메인}/{엔티티}
- 버전 : v1, v2 (시스템 단위)
- 리소스명(API명) : Controller구성 단위
- Query String : 데이터 객체

인증/인가 처리방안

OIDC Path / JWT 토큰

## 하위 페이지

- [[마이크로서비스 디자인패턴]]
- [[Chain of failure]]
- [[콘웨이의 법칙]]
- [[Service Mesh]]

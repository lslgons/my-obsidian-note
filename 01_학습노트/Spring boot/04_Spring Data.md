# Spring Data

RDB, MongDB, Redis 등의 NoSQL데이터베이스 엔진에 이르는 다양한 유형을 지원하고 저장하기 위한 공통 프로그래밍 모델

# JPA (Java Persistence API)

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Spring%20boot_Spring%20Data_Untitled.png)

## Entity

스프링 데이터가 저장하는 데이터

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Spring%20boot_Spring%20Data_Untitled%201.png)

MongoDB의 경우 MongoDB하위 프로젝트 애노테이션과 일반 스프링 데이터 애노테이션을 혼합하여 사용

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Spring%20boot_Spring%20Data_Untitled%202.png)

## Repository

여러유형의 데이터베이스에 데이터를 저장하고 접근하고자 사용

자바 인터페이스로 선언되며 스프링 데이터가 상황에 따른 구현을 독자적인 규칙에 맞추어 생성

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Spring%20boot_Spring%20Data_Untitled%203.png)

리포지토리를 서비스에 주입하여 사용

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Spring%20boot_Spring%20Data_Untitled%204.png)
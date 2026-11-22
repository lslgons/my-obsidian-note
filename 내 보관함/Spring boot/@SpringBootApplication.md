# @SpringBootApplication

> 출처: [Notion](https://app.notion.com/p/9345004647844cafabbc464741730b0f)

- 컴포넌트 검색을 활성화하여 클래스의 패키지와 그 하위 모든 패키지에서 컴포넌트와 구성요소를 검색
- 어플리케이션 클래스 자체를 구성 클래스로 만듬
- 자동 설정을 활성화하여 스프링부트가 설정가능한 JAR파일을 클래스 패스에서 자동으로 찾게 함
  - → 톰캣이 클래스패스에 있는 경우 스프링 부트는 톰캣을 내장형 웹서버로 자동구성 함

![](attachments/springboot-001.png)

## @Component

하위 패키지의 컴포넌트, Bean

@Autowired 애노테이션을 통해 주입이 가능

![](attachments/springboot-002.png)

## @Configuration

- 하위 패키지 탐색으로 애노테이션에의한 설정 클래스로 등록
- @ComponentScan은 해당 패키지에서 컴포넌트를 검색함
- @SpringBootApplication 애노테이션은 @Configuration 애노테이션이기도 함

![](attachments/springboot-003.png)

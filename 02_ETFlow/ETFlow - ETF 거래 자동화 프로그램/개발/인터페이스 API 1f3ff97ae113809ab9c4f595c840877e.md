# 인터페이스 API

세부기능 수행 API 서버

FastAPI 기반

31_api_server

# 개발로그

## 2025년 6월 19일

### 구현 완료기능

모의계좌 기준 시세수집 API 구현

### Todo. 계좌정보 가져오기

→ 일자별로 주요 계좌정보를 저장하도록 함, API호출 시 DB에 자동 적재되도록 구성 (일자기준 항상 업데이트 되도록…

→ 장 종료 시 일괄적으로 업데이트 하도록 배치를 하나 만들어놔야 함

→ 계좌 갱신주기 : 거래 (매수/매도)시  갱신, Frontend에서 조회할 경우 갱신, 시세정보 업데이트 마다 (10분) 

→ Table 정보

- 계좌번호 리스트
- 계좌별 잔고 정보 (일자별)
- 계좌별 보유종목 현황 리스트 (일자별)

[https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-balance](https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/trading/inquire-balance)
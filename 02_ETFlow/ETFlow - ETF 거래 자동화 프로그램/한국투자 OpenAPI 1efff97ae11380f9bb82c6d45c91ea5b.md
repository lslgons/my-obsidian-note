# 한국투자 OpenAPI

# 토큰발급

모든 API의 시작은 토큰 발급을 시작으로 함

- 발급된 토큰의 유효기간은 1일이므로 매일 하루 토큰을 갱신하여 DB에 저장하는 스케줄링이 필요
- 각 스케줄링 API수행 시마다 DB에서 토큰값을 가져오는 작업을 수행하여야 함
- [https://openapi.koreainvestment.com:9443](https://openapi.koreainvestment.com:9443/)/oath2/tokenP

```
{
    "grant_type": "client_credentials",
    "appkey": "{{PROD_APPKEY}}",
    "appsecret": "{{PROD_APPSECRET}}"
}
```

# 시세조회

종목별 현재 시세와 정보를 조회함, 중복된 정보가 많으므로 시세 (OHLC)정보, 거래량 정보만을 가져와 저장

- https://apiportal.koreainvestment.com/apiservice-apiservice?/uapi/domestic-stock/v1/quotations/inquire-daily-price
-
# AWS

AWS 기초 : [https://aws.amazon.com/ko/getting-started/fundamentals-core-concepts/?e=gs2020&p=gsrc](https://aws.amazon.com/ko/getting-started/fundamentals-core-concepts/?e=gs2020&p=gsrc)

리눅스 가상머신 구축 :[https://aws.amazon.com/ko/getting-started/hands-on/launch-a-virtual-machine/](https://aws.amazon.com/ko/getting-started/hands-on/launch-a-virtual-machine/)

# 리전 설정

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_AWS_Untitled.png)

# 1. EC2

## 1.1 인스턴스 시작

### 1.1.1 키페어(로그인)

인스턴스 액세스 권한

1. 새 키 페어 생성
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_AWS_Untitled%201.png)
    
2. RSA / .pem선택
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_AWS_Untitled%202.png)
    
3. 보안그룹 설정
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_AWS_Untitled%203.png)
    
4. 인스턴스 생성 후 로그인 (SSH) : WSL설치 가정 리눅스 쉘에서 실행

```yaml
1. SSH클라이언트를 엽니다.
2. 프라이빗 키 파일을 찾습니다. 이 인스턴스를 시작하는 데 사용되는 키는 key-01.pem입니다.
3. 필요한 경우 이 명령을 실행하여 키를 공개적으로 볼 수 없도록 합니다.
4. 퍼블릭 DNS을(를) 사용하여 인스턴스에 연결:
5. ssh -i "key-01.pem" ec2-user@ec2-43-201-28-99.ap-northeast-2.compute.amazonaws.com
```
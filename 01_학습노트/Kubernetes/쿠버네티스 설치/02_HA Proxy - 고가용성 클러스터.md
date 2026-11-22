# HA Proxy - 고가용성 클러스터

마스터 노드 장애 시 워커노드 영향도를 제거하며 부하를 분산시키기 위한 목적

- 마스터노드 3개를 클러스터로 구성
- 가상 네트워크 (weave)적용
- ceph storage 적용

![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled.png)

1. 호스트네임 설정 : 마스터노드 간 서로 다르게 지정해야 함
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%201.png)
    
2. 도커설치
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%202.png)
    
3. daemon.json - cgroupdriver를 systemd옵션으로 사용하게 설정
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%203.png)
    
4. 쿠버네티스 설치 : kubelet, kubeadm, kubectl
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%204.png)
    
5. 로드밸런서 설치 (haproxy)
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%205.png)
    
6. 클러스터 생성
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%206.png)
    
7. 클러스터 연결
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%207.png)
    
8. 클러스터 구성확인
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%208.png)
    
9. 가상 네트워크 (weave)적용
    
    ![Untitled](/images/01_%ED%95%99%EC%8A%B5%EB%85%B8%ED%8A%B8_Kubernetes_%EC%BF%A0%EB%B2%84%EB%84%A4%ED%8B%B0%EC%8A%A4%20%EC%84%A4%EC%B9%98_HA%20Proxy%20-%20%EA%B3%A0%EA%B0%80%EC%9A%A9%EC%84%B1%20%ED%81%B4%EB%9F%AC%EC%8A%A4%ED%84%B0_Untitled%209.png)
    
10.
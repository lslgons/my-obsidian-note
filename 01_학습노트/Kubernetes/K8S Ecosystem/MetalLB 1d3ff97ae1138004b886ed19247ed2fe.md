# MetalLB

Load Balancer implementation for bare metal

쿠버네티스는 네트워크 로드밸런싱 기능을 기본 제공하지 않음 => Service 타입이 LoadBalancer로 하여도 Pending상태로 유지, CSP 업체에서는 자사 클러스터 서비스에서는 해당 기능을 Implementation 하여 제공

이 때 온프레미스 같은 베어메탈 환경의 쿠버네티스 클러스터에서 이러한 로드밸런서 타입을 쓰기위한 구현체로 MetalLB를 사용

### **설치전 설정준비**

kube-proxy 내 strictARP가 true여야 함

```
# 설정 적용 전체크
kubectl get configmap kube-proxy -n kube-system -o yaml | \
sed -e "s/strictARP: false/strictARP: true/" | \
kubectl diff -f - -n kube-system
# ConfigMap 적용
kubectl get configmap kube-proxy -n kube-system -o yaml | \
sed -e "s/strictARP: false/strictARP: true/" | \
kubectl apply -f - -n kube-system
```

### **Helm 설치**

```
helm repo add metallb https://metallb.github.io/metallb
helm pull metallb/metallb --untar
helm install metallb ./metallb -n metallb-system --create-namespace
```

### **IPAddressPool**

```
apiVersion: metallb.io/v1beta1
kind: IPAddressPool
metadata:
  name: global-ip-address-pool
  namespace: metallb-system
spec:
  addresses:
  - 172.16.0.110-172.16.0.200
  - 172.16.0.210-172.16.0.253
```

### **IP Advertisement Mode**

**L2 vs BGP 선택 기준 정리**

| 선택 기준 | L2 모드 | BGP 모드 |
| --- | --- | --- |
| **구성 난이도** | 간단 | 복잡 (라우터 설정 필요) |
| **멀티 서브넷 지원** | ❌ 불가능 | ✅ 가능 |
| **외부 라우터 연동** | ❌ 불가능 | ✅ 가능 |
| **로드밸런싱** | 노드 1개만 광고 | 여러 노드에 분산 가능 |
| **고가용성(HA)** | 낮음 | 높음 |
| **사용 환경** | 로컬 네트워크, 단순한 구성 | 데이터센터, 클라우드, 고가용성 필요 |

### **L2 모드를 선택해야 하는 경우**

- Kubernetes 클러스터가 **같은 서브넷에서 동작**하고 있음
- 추가적인 네트워크 설정 없이 간단하게 IP를 광고하고 싶음
- 사내 네트워크, 소규모 클러스터에서 운영

### **✅ BGP 모드를 선택해야 하는 경우**

- **멀티 서브넷 / 클라우드 환경**에서 MetalLB를 사용해야 함
- **고가용성(HA)과 노드 간 트래픽 분산이 필요**
- 외부 BGP 라우터와 **네트워크 연동이 필요한 경우**

```
# L2 Advertisement
apiVersion: metallb.io/v1beta1
kind: L2Advertisement
metadata:
  name: global-l2-advertisement
  namespace: metallb-system
spec:
  ipAddressPools:
  - global-ip-address-pool
```

```
# BGP Advertisement
apiVersion: metallb.io/v1beta2
kind: BGPPeer
metadata:
  name: sample
  namespace: metallb-system
spec:
  myASN: 64500
  peerASN: 64501
  peerAddress: 10.0.0.1
---
apiVersion: metallb.io/v1beta1
kind: BGPAdvertisement
metadata:
  name: example
  namespace: metallb-system
```
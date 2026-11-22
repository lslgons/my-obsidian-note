# Worker Node 점검

### **특정 node 점검**

- 해당 node의 스케줄링을 중지, pod 제거 후 작업

```bash
# 해당 node 스케줄링 중지
kubectl cordon {node name}
 
# 해당 node pod 제거
kubectl drain {node name} --force
 
# node 스케줄링 복구
kubectl uncordon {node name}
```

### Worker node 삭제

```bash
#worker node 삭제
kubectl delete node {worker node}
 
#삭제된 node에서 reset
kubectl reset
```

### Node 추가

참조 : [https://blog.psnote.co.kr/183](https://blog.psnote.co.kr/183)

master token 생성 및 확인

```bash
#토큰 생성
sudo kubeadm token create
#토큰 리스트
sudo kubeadm token list
#Discovery token CA Cert Hash
openssl x509 -pubkey -in /etc/kubernetes/pki/ca.crt | openssl rsa -pubin -outform der 2>/dev/null | openssl dgst -sha256 -hex | sed 's/^.* //'
 
#join 커맨드
kubeadm join --token 3jaeaqt.rumpgep17nqunhgl k8smaster:6443 --discovery-token-ca-cert-hash sha256:16e1304347fa2f5cb6f1660e582ed3501aa63e0ddce276773566098aef13d0ba
```
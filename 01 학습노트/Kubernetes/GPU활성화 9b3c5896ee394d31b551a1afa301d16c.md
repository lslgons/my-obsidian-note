# GPU활성화

### **조건**

- kubernetes version > 1.10
- **GPU를 할당하여 오브젝트를 실행할 노드**에 NVIDIA 드라이버가 설치되어 있어야 한다. (133서버)
    - `nvidia-smi` 명령어로 확인 가능
- nvidia-docker 2.0 설치
- nvidia-container-runtime이 기본 런타임으로 설정되어 있어야한다.
    - 컨테이너를 docker, containerd로 돌리는 것에 따라 준비 방식이 다름
    

**nvidia-container-runtime 설치**

```bash
# repository 등록
curl https://nvidia.github.io/nvidia-docker/centos7/nvidia-docker.repo > /etc/yum.repos.d/nvidia-docker.repo
 
# nvidia-container-runtime 설치
yum -y install nvidia-container-toolkit
```

**nvidia-container-runtime 기본 런타임 설정 (docker)**

```bash
# /etc/docker/daemon.json에 추가 필요
{
  "default-runtime": "nvidia",
  "runtimes": {
      "nvidia": {
          "path": "/usr/bin/nvidia-container-runtime",
          "runtimeArgs": []
      }
  }
}

systemctl restart docker
```

**GPU 디바이스 플러그인 설치**

```bash
$ wget https://raw.githubusercontent.com/NVIDIA/k8s-device-plugin/v0.12.3/nvidia-device-plugin.yml
 
$ kubectl apply -f nvidia-device-plugin.yml

$ kubectl get daemonset -n kube-system
 
NAMESPACE      NAME                             DESIRED   CURRENT   READY   UP-TO-DATE   AVAILABLE   NODE SELECTOR            AGE
kube-system    kube-proxy                       5         5         5       5            5           kubernetes.io/os=linux   5m56s
kube-system    nvidia-device-plugin-daemonset   4         4         4       4            4           <none>                   38s
```
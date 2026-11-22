# Kube-spray에 의한 클러스터 설치

쿠버네티스 클러스터를 구성하기 위해 다양한 요소를 한번에 설치하기 위함

- Online상태에서 설치가 진행되는 점을 고민해야 함
- Offline 모드로 설치하기 위해 컨테이너 이미지와 바이너리 파일들을 구성하는 Coordinator 서버를 구성
- [https://kubespray.io/#/docs/operations/offline-environment](https://kubespray.io/#/docs/operations/offline-environment)
- [https://github.com/kubernetes-sigs/kubespray/tree/master/contrib/offline](https://github.com/kubernetes-sigs/kubespray/tree/master/contrib/offline)

# **Kubespray 다운로드**

```
git clone https://github.com/kubernetes-sigs/kubespray.git
cd kubespray/inventory
cp -rf sample mycluster
```

## **Ansible 설치**

```
cd kubespray
python3 -m venv .venv
source .venv/bin/activate
pip install -U -r requirements.txt
```

# **Offiline 모드 설정**

- Some static files (zips and binaries)
- OS packages (rpm/deb files)
- Container images used by Kubespray. Exhaustive list depends on your setup
- [Optional] Python packages used by Kubespray (only required if your OS doesn't provide all python packages/versions listed in `requirements.txt`)
- [Optional] Helm chart files (only required if `helm_enabled=true`)

## **컨테이너 이미지 다운로드**

이미지/파일 리스트 생성

```
cd contrib/offline
sh ./generate_list.sh
```

contrib/offline/temp 폴더가 생성됨 (files.list / images.list)

오프라인 파일 다운로드 및 레포지토리 실행

```
./manage-offline-files.sh
```

http://<Server-IP>:8080에서 레포지토리 실행 확인가능

## **바이너리 Repository 생성**

> ceph-provisioner에서 Image Scheme 호환성 문제로 컨테이너 이미지 Pull이 되지 않음
> 
> 
> 해당 경로를 삭제해주어야 함
> 
> http://quay.io/external_storage/cephfs-provisioner:v2.1.0-k8s1.11
> 
> http://quay.io/external_storage/rbd-provisioner:v2.1.1-k8s1.11
> 

```
 export IMAGES_FROM_FILE=$(pwd)/temp/images.list
 sh manage-offline-container-images.sh create
 sh manage-offline-container-images.sh register
```

RockyLinux repo

```
sudo dnf install -y createrepo
wget https://download.rockylinux.org/pub/rocky/9/isos/x86_64/Rocky-9.5-x86_64-dvd.iso
sudo mkdir /mnt/rocky-dvd
sudo mount -o loop /home/user/Rocky-9.5-x86_64-dvd.iso /mnt/rocky-dvd
sudo mkdir /data/rocky9.5-repo
sudo cp -av /mnt/rocky-dvd/* /data/rocky9.5-repo
sudo createrepo /data/rhel8-repo
# docker repo
sudo dnf install yum-utils
sudo reposync --repo=docker-ce-stable -p /data/docker-repo/
sudo createrepo /data/docker-repo
docker run --name nginx-yum-repo -v /data:/usr/share/nginx/html:ro -p 9080:80 -d nginx

```

각 서버에 repo 등록

```
[rocky9]
name=Rocky Linux9 - Local
baseurl=http://172.16.0.10:9080/rocky9.5-repo
enabled=1
gpgcheck=0
# Docker는 Ansible에 의해 레지스트리 등록됨
[docker]
name=Docker CE
baseurl=http://172.16.0.10:9080/docker-repo
enabled=1
gpgcheck=0

```

각 서버 방화벽 데몬 비활성화

```
sudo systemctl disable firewalld
sudo systemctl stop firewalld
```

inventory/mycluster/all/offline.yml 파일 수정

```
## Global Offline settings### Private Container Image Registry
registry_host: "172.16.0.10:5000"
files_repo: "http://172.16.0.10:8080"
### If using CentOS, RedHat, AlmaLinux or Fedora
yum_repo: "http://172.16.0.10:8080"
```

하위 files_repo라 쓰여있는 곳도 모두 주석해제해야 함

inventory/mycluster/k8s-cluster/k8s-cluster.yml 수정

- 컨테이너 엔진 => Docker 로 변경

invenvtory/mycluster/

- private registry insecure 설정

```
## Container runtime
## docker for docker, crio for cri-o and containerd for containerd.
## Default: containerd
container_manager: docker

```

## **SSH 설정**

Coordinator에서 각 노드별로 SSH접근이 가능하도록 키 등록

```
ssh-keygen -t rsa -b 4096
ssh-copy-id root@172.16.0.101
ssh-copy-id root@172.16.0.102
ssh-copy-id root@172.16.0.103
ssh-copy-id root@172.16.0.201
ssh-copy-id root@172.16.0.202
ssh-copy-id root@172.16.0.203
```

# **클러스터 설치**

```
ansible-playbook -i inventory/mycluster/inventory.ini cluster.yml -b -v

```

노드 제거

```
ansible-playbook -i inventory/mycluster/inventory.ini remove-node.yml -b -v --extra-vars "node=primary-02" --extra-vars reset_nodes=true
```

노드추가

```
ansible-playbook -i inventory/mycluster/inventory.ini scale.yml -b -v
```
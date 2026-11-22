# Image Pull&Push

### **목적**

- 현재 Docker Hub에 저장되어있는 image를 **서버 내 Registry**에 저장
- **tag를 이용**하여 간단하게 image pull, push 가능

### **사용 방법**

- 현재 접속 ip 및 port: **~~dw-image-storage:80~~ → pCmsSuggestA2:5000 (210.122.104.134:5000)**

[https://docs.docker.com/registry/](https://docs.docker.com/registry/)

[https://github.com/docker-library/docs/tree/master/registry](https://github.com/docker-library/docs/tree/master/registry)

**Docker daemon에 insecure-registry 등록 (신규 registry를 등록할 경우)**

```yaml
# 파일 생성/수정
vi /etc/docker/daemon.json
 
# 위 파일에 아래 내용 추가
{
    "insecure-registries": ["<ip>:<port>"]
}
 
# docker restart
systemctl restart docker
```

**private registy에 image push**

```yaml
# Docker Hub에서 원하는 image 가져오기
docker pull centos
 
# 받은 Docker image에 tag 설정
## docker tag 이미지이름 <registry서버 ip>:5000/태그이름
docker tag centos <ip>:<port>/centos_image
 
# Docker tag를 이용하여 Docker registry에 push
docker push <ip>:<port>/centos_image
```

**주의 사항:** tag 이름에 Docker Registry ip:port를 작성해야만 이후 Docker push 시점에 해당 repository를 직접 찾아갈 수 있음

**private registry에서 image pull**

```yaml
# docker pull <registry서버 ip>:5000/태그이름
docker pull <ip>:<port>/centos_image
```

**registry내 이미지 조회 (Registry REST API)**

```yaml
# registry 목록 조회
curl -X GET pCmsSuggestA2:5000/v2/_catalog
 
# registry의 이미지 tag list 조회
curl -X GET pCmsSuggestA2:5000/v2/[Image Name]/tags/list
```

수동 delete 시 아래 블로그 참고

[https://fliedcat.tistory.com/178](https://fliedcat.tistory.com/178)
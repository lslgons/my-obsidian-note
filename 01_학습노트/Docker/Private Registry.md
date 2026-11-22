# Private Registry

Docker Hub를 사용하지 않고 자체적인 저장소를 이용

- 네트워크 적인 이슈 : 아웃바운드를 허용하지 않는경우
- 보안적인 이슈 : 어플리케이션을 Docker이미지를 생성학고 배포 시 외부노출 X
- 가용성 이슈 : 외부변수에 의한 시스템 안정성 영향도 X

[https://docs.docker.com/registry/configuration/](https://docs.docker.com/registry/configuration/)

저장소는 일반적인 FileSystem을 이용하는 방안과 S3와 같은 오브젝트 스토리지를 활용

On-Premise환경이면 MinIO스토리지 환경을 구축하여 활용

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
    name: registry-deployment
    namespace: registry
    labels:
        app: registry
spec:
    replicas: 1
    selector:
        matchLabels:
            app: registry
    template:
        metadata:
            labels:
                app: registry
        spec:
            containers:
            - name: registry
              image: registry:latest
              env:
              - name: REGISTRY_STORAGE_DELETE_ENABLED
                value: "true"
              # MinIO 연동
              - name: REGISTRY_STORAGE
                value: "s3"
              - name: REGISTRY_STORAGE_S3_REGION
                value: "ap-northeast-2"
              - name: REGISTRY_STORAGE_S3_ACCESSKEY
                value: "Tcu1CzNLFOMQ8N6Y"
              - name: REGISTRY_STORAGE_S3_SECRETKEY
                value: "YinNZqEwi1coo6ueM4nQbv0wavSfQtq9"
              - name: REGISTRY_STORAGE_S3_REGIONENDPOINT
                value: "http://ctmupload-cfs.cjoshopping.com"
              - name: REGISTRY_STORAGE_S3_BUCKET
                value: "cms"
              - name: REGISTRY_STORAGE_S3_ENCRYPT
                value: "false"
              - name: REGISTRY_STORAGE_S3_SECURE
                value: "false"
              - name: REGISTRY_STORAGE_S3_CHUNKSIZE
                value: "5242880"
              - name: REGISTRY_STORAGE_S3_ROOTDIRECTORY
                value: "/registry"
---
apiVersion: v1
kind: Service
metadata:
  name: registry-service
  namespace: registry
spec:
  selector:
    app: registry
  ports:
  - name: registry-port
    protocol: TCP
    port: 80
    targetPort: 5000
```
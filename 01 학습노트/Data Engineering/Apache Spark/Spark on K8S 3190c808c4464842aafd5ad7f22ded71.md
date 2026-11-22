# Spark on K8S

쿠버네티스를 Driver로 설정하여 spark-submit, pyspark, spark-shell을 실행

### **배포 이미지 생성**

쿠버네티스에 배포할 수 있는 Docker Image를 생성시켜야 함. spark에 이미지를 빌드할 수 있는 쉘스크립트를 제공하고 있음

```bash
# spark download후 이미지 빌드 툴 실행
# kubernetes/dockerfiles
bin/docker-image-tool.sh -r localhost:5005 -t rev-01 -p ./kubernetes/dockerfiles/spark/bindings/python/Dockerfile build
bin/docker-image-tool.sh -r localhost:5005 -t rev-01 push
```

### pyspark 실행

```bash
./pyspark --master k8s://https://10.88.22.201:26443 --deploy-mode client --conf spark.kubernetes.container.image=localhost:5005/spark-py:rev-01 --conf spark.kubernetes.pyspark.pythonVersion=3 --conf spark.kubernetes.namespace=dt-work   --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.1.2 --conf spark.driver.host=172.21.226.221 --num-executors=6 --executor-cores=2 --executor-memory=8G
```

## Spark Application 배포

Kubernetes에서 Spark Application을 실행하기 위해서는 Spark-Client라는 별도의 Pod에서 Spark-Submit을 실행하게 되고

이미 정의된 Config에 따라 master가 Kuberenetes로 설정되어 Executor는 워커노드의 Pod들로 수행하게 됨

구동되는 이미지 종류는 2가지

- localhost:5005/dt-client:latest : Spark Application을 실행하기 위한 클라이언트, Ubuntu 기반의 환경에서 spark-submit실행을 하기 위한 환경으로 최적화
- localhost:5005/spark:rev-01 : Spark On K8S를 실행하기 위해 빌드된 이미지, 마스터에 의해 Executor들은 이 이미지를 기준으로 생성하게 됨

Spark Application은 Spark-History 서버의 Pod 설정과 유사하며 실행되는 커맨드가 Spark-Submit으로 실행되는 부분이 차이

쿠버네티스 환경에서 실행되기 위해서는 driver host가 클라이언트 주소가 되어야 함 (개선과제로서 정의된 Yaml파일에서 driver host를 CoreDNS를 경유하여 잡힐수 있도록 구성하여야 함, 지금은 어플리케이션 마다 직접 커맨드를 손봐야 하는 구조)

spark application 코드는 파드 간에 공유가 될 수 있어야 하므로 마운트된 볼륨에서 찾을수 있도록 해야 함 : **/opt/data/spark/apps**

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spark-application-001
  labels:
    app: spark-application-001
spec:
  subdomain: spark-apps
  volumes:
  - name: spark-data
    hostPath:
      path: /data/shared/spark
      type: Directory
  - name: spark-config
    configMap:
      name: spark-config
      items:
        - key: spark-defaults.conf
          path: spark-defaults.conf
  containers:
  - name: spark-application-001
    image: localhost:5005/dt-client:latest
    # spark-submit 실행 커맨드...
    # command: ['/root/spark/spark-3.3.2-bin-hadoop3/bin/spark-submit', '--conf', 'spark.driver.host=$(ifconfig eth0 | grep 'inet '| cut -d: -f2 | awk '{print $2}')', '/opt/data/spark/apps/test.py']
    command: ['tail', '-f', '/dev/null']
    volumeMounts:
    - name: spark-data
      mountPath: /opt/data
    - name: spark-config
      mountPath: /root/spark/spark-3.3.2-bin-hadoop3/conf
  dnsPolicy: ClusterFirst
```

### ServiceAccount (ClusterRoleBinding)

```bash
$ kubectl get pod spark-application-001 -n spark -o yaml | grep serviceAccount
...
serviceAccount: default
serviceAccountName: default
...
 
$ kubectl get sa -n spark
NAME     SECRET     AGE
default  1          14d
 
$ kubectl create clusterrolebinding spark-app-role --clusterrole=edit --serviceaccount=spark:default --namespace=spark
```
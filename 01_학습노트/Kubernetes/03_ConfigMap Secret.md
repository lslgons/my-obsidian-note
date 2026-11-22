# ConfigMap / Secret

[https://arisu1000.tistory.com/27843](https://arisu1000.tistory.com/27843)

# Configmap

컨테이너에서 필요한 환경설정 내용을 컨테이너와 분리해서 제공해 주기 위한 기능

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: config-dev
  namespace: default
data:
  DB_URL: localhost
  DB_USER: myuser
  DB_PASS: mypass
  DEBUG_INFO: debug
```

ConfigMap 바인드시

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: configapp
  labels:
    app: configapp
spec:
  replicas: 1
  selector:
    matchLabels:
      app: configapp
  template:
    metadata:
      labels:
        app: configapp
    spec:
      containers:
      - name: testapp
        image: arisu1000/simple-container-app:latest
        ports:
        - containerPort: 8080
        env:
          - name: DEBUG_LEVEL
            valueFrom:
              configMapKeyRef:
                 name: config-dev
                 key: DEBUG_INFO
---
#ConfigMap 전체를 반영할 경우
    spec:
      containers:
      - name: testapp
        image: arisu1000/simple-container-app:latest
        ports:
        - containerPort: 8080
        envFrom:
        - configMapRef:
            name: config-dev
```

파일 마운트 방식

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: spark-config
data:
  spark-defaults.conf: |
    spark.master k8s://https://10.88.22.201:26443
    spark.deploy.mode client
    spark.kubernetes.container.image  localhost:5005/spark:rev-01
    spark.kubernetes.namespace  spark-exec
    spark.executor.memory 8g
    spark.executor.cores  4
    spark.jars.packages  org.apache.spark:spark-sql-kafka-0-10_2.12:3.1.2,org.apache.ignite:ignite-core:2.15.0,org.postgresql:postgresql:42.6.0
    spark.driver.host  172.21.89.220
    spark.eventLog.enabled true
    spark.eventLog.dir file:///opt/data/spark/eventLog
    spark.history.fs.logDirectory file:///opt/data/spark/eventLog
---
apiVersion: v1
kind: Pod
metadata:
  name: spark-client
  labels:
    app: spark-client
spec:
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
  - name: spark-client
    image: localhost:5005/dt-client:latest
    command: ['tail', '-f', '/dev/null']
    volumeMounts:
    - name: spark-data
      mountPath: /opt/data
    - name: spark-config
      mountPath: /root/spark/spark-3.3.2-bin-hadoop3/conf

```
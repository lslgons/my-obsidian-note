# Spark History Server

Spark를 실행할 때 발생되는 로그들을 적재하여 Spark UI에서 확인이 필요한 경우에 사용

독립실행 시 Spark-Client, Airflow에 의해서 실행할 경우 내장된 Spark 바이너리에 의해 실행되며 이때 로그에 대한 저장소를 별도 지정하여 확인할 수 있도록 하여야 함

기본적으로 Spark는 HDFS를 경우하여 저장하게 되어있음

오브젝트 스토리지에 접근하는 경우에도 Hadoop을 경우하여 연동하게 되어있어 결국 Hadoop에 대한 실행이 필요하게 됨

우선 오브젝트 스토리지가 아닌 파일시스템 접근으로 설정 전환

로그를 적재하는 경로는 볼륨설정으로 Share할수 있도록 함

```yaml
# Configmap
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
    spark.driver.host  spark-client.spark.pod.cluster.local
    spark.eventLog.enabled true
    spark.eventLog.dir file:///opt/data/spark/eventLog
    spark.history.fs.logDirectory file:///opt/data/spark/eventLog
---
# Pod
apiVersion: v1
kind: Pod
metadata:
  name: spark-history
  labels:
    app: spark-history
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
  - name: spark-history
    image: localhost:5005/dt-client:latest
    # command: ['/root/spark/spark-3.3.2-bin-hadoop3/sbin/start-history-server.sh', '&&', 'tail', '-f', '/dev/null']
    command: ['tail', '-f', '/dev/null']
    volumeMounts:
    - name: spark-data
      mountPath: /opt/data
    - name: spark-config
      mountPath: /root/spark/spark-3.3.2-bin-hadoop3/conf
  hostNetwork: false
  dnsPolicy: Default
---
# Service
apiVersion: v1
kind: Service
metadata:
  name: spark-history-service
spec:
  type: ClusterIP
  ports:
  - port: 18080
    targetPort: 18080
  selector:
    app: spark-history
---
# Ingress
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: spark-ingress
  annotations:
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
    - host: "spark.datahub.cjoshopping.com"
      http:
        paths:
          - pathType: Prefix
            path: /
            backend:
              service:
                name: spark-history-service
                port: 
                  number: 18080
```
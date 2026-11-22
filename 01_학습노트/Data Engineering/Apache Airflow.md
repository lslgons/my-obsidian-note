# Apache Airflow

### Helm Chart

- airflow helm chart 버전 **8.6.1 이상** 필요
- helm char git repo: [https://github.com/airflow-helm/charts/tree/main/charts/airflow](https://github.com/airflow-helm/charts/tree/main/charts/airflow)
- kubernetes executor configuration: [https://airflow.apache.org/docs/apache-airflow/stable/configurations-ref.html#kubernetes-executor](https://airflow.apache.org/docs/apache-airflow/stable/configurations-ref.html#kubernetes-executor)
- helm chart를 이용한 설치/삭제

```bash
## install using helm 3
helm install \
  airflow \
  airflow-stable/airflow \
  --namespace airflow \
  --version "8.6.1" \
  --values ./custom-values.yaml
#Always pin the --version so you don't unexpectedly update chart versions!
  
## wait until the above command returns and resources become ready 
## (may take a while)

## uninstall the chart
helm uninstall \
  airflow \
  --namespace airflow

## upgrade the chart
helm upgrade \
  --install airflow \
  airflow-stable/airflow \
  --namespace airflow \
  --version "8.6.1" \
  --values custom-values.yaml
```

## Tips

### 부가기능 변경

[https://airflow.apache.org/docs/apache-airflow/stable/howto/index.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/index.html)

### Teams Alert 구현

[https://airflow.apache.org/docs/apache-airflow/stable/howto/notifications.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/notifications.html)

BaseNotifier를 상속받아 notifier.py 파일로 관리하도록 함

### MinIO를 경유한 Dag Dependencies 처리

[https://airflow.apache.org/docs/apache-airflow/stable/howto/operator/external_task_sensor.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/operator/external_task_sensor.html)

User-Defined Operator와 Sensor를 통해 구현된 부분을 ExternalTaskSensor와 ExternalTaskMarker 컴포넌트로 대체

### Daily 수행 Dag에서 특정 요일만 수행

[https://airflow.apache.org/docs/apache-airflow/stable/howto/operator/weekday.html#dayofweeksensor](https://airflow.apache.org/docs/apache-airflow/stable/howto/operator/weekday.html#dayofweeksensor)

SQLBranchOperator나 PythonOperator로 직접 구현된 부분을 BranchDayOfWeekOperator나 DayOfWeekSensor로 대체

---

### Customizing DAG Scheduling with Timetables (난이도 上)

[https://airflow.apache.org/docs/apache-airflow/stable/howto/timetable.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/timetable.html)

cron방식 스케쥴 외 특정일자 및 패턴에 의한 스케줄링 구현방안 (예: 추천모델 스케줄은 매월 두번째주 금요일에 수행)

### Customize view of Apache from Airflow web UI (난이도 上上)

[https://airflow.apache.org/docs/apache-airflow/stable/howto/custom-view-plugin.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/custom-view-plugin.html)

플러그인을 통해 Custom UI 구현 및 적용이 가능

### Listener Plugin of Airflow (난이도 中)

[https://airflow.apache.org/docs/apache-airflow/stable/howto/listener-plugin.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/listener-plugin.html)

dag이나 task 수행의 상태를 감지하여 필요한 작업을 수행할 수 있음

eg.) task를 수행하고 완료됨을 감지하여 DW에 적재

### Customizing the UI

[https://airflow.apache.org/docs/apache-airflow/stable/howto/customize-ui.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/customize-ui.html)

기본 Airflow Web에 테마 변경이나 타이틀 문구 수정

### Creating a custom Operator (중요, 난이도 中)

[https://airflow.apache.org/docs/apache-airflow/stable/howto/custom-operator.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/custom-operator.html)

Operator 개발 가이드

### Managing Connections

[https://airflow.apache.org/docs/apache-airflow/stable/howto/connection.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/connection.html)

Connection의 경우 시스템 환경변수, Secret Backend, DB직접 삽입 방법으로 관리 가능

### Managing Variables

[https://airflow.apache.org/docs/apache-airflow/stable/howto/variable.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/variable.html)

Airflow에서도 직접 변수를 정의하여 코드에서 사용가능함

### 기타

Plugin이나 Operator 개발 후 Packaging 및 모듈 관리 : [https://airflow.apache.org/docs/apache-airflow/stable/administration-and-deployment/modules_management.html](https://airflow.apache.org/docs/apache-airflow/stable/administration-and-deployment/modules_management.html)

## SparkSubmitOperator

TBD
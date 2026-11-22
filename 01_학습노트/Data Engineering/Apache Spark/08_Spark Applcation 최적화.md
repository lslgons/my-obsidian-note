# Spark Applcation 최적화

# 데이터 셋 및 데이터프레임을 위한 메모리 관리

스파크 텅스텐 방식의 데이터와 JVM 오브젝트 형식 간의 변환이 빈번하게 일어남 ← Overhead

노드 간 분산처리를 위해서는 네트워크를 통해 데이터가 오가게 되며 이에 대한 직렬화와 역직렬화 필요 (SerDe; Serialization and Deserialization)

SerDe는 데이터 셋에 대한 시간적 측면의 비용이 발생

- 람다 함수 방식의 처리는 반복적 SerDe비용을 발생
- DSL(Domain Specific Language)나 SQL형식을 활용하여야 함

```scala
// 람다함수처리 형식
df
.filter( x=> x.birthDate.split("-")(0).toInt > earliestYear)
.filter( x=> $"salary">80000 )
.filter( x=> x.lastName.startWith("J"))
.filter( $"firstName".startWith("D"))
.count()

//DSL빙식 처리
df
.filter(year($"birthDate")>earliestYear)
.filter($"salary">80000)
.filter($"lastName".startsWith("J"))
.filter($"firstName".startsWith("D"))
.count()
```

# 스파크 최적화 및 튜닝

## 스파크 설정 및 세팅

$SPARK_HOME 디렉토리의 설정파일 조정 : template파일들을 설정파일들로 전환하여 변경

- conf/spark-defaults.conf.template
- conf/log4j.properties.template
- conf/spark.env-sh.template

스파크 어플리케이션을 제출할 때 --conf 옵션을 통해서 직접 설정 가능

```bash
spark-submit --conf spark.sql.shuffle.partitions=5 --conf "spark.executor.memory=2g"
 --classmain.scala.chaper....jar
```

## 대규모 워크로드를 위한 스파크 규모 확장

스파크 드라이버 : 클러스터에 Executor들을 띄우고 그 위에 돌아갈 Spark Task들을 스케줄링 하는 역할을 수행

### 동적 자원할당

방대한 작업량과 제한된 자원으로 인한 병목현상 방지, 거대한 워크로드가 요청될때 마다 컴퓨팅 자원을 더 할당하거나 줄이도록 요청이 가능

```bash
spark.dynamicAllocation.enabled true
spark.dynamicAllocation.minExecutors 2
spark.dynamicAllocation.schedulerBacklogTimeout 1m
spark.dynamicAllocation..maxExecutors 20
spark.dynamicAllocation.executorIdleTimeout 2min
```

### executor의 메모리와 셔플설정

spark.executor.memory에 의해 제어

- 실행메모리 60% : 셔플, 조인, 정렬, 집계등에 사용
- 저장메모리 40% : 사용자 데이터 구조 캐싱하고 df에서 온 파티션들을 저장
- 예비메모리 300mb

### 데이터 캐싱

허용하는 메모리 수준만큼 스파크 executor들의 메모리에 읽은 파티션을 최대한 저장

```scala
val df=spark.range(1*1000000).toDF("id").withColumn("square",$"id"*$"id")
df.cache()
df.count() //캐시수행
df.count() //캐시사용
```

### persist()

StorageLevel을 통해 데이터간에 어떤 방식으로 캐시될 것인지 제어

캐시를 쓰는 일반적인 경우는 큰 데이터세트에 쿼리나 트랜스포메이션으로 반복적으로 접근해야 하는 시나리오에서 활용

```python
from pyspark import StorageLevel
from pyspark.sql import SparkSession
from pyspark.sql.functions import col
spark=SparkSession.builder.appName("test").getOrCreate()

df=spark.range(1*10000000).toDF("id").withColumn("square", col("id")*col("id"))

from datetime import datetime

df.persist(StorageLevel.DISK_ONLY) #데이터를 직렬화하여 디스크에 저장
start=datetime.now().timestamp()
print(df.count())
print(datetime.now().timestamp()-start)
start=datetime.now().timestamp()
print(df.count())
print(datetime.now().timestamp()-start)

# DISK_ONLY 미적용시 : 2.28 / 0.077
# DISK_ONLY 적용시 : 2.79 / 0.162
```

## 스파크 조인

### Broadcast Hash Join (BHJ)

Map-side-only Join 이라고도 하며 한쪽은 작고 다른쪽은 큰 두 종류의 데이터를 사용하여 특정컬럼 기준으로 조인

작은쪽 데이터가 10MB이하일때 사용되며 “spark.sql.authBroadcastJoinThreashold”에 의해 지정

- 양쪽 데이터셋의 각 키가 스파크에서 동일한 파티션 안에 해시될 때
- 한쪽 데이터가 다른쪽보다 많이 작은 경우

```python
from pyspark.sql.functions import broadcast
joinDF=playersDF.join(broadcast(clubsDF), "key1===key2")
```

### Shuffle Sort Merge Join

정렬 가능하고 겹치지 않으면서 공통 파티션에 저장 가능한 공통 키를 기반으로 큰 두 종류의 데이터셋을 합치는 경우 사용

각 데이터 셋의 동일 키를 가진 것들의 모든 레코드가 동일한 Executor의 동일 파티션에 존재하는 경우

“spark.sql.join.preferSortMergeJoin” 설정에 의해 활성화
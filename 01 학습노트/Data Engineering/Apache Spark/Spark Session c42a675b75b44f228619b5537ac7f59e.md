# Spark Session

모든 스파크 연산과 데이터에 대한 통합 연결채널

SparkApplication의 시작코드…

```scala
import org.apache.spark.sql.SparkSession

val spark= SparkSession.builder
		.appName("AppName")
		.config("spark.sql.shuffle.partitions", 6)
		.getOrCreate()
```
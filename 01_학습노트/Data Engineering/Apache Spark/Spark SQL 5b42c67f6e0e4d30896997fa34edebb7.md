# Spark SQL

```python
csv_file="departuredelays.csv"

#임시뷰 생성
df=(
    spark.read.format("csv")
    .option("inferSchema", True)
    .option("header", True)
    .load(csv_file)
)
df.createOrReplaceTempView("us_delay_flights_tbl")
spark.sql('''
	SELECT distance, origin, destination 
	FROM us_delay_flights_tbl 
	WHERE distance > 1000 
	ORDER BY distance DESC
''')
.show(10, truncate=False)
```

각 테이블과 해당 데이터에 관련된 정보인 스키마, 설명, 테이블명, 데이터베이스명, 칼럼명 등의 메타데이터를 가지고 있고 중앙 메타스토어에 저장

/user/hive/warehouse에 있는 아파치 하이브 메타스토어를 사용하여 메타데이터를 유지

관리형 테이블 : 메타데이터와 파일 저장소의 데이터를 모두 관리, FS/HDFS/S3 등

비관리형 테이블 : 메타데이터만 관리하고 카산드라 같은 외부 데이터 소스에서 데이터를 직접 관리

```python
#관리형 테이블
spark.sql("CREATE DATABASE learn_spark_db")
spark.sql("USE learn_spark_db")
schema="date STRING, delay INT, distance INT, origin STRING, destination STRING"
spark.sql("CREATE TABLE managed_us_delay_flights_tbl ({})".format(schema))
df.write.saveAsTable("managed_us_delay_flights_tbl")
```

```python
#비관리형 테이블 : 데이터 저장없이 메타데이터만으로 테이블 생성
spark.sql('''
    CREATE TABLE us_delay_flights_tbl (
        date STRING,
        delay INT,
        distance INT,
        origin STRING,
        destination STRING
    ) USING csv OPTIONS (PATH '/home/user/spark_code/departuredelays.csv')
''')
```

메타데이터 보기

```python
spark.catalog.listDatabases()
spark.catalog.listTables()
spark.catalog.listColumns("us_delay_flights_tbl")
```

SQL 테이블 캐싱하기 : Spark 3.0부터 LAZY를 지정하여 테이블을 바로 캐싱하지 않고 처음 사용되는 시점에 캐싱

```python
CACHE [LAZY] TABLE <table-name>
UNCACHE TABLE <table-name>
```

UDF(User Defined Function)

```bash
from pyspark.sql.types import LongType
def cubed(s):
    return s*s*s
spark.udf.register("cubed", cubed, LongType())
spark.range(1,9).createOrReaplaceTempView("udf_test")
spark.sql("SELECT id, cubed(id) AS id_cubed FROM udf_test").show()
```

스파크 SQL에서의 평가순서 미보장 : 하위표현식의 평가순서를 보장하지 않음

```bash
# IS NOT NULL이 먼저 표현식 우선순위 보장이 안됨
spark.sql("SELECT s FROM test1 WHERE s IS NOT NULL AND strlen(s) >1")
```

Pandas UDF : pyspark에서의 UDF성능 지연문제를 해결하기 위해 도입

```bash
import pandas as pd
#Import various pyspark SQL functions including pandas_udf
from pyspark.sql.functions import col, pandas_udf
from pyspark.sql.types import LongType

#큐브함수 선언
def cubed(a:pd.Series) -> pd.Series:
    return a*a*a
#큐브함수에 대한 pandas UDF 선언
cubed_udf=pandas_udf(cubed, returnType=LongType())

df=spark.range(1,4)
df.select("id", cubed_udf(col("id"))).show()
```

### JDBC

다른 DB에서 데이터를 읽을 수 있는 데이터 소스 API가 포함, 결과를 데이터 프레임으로 반환

JDBC데이터 소스에 대한 드라이버를 지정해야 하며 스파크 클래스 경로에 있어야 함

```bash
./bin/spark-shell --driver-class-path $database.jar --jars $database.jar
```

파티셔닝의 중요성 : 모든 데이터가 하나의 드라이버 연결을 통해 처리되므로 소스 시스템의 리소스를 포화상태로 만들 수 있어 partition 속성을 지정해야 함

- numPartitions : 병렬처리를 위한 최대 파티션 수, 최대 연결 JDBC수
- partitionColumn : 파티션을 결정하는데 사용되는 컬럼, 숫자, 날짜, 타임스탬프 컬럼이여야 함
- lowerBound : 파티션 크기에 대한 파티션 열의 최소값
- upperBound : 파티션 크기에 대한 파티션 열의 최대

### 고차함수

SQL내부에서 익명람다함수를 인수로 사용하는 내장함수

```python
# 입력값의 각 요소마다 function을 적용하여 출력
# ex) transform(celsius, t -> (((t * 9) div 5) + 32 )) AS fahrenheit
transform(array<T>, function<T,U>): array<U>
# 입력값의 요소 중 참인 요소만으로 구성된 배열을 출력, ex) filter(celsius, t->t>38) AS high
filter(array<T>, function<T,Boolean>): array<T>
# 입력한 배열의 요소 중 Boolean함수를 만족시키것이 존재하면 참을 반환
# ex) exists(celsius, t->t=38) as threshold
exists(array<T>, function<T,V,Boolean>): Boolean
# 익명함수를 사용하여 요소를 버퍼 B에 병합하고 최종버퍼에 마무리 funtion<B,R>을 적용하여 배열의 요소를 단일값을 줄임
reduce(array<T>, B, function<B,T,B>, function<B,R>)
spark.sql("""
    SELECT
        celsius,
        reduce(
            celsius,
            0,
            (t, acc) -> t+acc,
            acc-> (acc div size(celsius) * 9 div 5) + 32
        ) as avgFahrenheit
    FROM
        tC
""").show(truncate=False)
```
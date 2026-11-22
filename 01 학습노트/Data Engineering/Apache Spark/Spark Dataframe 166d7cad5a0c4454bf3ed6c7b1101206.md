# Spark Dataframe

### Pandas DF vs. Spark DF

![Untitled](Spark%20Dataframe/Untitled.png)

주요 차이점

- Spark DataFrame은 SQL연산과 비슷한 연산자를 제공
- Spark DF의 연산은 대부분 새로운 DF객체를 반환하는 형태로 구성
- Spark DF객체에 직접 수정을 허용하지 않음
- Spark DF은 []연산자 활용이 제한적
    
    ![Untitled](Spark%20Dataframe/Untitled%201.png)
    
- Spark DF은 여러컬럼을 access하기위해 리스트[]를 이용하지 않음
    
    ![Untitled](Spark%20Dataframe/Untitled%202.png)
    

### Pandas DF ↔ Spark DF

![Untitled](Spark%20Dataframe/Untitled%203.png)

# 스키마

데이터프레임을 위해 칼럼 이름과 연관된 데이터 타입을 정의한 것

- 데이터 타입을 추측해야 하는 부담을 덜어줌
- 스키마 확정을 위한 Fullscan이 필요없음
- 스키마가 맞지 않을경우 조기에 발견할 수 있음

```python
# DDL
schema="Id INT, First STRING, Last String, Url STRING, Published STRING, Hits INT, Campaigns ARRAY<STRING>"

# DataStruct
schema=StructType([
        StructField("Id", IntegerType(), False),
        StructField("First", StringType(), False),
        StructField("Last", StringType(), False),
        StructField("Url", StringType(), False),
        StructField("Published", StringType(), False),
        StructField("Hits", IntegerType(), False),
        StructField("Campaigns", ArrayType(StringType(), False), False),
    ])
```

# 구성요소

## 컬럼 (Column)

컬럼들의 이름으로 나열해 볼 수도 있고, 관계형 표현이나 계산식 형태의 표현식으로 연산을 수행, 공개 메소드를 가진 객체로 표현

```python
blog_df=spark.read.schema(schema).json(jsonFile)
blog_df.show(truncate=False)
print(blog_df.printSchema())

blog_df.select(expr("Hits * 2")).show(2)
blog_df.select(col("Hits")*2).show(2) #col을 사용한 계산
blog_df.withColumn("Big Hitters", (expr("Hits > 10000"))).show()
```

## 로우 (Row)

하나의 행은 일반적으로 하나 이상의 칼럼을 갖고있는 로우객체로 표현

```python
from pyspark.sql import Row
blog_row=Row(6,"Reynold", "Xin", "https://tinyurl.6", 255568, 
		"3/2/2015", Array("twitter", "LinkedIn"))
```

# 자주 쓰이는 DF 작업들

## DataFrameReader

데이터 소스에서 데이터 프레임으로 데이터를 읽기위한 핵심구조

SparkSession인스턴스를 통해서만 액세스가 가능함

format지원은 parquet, csv, txt, json, jdbc, orc, avro 등

```python
#DataFrameReader.format(args).option("key","value").schema(args).load()
# spark.read / spark.readStream
df=spark.read.csv(n=20, truncate=False)

```

## DataFrameWriter

지정된 내장 데이터 소스에 데이터를 저장하거나 쓰는 작업을 수행

SparkSession이 아닌 저장하려는 데이터프레임에서 인스턴스에 액세스가 가능

```python
# DataFrameWriter.format(args).option(args).bucketBy(args).partitionBy(args).save(args)
# DataFrameWriter.format(args).option(args).sortBy(args).saveAsTitle(table)
df.write.format("parquet").save(some_of_path)
```

# Trasformation / Action

## Projection & Filter

필터를 이용해 특정 관계 상태와 매치되는 행들만 되돌려주는 방법

```python
few_fire_df=(
    fire_df
    .select("IncidentNumber", "AvailableDtTm", "CallType")
    .where(col("CallType") != "Medical Incident")
)
call_type=(
    fire_df
    .select("CallType")
    .where(col("CallType".isNotNull())
    .agg(countDistint("CallType").alias("DistinctCallTypes"))
    .show()
)
```

### select()

![Untitled](Spark%20Dataframe/Untitled%204.png)

### filter()

![Untitled](Spark%20Dataframe/Untitled%205.png)

## 집계연산

### orderBy()

![Untitled](Spark%20Dataframe/Untitled%206.png)

### groupBy()

- pandas DataFrame의 groupby(by='group_by_컬럼명') 수행 시 group_by_컬럼명 레벨로 group by 된 DataFrameGroupBy 객체 반환하고 여기에 aggregation 메소드 적용.
- spark DataFrame도 groupBy('group_by_컬럼명') 수행 시 group_by_컬럼명 레벨로 <u>**group by 된 GroupedData 객체 반환하고 여기에 aggregation 메소드 적용.**</u>
- pandas DataFrameGroupBy 객체에 agg() 메소드를 이용하여 서로 다른 컬럼에 서로 다른 aggregation 함수 적용 가능
- spark GroupedData 객체도 <u>**agg() 메소드를 이용하여 서로 다른 컬럼에 서로 다른 aggregation 함수 적용 가능**</u>
- spark groupBy()는 pandas groupby()의 특징과 SQL의 특징을 함께 가짐.

### Spark DF aggregation

- pandas DataFrame은 DataFrame 객체에서 aggregation 메소드를 많이 가질 수 있음(DataFrame.count(), DataFrame.max())
- pandas DataFrame은 DataFrame 객체에 aggregation 메소드를 적용 시 DataFrame에 속한 전체 컬럼들에 모두 aggregation 메소드를 적용
- spark DataFrame은 DataFrame 객체에서 aggregation 메소드를 별로 가지고 있지 않음. count() 메소드 정도...
- spark DataFrame에 aggregation 메소드를 적용 시에는 pyspark.sql.functions 모듈의 max, min, sum 등의 함수를 이용해야함.

```python
import pyspark.sql.functions as F
(
    fire_ts_df
    .select(F.sum("NumAlarms"), F.avg("ResponseDelayedinMins"), 
			F.min("ResponseDelayedinMins"), F.max("ResponseDelayedinMins"))
)
```

## 컬럼의 이름변경 및 추가삭제

StructField를 써서 스키마 내에서 원하는 컬럼 이름들을 지정하여 변경 가능

```python
new_fire_df = fire_df.withColumnRenamed("Delay", "ResponseDelayedinMins")
(
    new_fire_df
    .select("ResponseDelayedinMins")
    .where(col("ResponseDelayedinMins")>5)
    .show(5,False)
)
```

타입변환

```python
fire_ts_df = (
    fire_df
    .withColumn("IncidentDate", to_timestamp(col("CallDate"), "MM/dd/yyyy"))
    .drop("CallDate")
    .withColumn("OnWatchDate", to_timestamp(col("WatchDate"), "MM/dd/yyyy"))
    .drop("WatchDate")
    .withColumn("AvailableDtTS", to_timestamp(col("AvailableDtTm"), "MM/dd/yyyy hh:mm:ss a"))
    .drop("AvailableDtTm")
)
```

# 데이터 세트

엄격하게 타입이 정해진 JVM객체의 집합, 자바에서는 클래스라고 볼 수 있음

데이터세트는 자바와 스칼라에서 통용되며, 파이썬과 R에서는 데이터프레임만 사용이 가
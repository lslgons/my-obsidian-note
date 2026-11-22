# Spark Structured Streaming

[https://spark.apache.org/docs/latest/structured-streaming-programming-guide.html](https://spark.apache.org/docs/latest/structured-streaming-programming-guide.html)

```scala
import org.apache.spark.sql.functions._
val lines = spark.readStream.format("socket")
		.option("host","localhost").option("port", 9999)
		.load()

val words = lines.select(split(col("value"), "\\s").as("word"))
val counts = words.groupBy("word").count()

val writer = counts.writeStream.format("console").outputMode("complete")

//처리 세부사항 지정
//체크포인트는 스트리밍 쿼리가 진행 중인 상황에서 저장할 수 있는 디렉터리를 의미
//실패 시 마지막 지점에서 실행시킬 수 있는 용도로 사용
import org.apache.spark.sql.streaming._
val writer2 = writer.trigger(Trigger.ProcessingTime("1 second"))
		.option("checkpointLocation", "path/to/checkpoint")

val streamingQuery = writer2.start()

//쿼리 수행이 끝날때까지 스레드 블로킹을 위함
streamingQuery.awaitTermination()
```

```python
from pyspark.sql.functions import col, from_json, sequence
from pyspark.sql.types import *
  
'''
PV Counter의 기준
상품상세 페이지 (일반상품, 팩상품)에 유입되는 고객의 수
CID기준으로 status 저장하여야 함
일반상품       : https://display.cjonstyle.com/m/item/65085968?channelCode=50001002
팩상품         : https://display.cjonstyle.com/m/mocode/M1134947
팩상품(레이어) : https://display.cjonstyle.com/m/mocode/M1134947
  
'''
  
schema=StructType([
    StructField("std_udttm", StringType(), False),
    StructField("cd1", StringType(), False), #CID
    StructField("cd6", StringType(), False), #고객번호
    StructField("dl", StringType(), False), #현재 사이트주소
    StructField("dr", StringType(), False), #이전 사이트주소
    StructField("cd7", StringType(), False), #고객성별 (M/F)
    StructField("cd8", StringType(), False), #고객나이
    StructField("cd9", StringType(), False), #고객등급 (SEG. Code)
    StructField("cd36", StringType(), False), #방문유입채널
    StructField("cd94", StringType(), False), #제휴유입코드
    StructField("cd96", StringType(), False), #제휴유입데이터
    StructField("cd70", StringType(), False), #상품코드
    StructField("cd71", StringType(), False), #상품명
    StructField("cd94", StringType(), False), #INFL_CODE, 제휴유입코드
    StructField("cd96", StringType(), False), #캠페인ID
    StructField("cd123", StringType(), False), #팩상품코드 : 일반 상품의 경우 '없음'
    StructField("t", StringType(), False), #조회 유형 event or pageview
    ])
from pyignite import Client
ignite_client = Client()
# ignite_client.connect("ignite.dt-work.svc.cluter.local", 10800)
ignite_client.connect("172.21.89.252", 10800)
  
# TODO 등록되는 시간이 utc 기준이므로 출력할때는 kst로 변환시켜야 함 (hours+9)
def proc_data(df, epoch_id):
    print('epoch: ', epoch_id, ", Length: ", df.count())
    df2=df.withColumn("jsonData", from_json(col("value"), schema)).select("jsonData.*")
    # 상품상세 진입 로그
    df3=df2.filter(col("cd70").isNotNull()).filter(col("t")=="pageview").select('cd1','cd6','cd70','cd71', 'cd36', 'cd94', 'cd96', 'cd123', 't') \
        .withColumnRenamed('cd1', 'CID') \
        .withColumnRenamed('cd6', 'CUST_NO') \
        .withColumnRenamed('cd70', 'ITEM_CODE') \
        .withColumnRenamed('cd71', 'ITEM_NAME') \
        .withColumnRenamed('cd123', 'MO_CODE') \
        .withColumnRenamed('cd36', 'CHANNEL') \
        .withColumnRenamed('cd94', 'INFL_CODE') \
        .withColumnRenamed('cd96', 'INFL_DATA') \
        .withColumnRenamed('t', 'EVENT_TYPE')
    df_collected=df3.collect()
    for log in df_collected:
        #Row Type을 dict로 변환
        log_dict=log.asDict()
        # 상품상세 유입을 감지하여 INSERT 시도
        try:
            insert_sql="INSERT INTO CUSTOMS_ENTRY_LIST (CLIENT_ID, ITEM_CODE, ITEM_NAME, CUST_NO, CHANNEL, INFL_CODE, INFL_DATA, ENTRY_DTM) VALUES ('{0}', '{1}', '{2}', '{3}', '{4}', '{5}', '{6}', CURRENT_TIMESTAMP())".format(log_dict["CID"], log_dict["ITEM_CODE"], log_dict["ITEM_NAME"], log_dict["CUST_NO"], log_dict["CHANNEL"], log_dict["INFL_CODE"], log_dict["INFL_DATA"])
            # print(insert_sql)
            ignite_client.sql(insert_sql)
            # print("INSERT : {0}: {1}".format(log_dict["CID"], log_dict["ITEM_CODE"]))
        except:
            # 복합키 기준으로 이미 값을 가지고 있음 UPDATE수행 함
            update_sql="UPDATE CUSTOMS_ENTRY_LIST SET ENTRY_DTM=CURRENT_TIMESTAMP() WHERE ITEM_CODE='{0}'AND CLIENT_ID='{1}';".format(log_dict["ITEM_CODE"], log_dict["CID"])
            ignite_client.sql(update_sql)
            # print("UPDATE : {0}: {1}".format(log_dict["CID"], log_dict["ITEM_CODE"]))
    # 상품상세 로그가 아닌 로그들
    df4=df2.filter(col("cd70").isNull()).filter(col("t")=="pageview").select('cd1', 'cd6', 'cd70', 'cd123', 'cd71') \
        .withColumnRenamed('cd1', 'CID') \
        .withColumnRenamed('cd6', 'CUST_NO') \
        .withColumnRenamed('cd70', 'ITEM_CODE') \
        .withColumnRenamed('cd123', 'MO_CODE') \
        .withColumnRenamed('cd71', 'ITEM_NAME') \
        .withColumnRenamed('t', 'EVENT_TYPE')
    others_collected=df4.collect()
    # 감지대상 CID를 기준으로 이탈
    for log in others_collected:
        log_dict=log.asDict()
        cursor=ignite_client.sql("SELECT * FROM CUSTOMS_ENTRY_LIST WHERE CLIENT_ID='{0}' ORDER BY ENTRY_DTM ASC".format(log_dict["CID"]))
        try:
            # print("EXIT Prod. description : ", log_dict)
            qu=list(cursor)
            # print("Deleted CID : ", qu)
            for entry in qu:
                # 로그 대상 감지로 이탈 체크함
                print(entry)
                insert_query="INSERT INTO CUSTOMS_VISIT_HISTORY (ITEM_CODE, ITEM_NAME, CLIENT_ID, CUST_NO, CHANNEL, INFL_CODE, INFL_DATA, ENTRY_DTM, EXIT_DTM) VALUES ('{0}','{1}','{2}','{3}','{4}','{5}','{6}', PARSEDATETIME('{7}', 'yyyyMMddHHmmss'), CURRENT_TIMESTAMP())".format(entry[1], entry[2], entry[0], entry[3], entry[4], entry[5], entry[6], entry[7][0].strftime("%Y%m%d%H%M%S"))               
                ignite_client.sql(insert_query)
                # 대상목록에서 제외
                delete_query="DELETE FROM CUSTOMS_ENTRY_LIST WHERE CLIENT_ID='{0}' AND ITEM_CODE='{1}'".format(entry[0], entry[1])
                ignite_client.sql(delete_query)
                print("=================>>", insert_query)
                print("=================>>", delete_query)
        except:
            # 값이 없으므로 패스
            pass
  
df=spark \
    .readStream.format("kafka") \
    .option("kafka.bootstrap.servers", "common-kafka-0.common-kafka-headless.common-app.svc.cluster.local:9092")    \
    .option("subscribe", "DT_WORK_01") \
    .option("startingOffsets","latest") \
    .load()
   
query=df \
    .selectExpr("CAST(value AS STRING)") \
    .writeStream \
    .foreachBatch(proc_data) \
    .start()
 
  
# .outputMode("append").format("console").option("truncate", "false") \
# .foreachBatch(show_data) \
  
query.awaitTermination()
```
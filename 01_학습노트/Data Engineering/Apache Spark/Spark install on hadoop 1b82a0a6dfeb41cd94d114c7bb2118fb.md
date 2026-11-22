# Spark install on hadoop

[Spark 설치하기 | Reinventing the Wheel (heekangpark.github.io)](https://heekangpark.github.io/hadoop/spark-installation)

스파크 다운로드

```bash
wget https://dlcdn.apache.org/spark/spark-3.3.2/spark-3.3.2-bin-hadoop3.tgz
sudo tar -xzf spark-3.3.2-bin-hadoop3.tgz -C /usr/local
# sudo mv spark-3.3.2-bin-hadoop3 /usr/local
sudo ln -s /usr/local/spark-3.3.2-bin-hadoop3 /usr/local/spark
rm spark-3.3.2-bin-hadoop3.tgz
```

환경변수 추가

```bash
echo 'export SPARK_HOME=/usr/local/spark' >> ~/.bashrc
echo 'export PATH=$SPARK_HOME/bin:$SPARK_HOME/sbin:$PATH' >> ~/.bashrc

source ~/.bashrc
```

conf template 복사

```bash
sudo cp /usr/local/spark/conf/workers.template /usr/local/spark/conf/workers
sudo cp /usr/local/spark/conf/spark-defaults.conf.template /usr/local/spark/conf/spark-defaults.conf
sudo cp /usr/local/spark/conf/log4j2.properties.template /usr/local/spark/conf/log4j2.properties
sudo cp /usr/local/spark/conf/spark-env.sh.template /usr/local/spark/conf/spark-env.sh
```

Spark 환경설정

```bash
#conf/workers
worker1.mshome.net
worker2.mshome.net
worker3.mshome.net

#spark-env.sh
export JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64
export SPARK_MASTER_HOST=master1.mshome.net
export HADOOP_HOME=/usr/local/hadoop
export HADOOP_CONF_DIR=$HADOOP_HOME/etc/hadoop

#spark-defaults.conf
spark.master                     spark://master1.mshome.net:7077
spark.eventLog.enabled           true
spark.eventLog.dir               hdfs://master1.mshome.net:9000/spark
spark.serializer                 org.apache.spark.serializer.KryoSerializer
spark.driver.memory              5g
spark.executor.extraJavaOptions  -XX:+PrintGCDetails -Dkey=value -Dnumbers="one two three"
```

이벤트 로그 저장 디렉토리

```bash
mkdir -p $SPARK_HOME/eventLog
```

pyspark 설치

```bash
pip install findspark pyspark
```

스파크 실행

마스터노드에서만 실행

```bash
$HADOOP_HOME/sbin/start-dfs.sh
$HADOOP_HOME/sbin/start-yarn.sh
**$HADOOP_HOME/bin/mapred --daemon start historyserver**

$SPARK_HOME/sbin/start-all.sh
$SPARK_HOME/sbin/start-history-server.sh
```

스파크 수행 테스트

```bash
wget https://www.gutenberg.org/cache/epub/10/pg10.txt
hdfs dfs -mkdir -p /user/hadoop
hdfs dfs -put pg10.txt /user/hadoop/pg10.txt
```

```python
import findspark
from pyspark.sql import SparkSession

findspark.init()
spark = SparkSession.builder.master("yarn").appName("test1").getOrCreate()
sc = spark.sparkContext
text_file = sc.textFile("/user/hadoop/pg10.txt")

counts = text_file.flatMap(lambda line: line.split(" ")) \
             .map(lambda word: (word, 1)) \
             .reduceByKey(lambda a, b: a + b)\
             .map(lambda x: (x[1], x[0])) \
             .sortByKey(False)

counts.saveAsTextFile("/user/hadoop/spark-wordcount-output")
```

실행결과 확인

```bash
# spark-wordcount-output 디렉토리 탐색
hdfs dfs -ls /user/hadoop/spark-wordcount-output 

# wordcount 결과 파일 확인
hdfs dfs -cat /user/hadoop/spark-wordcount-output/part-00000
hdfs dfs -cat /user/hadoop/spark-wordcount-output/part-00001

# 결과 파일 (로컬로) 다운로드
hdfs dfs -get /user/hadoop/spark-wordcount-output/part-00000
hdfs dfs -get /user/hadoop/spark-wordcount-output/part-00001
```

스파크 종료

```bash
$SPARK_HOME/sbin/stop-all.sh
$SPARK_HOME/sbin/stop-history-server.sh

$HADOOP_HOME/sbin/stop-dfs.sh
$HADOOP_HOME/sbin/stop-yarn.sh
$HADOOP_HOME/bin/mapred --daemon stop historyserver
```
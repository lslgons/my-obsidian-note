# Hive on Spark

Hive 3.0 이상부터는 MapReduce기반의 쿼리엔진 사용을 권장하지 않으며 대체 엔진으로 Spark를 사용하여야 함

[https://cwiki.apache.org/confluence/display/Hive/Hive+on+Spark](https://cwiki.apache.org/confluence/display/Hive/Hive+on+Spark)

[https://cwiki.apache.org/confluence/display/Hive/Hive+on+Spark%3A+Getting+Started](https://cwiki.apache.org/confluence/display/Hive/Hive+on+Spark%3A+Getting+Started)

trouble shoot

[https://stackoverflow.com/questions/56882129/yarn-error-failed-to-create-spark-client-for-spark-session](https://stackoverflow.com/questions/56882129/yarn-error-failed-to-create-spark-client-for-spark-session)

```xml
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<?xml-stylesheet type="text/xsl" href="configuration.xsl"?>
<configuration>
  <property>
       <name>hive.metastore.warehouse.dir</name>
       <value>/user/hive/warehouse</value>
  </property>
  <property>
       <name>hive.cli.print.header</name>
       <value>true</value>
  </property>
  <property>
    <name>hive.metastore.local</name>
    <value>true</value>
    <description>controls whether to connect to remove metastore server or open a new metastore server in Hive Client JVM</description>
  </property>
  <property>
    <name>javax.jdo.option.ConnectionURL</name>
    <value>jdbc:postgresql://localhost:5432/metastore_db</value>
    <description>JDBC connect string for a JDBC metastore</description>
  </property>
  <property>
    <name>javax.jdo.option.ConnectionDriverName</name>
    <value>org.postgresql.Driver</value>
    <description>Driver class name for a JDBC metastore</description>
  </property>
  <property>
    <name>javax.jdo.option.ConnectionUserName</name>
    <value>hadoop</value>
    <description>username to use against metastore database</description>
  </property>
  <property>
    <name>javax.jdo.option.ConnectionPassword</name>
    <value>hadoop</value>
    <description>password to use against metastore database</description>
  </property>
  <property>
    <name>hive.execution.engine</name>
    <value>spark</value>
    <description>
      Expects one of [mr, tez, spark].
      Chooses execution engine. Options are: mr (Map reduce, default), tez, spark. While MR
      remains the default engine for historical reasons, it is itself a historical engine
      and is deprecated in Hive 2 line. It may be removed without further warning.
    </description>
  </property>
  <property>
    <name>spark.master</name>
    <value>master1.mshome.net:7077</value>
  </property>
</configuration>
```
# Apahe Hive

[아파치 하이브란? (Apache Hive) :: Kaden Sungbin Cho (tistory.com)](https://kadensungbincho.tistory.com/65)

하둡환경에서 복잡한 MR코드를 SQL과 유사한 간단한 hiveSQL로 처리하도록 제공

파일 시스템에 저장된 데이터에 catalog와 metastore를 제공하여 논리적 DB, 테이블, 파티션을 제공하여 데이터를 구조화

[https://cwiki.apache.org/confluence/display/Hive/](https://cwiki.apache.org/confluence/display/Hive/)

## Hive 아키텍처

![Untitled](Apahe%20Hive/Untitled.png)

## Hive 데이터모델

데이터베이스 : 테이블의 그룹, 실제 데이터를 포함하지 않음

테이블

- RDB의 테이블에 상응하는 객체, 각 테이블은 해당되는 HDFS 디렉토리를 가짐
- 테이블안의 데이터는 직렬화되어 저장
- 직렬화 포맷설정은 테이블 카탈로그에 설정되어 하이브 쿼리 수행 시 데이터에 접근할 떄 자동적으로 사용
- Hive는 HDFS, NFS, 로컬 디렉토리 등의 저장소를 지원함

파티션

- 각 테이블은 여러 개의 파티션을 가질 수 있음
- 파티션은 테이블 하위의 데이터의 분배와 위치를 결정하게 됨
- 데이터 스캔을 줄여 쿼리를 최적화하는데 큰 역할을 하게 됨

버킷

- 각 파티션에 존재하는 데이터는 한 컬럼에 들어있는 값의 해쉬를 통해 버킷으로 나누어 짐
- CLUSTER BY 문구를 사용하여 한 컬럼에서 같은 값을 가진 행들은 같은 버킷으로 저장
- 파티션과 마찬가지로 데이터 스캔을 줄이는 역할을 수행

뷰(View)

- 논리적 데이터 구조로 쿼리의 복잡함을 줄여 간단하게 만드는 기능을 수행
- HDFS에 저장되지 않으며 metastore에만 정의되어 관

## 사전 환경설정

Postgresql 설치 (보통 클러스터 마지막 서버에 설치)

[https://ko.linux-console.net/?p=2487#gsc.tab=0](https://ko.linux-console.net/?p=2487#gsc.tab=0)

[https://www.howtoforge.com/how-to-install-postgresql-on-rocky-linux/](https://www.howtoforge.com/how-to-install-postgresql-on-rocky-linux/)

```sql
CREATE USER hadoop WITH CREATEDB CREATEROLE PASSWORD 'hadoop';
CREATE SCHEMA metastore_db;
ALTER SCHEMA metastore_db OWNER TO hadoop;
GRANT ALL ON ALL TABLES IN SCHEMA public TO hadoop;
```

# Hive 설치

[https://truman.tistory.com/209](https://truman.tistory.com/209)

```bash
wget https://dlcdn.apache.org/hive/hive-3.1.3/apache-hive-3.1.3-bin.tar.gz
sudo tar xvzf apache-hive-3.1.3-bin.tar.gz -C /usr/local
sudo ln -s /usr/local/apache-hive-3.1.3-bin /usr/local/hive
sudo chown -R hadoop:hadoop /usr/local/apache-hive-3.1.3-bin
```

환경설정

```bash
#.bashrc
export HIVE_HOME=/usr/local/hive
export PATH=$HIVE_HOME/bin:$HIVE_HOME/sbin:$PATH

source .bashrc
```

Hive 실행

사전 HDFS 폴더 생성

```bash
$ $HADOOP_HOME/bin/hadoop fs -mkdir       /tmp
$ $HADOOP_HOME/bin/hadoop fs -mkdir       /hadoop/hive/warehouse
$ $HADOOP_HOME/bin/hadoop fs -chmod g+w   /tmp
$ $HADOOP_HOME/bin/hadoop fs -chmod g+w   /hadoop/hive/warehouse
```

[hive-env.sh](http://hive-env.sh) 편집

```bash
HADOOP_HOME=/usr/local/hadoop
```

hive-site.xml 생성 및 편집

```xml
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<?xml-stylesheet type="text/xsl" href="configuration.xsl" ?>
<configuration>
<property>
  <name>hive.cli.print.header</name>
  <value>true</value>
  <description>Whether to print the names of the columns in query output.</description>
</property>
<property>
  <name>hive.metastore.warehouse.dir</name>
  <value>/user/hive/warehouse</value>
  <description>location of default database for the warehouse</description>
</property>
<property>
  <name>hive.server2.enable.doAs</name>
  <value>false</value> 
</property>
</configuration>
```

SchemaTool

```xml
schematool -dbType derby -initSchema
```

Run HIVE Server

```python
hiveserver2
hive --service hiveserver2
beeline -u jdbc:hive2://master1.mshome.net:10002
```

## 메타 DB설정

Hive 메타데이터는 로컬로 derby를 사용할 수 있으나 세션을 동시에 사용이 어려워 MySQL이나 Postgresql 로 메타 데이터를 관리할 수 있음

```sql
# postgresql 관리자 계정으로 사용자 및 DB생성
create user hive with password 'hive';
create database metastore_db owner=hive;
create schema authorization hive;
```

PostgreSQL jdbc 라이브러리를 hive lib디렉토리에 복사

```sql
cd /usr/local/hive/lib
wget https://jdbc.postgresql.org/download/postgresql-42.5.4.jar
cp postgresql-42.5.4.jar ./
```

hive-site.xml

```xml
<property>
    <name>hive.metastore.local</name>
    <value>true</value>
    <description>controls whether to connect to remove metastore server or open a new metastore server in Hive Client JVM</description>
</property>
<property>
    <name>javax.jdo.option.ConnectionURL</name>
    <value>jdbc:postgresql://node3:5432/metastore_db</value>
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
```

# Hive Server2

원격 클라이언트가 하이브에 쿼리를 실행할 수 있도록 인터페이스를 지원하는 서버

Thrift RPC를 기본으로 하고 있음

JDBC나 ODBC와 같은 인터페이스를 지원하는 것을 강화

```xml

```

[Hive on Spark](Apahe%20Hive/Hive%20on%20Spark%20dbcf9ad2d69e4ad2aee0af32d5ba7a0b.md)
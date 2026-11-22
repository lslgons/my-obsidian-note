# Apache HBase

하둡플랫폼을 위한 공객 비관계형 분산 컬럼기반 데이터베이스

HDFS위에서 동작하며 대량의 데이터를 저장하기 위한 방법을 제공

## 특징

선형 확장성

읽기와 쓰기의 일관성 제공

클러스터를 통한 데이터의 복제 제공

![Untitled](Apache%20HBase/Untitled.png)

## Architecture

![Untitled](Apache%20HBase/Untitled%201.png)

### Master Server

Region을 할당하고 zookeeper의 도움을 받는 작업을 수행

Region서버에 퍼져있는 region을의 로드밸런싱 수행

![Untitled](Apache%20HBase/Untitled%202.png)

### Region Server

- 클라이언트와 통신하고 데이터 관련 연산을 관리
- 내부 region의 읽기와 쓰기 요청을 관리
- Region의 크기는 region크기의 threshold에 의해 결정
- 내부에 메모리 저장소와 HFile을 가지고 있음
- 메모리 저장소는 캐시와 같은 동작을 함

![Untitled](Apache%20HBase/Untitled%203.png)

### Zookeeper

설정 정보를 관리, 분산처리의 동기화 등을 제공해주는 오픈소스 프로젝트

가용성을 추가하여 서버 실패나 네트워크 분할 등을 추적

클라이언트가 region에 통신하려면 zookeeper를 통해야 

## 테이블 구조

![Untitled](Apache%20HBase/Untitled%204.png)

![Untitled](Apache%20HBase/Untitled%205.png)

### Put Method

![Untitled](Apache%20HBase/Untitled%206.png)

### Get Method

![Untitled](Apache%20HBase/Untitled%207.png)

### Delete Method

![Untitled](Apache%20HBase/Untitled%208.png)

# HBase 설치

노드별로 모두 설치해야 함

[https://www.joinc.co.kr/w/man/12/hadoop/hbase/install](https://www.joinc.co.kr/w/man/12/hadoop/hbase/install)

```bash
wget https://dlcdn.apache.org/hbase/2.5.3/hbase-2.5.3-bin.tar.gz
sudo tar xvzf hbase-2.5.3-bin.tar.gz -C /usr/local
sudo ln -s /usr/local/hbase-2.5.3 /usr/local/hbase
sudo chown -R user:user /usr/local/hbase-2.5.3

#~/.bashrc
export HBASE_HOME=/usr/local/hbase
export HBASE_PID_DIR=/usr/local/hbase/pid
export PATH=$PATH:$HBASE_HOME/bin

```

hbase-site.xml

```xml
<configuration>
  <property>
    <name>hbase.tmp.dir</name>
    <value>./tmp</value>
  </property>
  <property>
    <name>hbase.zookeeper.property.dataDir</name>
    <value>/data/zookeeper</value>
  </property>
  <property>
    <name>hbase.unsafe.stream.capability.enforce</name>
    <value>false</value>
  </property>
  <property>
    <name>hbase.rootdir</name>
    <value>hdfs://master1.mshome.net:9000/hbase</value>
  </property>
  <property>
    <name>hbase.cluster.distributed</name>
    <value>true</value>
  </property>
  <property>
    <name>hbase.zookeeper.quorum</name>
    <value>master1.mshome.net,worker1.mshome.net,worker2.mshome.net</value>
  </property>
  <property>
    <name>dfs.replication</name>
    <value>1</value>
  </property>
  <property>
    <name>hbase.zookeeper.property.clientPort</name>
    <value>2181</value>
  </property>
  <property>
    <name>hbase.wal.provider</name>
    <value>filesystem</value>
  </property>
</configuration>
```

hbase-env.sh

```bash
export HBASE_MANAGES_ZK=false #Zookeeper (이하 ZK)인스턴스 관리, 별도 운영중이면 false
export HBASE_PID_DIR=/usr/local/hbase/pid
export JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64
```

regionservers : Region은 Hbase의 기본단위, 이것을 이용해 수평확

```bash
master1.mshome.net
worker1.mshome.net
worker2.mshome.net
```

hbase 실행

```bash
start-hbase.sh
```

hbase shell

```bash
hbase shell
list
create 'emps', 'personal data', 'professional data'
put 'emps', '1', 'personal data:name', 'yundream'
put 'emps', '1', 'personal data:city', 'seoul'
put 'emps', '1', 'professional data:designation', 'manager'
put 'emps', '1', 'professional data:salary', '50000'
scan 'emps'
get 'emps', '1'
```
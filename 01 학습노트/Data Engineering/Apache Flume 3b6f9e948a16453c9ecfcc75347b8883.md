# Apache Flume

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html)

File, DB, API 등의 다양한 데이터를 수집하여 Target으로 전송하는 도구

![Untitled](Apache%20Flume/Untitled.png)

![Untitled](Apache%20Flume/Untitled%201.png)

Flume 이벤트는 바이트 페이로드와 optional set of string 속성 단위의 데이터로 정의

Flume agent는 외부 소스에서부터 다음 목적지로의 이벤트 플로우를 만들어내는 JVM 프로세스

## Flume 아키텍처

### Consolidation (Fan-in)

수십대의 웹서버 로그를 수집하기 위한 Flume Agent가 있고 이를 하나의 Agent에서 통합하여 저장

![Untitled](Apache%20Flume/Untitled%202.png)

Multiplexing the Flow

하나 이상의 목적지에 데이터를 저장할 수 있게하기 위해 다중 이벤트 Flow를 지원

Channel-Sink가 복사되어 원하는 레파지토리로 데이터 전송이 가능

![Untitled](Apache%20Flume/Untitled%203.png)

# Source & Sink & Channel

## Flume Source

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-sources](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-sources)

## Flume Sink

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-sinks](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-sinks)

## Flume Channel

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-channels](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-channels)

### Flume Channel Selectors

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-channel-selectors](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-channel-selectors)

## Event Serializer

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#event-serializers](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#event-serializers)

## Interceptor

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-interceptors](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#flume-interceptors)

# 설치

[https://velog.io/@yje876/flume-설치](https://velog.io/@yje876/flume-%EC%84%A4%EC%B9%98)

```bash
wget https://dlcdn.apache.org/flume/1.11.0/apache-flume-1.11.0-bin.tar.gz
sudo tar xvzf apache-flume-1.11.0-bin.tar.gz -C /usr/local

cp flume-env.sh.template flume-env.sh
cp flume-conf.properties.template flume-conf.properties

# .bashrc
export FLUME_HOME=/usr/local/apache-flume-1.11.0-bin
export HIVE_HOME=/usr/local/hive
export PATH=$HIVE_HOME/bin:$HIVE_HOME/sbin:$FLUME_HOME/bin:$PATH

```

### Flume Agent 생성

```bash
$ bin/flume-ng agent -n smart_car -c conf -f conf/flume-conf.properties
```

예제 configuration file

```python
# example.conf: A single-node Flume configuration

# Name the components on this agent
smart_car.sources = SmartCarInfo_SpoolSource
smart_car.sinks = SmartCarInfo_LoggerSink
smart_car.channels = SmartCarInfo_Channel

# Describe/configure the source
smart_car.sources.SmartCarInfo_SpoolSource.type = spooldir
smart_car.sources.SmartCarInfo_SpoolSource.spoolDir = /home/hadoop/bigdata2nd-master/CH02/SmartCar
smart_car.sources.SmartCarInfo_SpoolSource.deletePolicy = immediate
smart_car.sources.SmartCarInfo_SpoolSource.batchSize=1000

# Describe the sink
smart_car.sinks.SmartCarInfo_LoggerSink.type = logger

# Use a channel which buffers events in memory
smart_car.channels.SmartCarInfo_Channel.type = memory
smart_car.channels.SmartCarInfo_Channel.capacity = 100000
smart_car.channels.SmartCarInfo_Channel.transactionCapacity = 10000

# Bind the source and sink to the channel
smart_car.sources.SmartCarInfo_SpoolSource.channels = SmartCarInfo_Channel
smart_car.sinks.SmartCarInfo_LoggerSink.channel = SmartCarInfo_Channel
```

interceptor

```python
smart_car.sources.SmartCarInfo_SpoolSource.interceptors=filterInterceptor
smart_car.sources.SmartCarInfo_SpoolSource.interceptors.filterInterceptor.type=regex_filter
smart_car.sources.SmartCarInfo_SpoolSource.interceptors.filterInterceptor.regex=^\\d{14}
smart_car.sources.SmartCarInfo_SpoolSource.interceptors.filterInterceptor.excludeEvents=false
```

Source-Channel-Sink 가 복수개로 추가

```python
smart_car.sources = SmartCarInfo_SpoolSource DriverCarInfo_TailSource
smart_car.sinks = SmartCarInfo_LoggerSink DriverCarInfo_KafkaSink
smart_car.channels = SmartCarInfo_Channel DriverCarInfo_Channel

# Describe/configure the source
smart_car.sources.DriverCarInfo_TailSource.type = exec
smart_car.sources.DriverCarInfo_TailSource.command= tail -F /home/hadoop/bigdata2nd-master/CH02/driver-realtime-log/SmartCarDriverInfo.log
smart_car.sources.DriverCarInfo_TailSource.restart=true
smart_car.sources.DriverCarInfo_TailSource.batchSize=1000

# Describe the sink
smart_car.sinks.DriverCarInfo_KafkaSink.type = org.apache.flume.sink.kafka.KafkaSink
smart_car.sinks.DriverCarInfo_KafkaSink.topic = SmartCar-Topic
smart_car.sinks.DriverCarInfo_KafkaSink.brokerList=node1:9092 node2:9092 node3:9092
smart_car.sinks.DriverCarInfo_KafkaSink.requireAcks = 1
smart_car.sinks.DriverCarInfo_KafkaSink..batchSize=1000

# Use a channel which buffers events in memory
smart_car.channels.DriverCarInfo_Channel.type = memory
smart_car.channels.DriverCarInfo_Channel.capacity = 100000
smart_car.channels.DriverCarInfo_Channel.transactionCapacity = 10000

# Bind the source and sink to the channel
smart_car.sources.DriverCarInfo_TailSource.channels = DriverCarInfo_Channel
smart_car.sinks.DriverCarInfo_KafkaSink.channel = DriverCarInfo_Channel

smart_car.sources.DriverCarInfo_TailSource.interceptors=filterInterceptor
smart_car.sources.DriverCarInfo_TailSource.interceptors.filterInterceptor.type=regex_filter
smart_car.sources.DriverCarInfo_TailSource.interceptors.filterInterceptor.regex=^\\d{14}
smart_car.sources.DriverCarInfo_TailSource.interceptors.filterInterceptor.excludeEvents=false
```

# Spring Boot Setup

[https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#spring-boot-setup](https://flume.apache.org/releases/content/1.11.0/FlumeUserGuide.html#spring-boot-setup)
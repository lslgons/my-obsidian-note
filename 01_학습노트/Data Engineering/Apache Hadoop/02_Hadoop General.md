# 02. Hadoop General

# Hadoop Command Reference

[https://hadoop.apache.org/docs/r3.3.4/hadoop-project-dist/hadoop-common/CommandsManual.html](https://hadoop.apache.org/docs/r3.3.4/hadoop-project-dist/hadoop-common/CommandsManual.html)

### Shell Commnad

하둡공통으로 사용하는 에코시스템용 처리시스템 (hadoop, hdfs, yarn, …)

### Shell Options

Options that the shell processes prior to executing Java (—buildpaths, —config, —debug, …)

### Command

Action to perform (fs, jar, archive)

### Generic Options

The command set of options (-fs, -config, -archive)

### Command Options

For the hadoop common sub-project(-p, -h, -format)

# Hadoop FS Shell

```bash
hadoop fs -ls / #Recursive: -R
bin/hdfs dfs -ls /

hadoop fs -mkdir /user
bin/hdfs dfs -mkdir /hdfs-user

bin/hdfs dfs -touchz /hdfs-user/test.txt

hadoop fs -put .bashrc /etc

hadoop fs -copyFromLocal /etc/apt/ /etc/apt

hadoop fs -get /etc/apt/sources.list /tmp

hadoop fs -copyToLocal /etc/apt/ /tmp

hadoop fs -cat /user/hadoop-test.txt

hadoop fs -mv /user/hadoop-test.txt /etc  #하둡내 파일시스템 내에서만 이동이 가능

hadoop fs -cp /user/hadoop-test.txt /etc

hadoop fs -rm /etc/hadoop-test.txt

hadoop fs -chmod 777 /user/

hadoop fs -chown root /user/hadoop-test.txt
```

Hadoop Check System Commands

```jsx
hdfs dfs -df -h
hdfs dfs -du -h
hdfs dfsadmin -report
hdfs dfs -test -e /tmp/test.csv
hdfs dfsadmin -setQuota 10000 /tmp #Number of files
hdfs dfsadmin -setSpaceQuota 60G /tmp/test
```

# Hadoop Common Commands

```bash
hadoop archive -archiveName daegon.har -p /user/hadoop logs /user/archive
```

[Hadoop General - User Commands.docx](02%20Hadoop%20General/Hadoop_General_-_User_Commands.docx)

# HDFS 문제 발생

- 파일 시스템 검사 : hdfs fsck /
- Safe모드 발생 후 빠져나오지 못할 경우 : hdfs dfsadmin —safemode leave
- Corrupt Blocks/Files 등이 복구 불가한 경우
    - 강제삭제 : hdfs fsck / —delete
    - /lost+found 디렉토리 이동 : hdfs fsck / —move
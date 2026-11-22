# Apache Presto

[https://medium.com/@sean.azoci/apache-presto-%EC%95%8C%EC%95%84%EB%B3%B4%EA%B8%B0-11faddcc4052](https://medium.com/@sean.azoci/apache-presto-%EC%95%8C%EC%95%84%EB%B3%B4%EA%B8%B0-11faddcc4052)

[https://prestodb.io/](https://prestodb.io/)

[https://medium.com/swlh/presto-with-kubernetes-and-s3-deployment-4e262849244a](https://medium.com/swlh/presto-with-kubernetes-and-s3-deployment-4e262849244a)

[https://ahana.io/blog/prestotutorial101/](https://ahana.io/blog/prestotutorial101/)

분산형 쿼리엔진 → Apache Hive metastore에 의해 MinIO에 데이터를 저장함으로써 DataLake를 오브젝트스토리지 기반으로 구성??

쿼리 엔진에 대한 리소스를 DB서버가 아닌 다른 서버로 분산할 수 있어 MPP DB가 아니더라도 Postgres, Mysql과 같은 일반적인 RDB활용이 가능하지 않을까?

Catalog를 통해 다양한 데이터 소스에 접근 → 단일 쿼리를 통해 Oracl → GPDB, MySQL → GPDB와 같은 ETL동작 수행이 가능해 보임 ( Insert gpdb.dmd.o_주문정보 into ... Select ... from oracle.adm.torderdtl )
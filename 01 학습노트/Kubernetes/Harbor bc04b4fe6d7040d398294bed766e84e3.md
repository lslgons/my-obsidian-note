# Harbor

## Container Registry 란?

- 컨테이너 이미지를 로컬 저장소에 저장하면서 생기는 불편함(외부에서 이미지 사용)을 해결하기 위한 방법
- **컨테이너 전용 저장소**

## Container Registry 종류

- 소스 코드를 담는 **Soruce Registry** (ex. github)
- War, Jar 등의 바이너리 파일을 담는 **Artifact Registry** (ex. Nexus)

## Container Registry 장점

- 컨테이너 레지스트리를 이용해 외부에서 이미지를 쉽게 가져올 수 있기 때문에 **컨테이너 이미지의 중앙화**가 가능해짐
    - Docker Hub, AWS, GCP, Azure 등의 클라우드 벤더 사에서 컨테이너 레지스트리를 서비스로 제공하고 있어 직접 구축할 필요가 없다.

## Harbor의 필요성

- 선택지가 많아짐에 따라 기존에 사용하던 Container Registry 서비스에서 **다른 서비스로 이미지를 옮겨**야 할 필요성 발생
- 오픈 소스 Container Registry인 Harbor를 통해 **Container Registry간 레플리케이션 수행 가능**
    - 레플리케이션: 데이터 저장 및 백업을 하는 방법과 관련있는 데이터를 호스트 컴퓨터에서 다른 컴퓨터로 옮기는 것

## Harbor 란?

- Open Source Container Registry 도구
- 격리된 환경에서 직접 Container Registry를 구축해야 할 니즈가 있는 경우 사용하는 도구
- 많은 레퍼런스 소유 & 신뢰도가 높은 도구
- RBAC 기반의 접근 제어, 컨테이너 취약성 검사, 웹 UI, 레지스트리 레플리케이션 등의 다양한 컨테이너 레지스트리를 관리하는 기능을 제공

## Harbor Architecture

- Core Service: Harbor의 **API와 인증, 웹 GUI를 담당**하는 Component. 이미지의 Pull/Push에 필요한 Token을 발행하며 웹 환경의 GUI 및 Webhook 제공한다.
- Job Service: **이미지 Replication과 같은 작업을 담당**하는 Components. 이미지 Pull/Push 작업을 시작, 중단, 재시도하는 역할이다.
- DataBase: **Harbor의 모든 Components는 Stateless**이다. 때문에 저장해야할 데이터들은 Redis와 PostgreSQL같은 **외부 저장소에 저장**한다. 메타데이터같은 정보들을 저장하는 역할이다.
- Image Registry: Harbor에서 **실제 컨테이너 레지스트리 역할**을 하는 Component. 이미지를 저장하는 저장소 역할을 해 가져온 이미지를 저장 및 관리한다.
- Vulnerability Scanning: Harbor는 **이미지의 취약점을 검사하기 위한 Scanner를 제공**한다. 이에 사용되는 Vulnerability Scanning는 오픈소스 프로젝트인 Trivy와 Clair를 사용한다.
- Trusted Content: **이미지의 Integrity와 Freshness를 증명하기 위한 Image Signing**을 맡는 Component. 이를 위해 Notary라는 도구를 사용하며, 이미지의 신뢰성을 증명하기 위해 Metadata에 새긴 Sign을 관리하는 역할을 한다.

### **Trivy service**

[https://github.com/aquasecurity/trivy](https://github.com/aquasecurity/trivy)

포괄적인 보안 스캐너

보안적 이슈와 타겟에 따라 다른 스캐너를 가진다.

타겟 : Container Image, Filesystem, Git repository(remote), Kuberentes cluster or resource

스캐너

- OS packages and software dependencies in use (SBOM)
- Known vulnerabilities (CVEs)
- IaC misconfigurations
- Sensitive information and secrets

### **Notary Service**

메타 데이터와 같은 신뢰하는 컨텐츠 서명에 사용되는 도구이다.

신뢰하는 메타 데이터에 서명을 하고, repository에 이미지를 push하고, 서명된 메타데이터를 사용하여 이미지의 무결성을 확인할 수 있다.

Notary Service = Notary Server + Notary Signer

Notary Server : 서명된 The Update Framework(TUF) meta data를 저장하고 업데이트한다.

Notary Signer : private한 서명키를 저장하고, 서버에 대한 서명 작업을 수행한다.
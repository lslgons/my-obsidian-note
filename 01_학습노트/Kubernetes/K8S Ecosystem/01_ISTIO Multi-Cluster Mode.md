# ISTIO Multi-Cluster Mode

[https://istio.io/latest/docs/setup/install/multicluster/multi-primary/](https://istio.io/latest/docs/setup/install/multicluster/multi-primary/)

이제 **1번 클러스터**를 `cluster.primary`로, **2번 클러스터**를 `cluster.secondary`로 구분하여 멀티 클러스터 환경에서 트래픽을 분배하는 방법을 다시 설명드리겠습니다. 이 예제에서는 **Istio의 멀티 클러스터 구성**을 통해 두 클러스터 간에 트래픽을 효율적으로 라우팅하는 방법을 다룹니다.

### **기본 설정**

- **클러스터 A**는 `cluster.primary`
- **클러스터 B**는 `cluster.secondary`
- 두 클러스터에 동일한 **apps** 네임스페이스에 **app**이라는 서비스가 배포되어 있으며, 클러스터 간 서비스 디스커버리가 가능하도록 설정됩니다.

### **1. 클러스터 간 서비스 디스커버리 설정 (ServiceEntry)**

멀티 클러스터 환경에서 **ServiceEntry**를 사용하여 두 클러스터 간의 통신을 설정합니다. 클러스터 간의 서비스 디스커버리를 활성화하여 각 클러스터가 상대 클러스터의 서비스를 알아볼 수 있게 합니다.

### **`ServiceEntry` (클러스터 A에서 클러스터 B로의 서비스 디스커버리 설정):**

```
apiVersion: networking.istio.io/v1alpha3
kind: ServiceEntry
metadata:
  name: app-serviceentry
  namespace: apps
spec:
  hosts:
  - "app.apps.svc.cluster.secondary"  # cluster.secondary에서의 app 서비스 주소
  location: MESH_INTERNAL
  resolution: DNS
  addresses:
  - "10.10.0.0/16"  # 다른 클러스터의 IP 주소 범위

```

- 위 설정은 **클러스터 A (cluster.primary)**에서 **클러스터 B (cluster.secondary)**의 `app.apps.svc.cluster.secondary` 서비스를 발견할 수 있도록 설정합니다.
- `addresses` 필드는 네트워크 범위에 따라 적절하게 설정해야 합니다.

### **2. Gateway 설정 (cluster.primary)**

이제 외부에서 들어오는 트래픽을 처리할 **Gateway**를 설정합니다. `cluster.primary`에서 **Ingress Gateway**를 설정하여 트래픽을 수신합니다.

```
apiVersion: networking.istio.io/v1alpha3
kind: Gateway
metadata:
  name: app-gateway
  namespace: apps
spec:
  selector:
    istio: ingressgateway  # Istio의 기본 ingress gateway를 사용
  servers:
  - port:
      number: 80
      name: http
      protocol: HTTP
    hosts:
    - "app.example.com"  # 외부에서 접근할 도메인

```

- **Gateway**는 외부에서 `app.example.com` 도메인으로 들어오는 HTTP 요청을 수신합니다.
- **Ingress Gateway**는 트래픽을 처리하고, 이후 **VirtualService**를 통해 트래픽을 클러스터 간에 분배합니다.

### **3. VirtualService 설정**

이제 **VirtualService**를 설정하여 외부에서 들어오는 트래픽을 각 클러스터의 `app` 서비스로 라우팅합니다. 각 클러스터의 `app` 서비스가 동일한 `app.apps.svc.cluster.local` 주소를 사용하도록 하고, 트래픽을 **Active-Active** 방식으로 분배합니다.

```
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: app-virtualservice
  namespace: apps
spec:
  hosts:
  - "app.example.com"  # 외부에서 접근할 도메인
  gateways:
  - app-gateway  # 위에서 설정한 Gateway를 참조
  http:
  - route:
    - destination:
        host: app.apps.svc.cluster.primary  # cluster.primary의 app 서비스
        subset: v1  # v1 버전
      weight: 50  # 클러스터 A에 50% 트래픽 할당
    - destination:
        host: app.apps.svc.cluster.secondary  # cluster.secondary의 app 서비스
        subset: v1  # v1 버전
      weight: 50  # 클러스터 B에 50% 트래픽 할당

```

- **VirtualService**는 `app.example.com`으로 들어오는 HTTP 요청을 두 클러스터에 배포된 `app` 서비스로 라우팅합니다.
- `weight`는 트래픽의 분배 비율을 나타내며, 예제에서는 **50%씩** 분배하고 있습니다.
- `destination.host` 필드는 각 클러스터의 `app` 서비스 주소를 지정합니다.

### **4. DestinationRule 설정 (각 클러스터에 대한 트래픽 정책)**

**DestinationRule**은 각 클러스터에 배포된 `app` 서비스에 대해 트래픽을 라우팅할 때 필요한 정책을 설정합니다. 예를 들어, `v1` 버전의 서비스를 대상으로 트래픽을 라우팅하도록 설정합니다.

### **`DestinationRule` (클러스터 A - cluster.primary):**

```
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: app-destinationrule-primary
  namespace: apps
spec:
  host: app.apps.svc.cluster.primary
  subsets:
  - name: v1
    labels:
      version: v1  # v1 버전의 서비스

```

- 클러스터 A (cluster.primary)의 `app` 서비스에 대해 `v1` 버전의 트래픽을 라우팅하도록 설정합니다.

### **`DestinationRule` (클러스터 B - cluster.secondary):**

```
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: app-destinationrule-secondary
  namespace: apps
spec:
  host: app.apps.svc.cluster.secondary
  subsets:
  - name: v1
    labels:
      version: v1  # v1 버전의 서비스

```

- 클러스터 B (cluster.secondary)의 `app` 서비스에 대해 `v1` 버전의 트래픽을 라우팅하도록 설정합니다.

### **5. 멀티 클러스터 네트워크 설정**

멀티 클러스터 환경에서는 각 클러스터 간 네트워크 설정이 중요합니다. Istio는 **DNS 기반의 서비스 디스커버리**를 사용하여 각 클러스터의 서비스를 찾습니다. 따라서 각 클러스터에서 다른 클러스터의 서비스 주소를 올바르게 구성해야 합니다.

1. **DNS 해석**: Istio는 서비스 이름을 `app.apps.svc.cluster.primary` 또는 `app.apps.svc.cluster.secondary`와 같은 형식으로 사용하여, 각 클러스터의 서비스를 해석할 수 있습니다.
2. **멀티 클러스터 연결**: 두 클러스터는 Istio **Multi-Cluster** 모드를 사용하여 서로 연결됩니다. 클러스터 간 **서비스 디스커버리**가 활성화되어야 하며, 이를 위해 Istio의 **ServiceEntry**와 같은 리소스를 사용하여 각 클러스터 간 연결을 구성합니다.

### **결론**

- **Cluster.primary**와 **Cluster.secondary**에서 각각 동일한 **app** 서비스가 존재하고, **Gateway**와 **VirtualService**를 통해 외부 트래픽을 두 클러스터로 분배할 수 있습니다.
- 트래픽은 `VirtualService`에서 정의한 `weight` 값을 기반으로 클러스터 간에 분배되며, 각 클러스터의 `DestinationRule`은 서비스의 버전 및 트래픽 라우팅을 정의합니다.
- **ServiceEntry**를 통해 멀티 클러스터 환경에서 서비스 디스커버리가 가능하고, **Active-Active** 방식으로 트래픽을 처리할 수 있습니다.

이러한 구성을 통해 **고가용성**을 제공하면서, 여러 클러스터에 걸쳐 **서비스 메시**를 효율적으로 관리할 수 있습니다.

좋은 질문입니다! `ServiceEntry`의 `addresses`와 **Gateway 설정**에 대해 좀 더 명확히 설명드리겠습니다.

### **1. ServiceEntry의 `addresses` 필드**

`ServiceEntry` 리소스의 `addresses` 필드는 **클러스터 간 서비스 통신을 위한 IP 주소 범위**를 지정하는 데 사용됩니다. 이는 **클러스터의 내부 IP** 범위를 나타내며, **노드의 IP**가 아니라 클러스터 내에서 서로 연결된 서비스 간의 통신을 위한 **네트워크 주소**를 지정합니다.

- `addresses` **필드**는 기본적으로 **다른 클러스터**에 배포된 서비스가 사용할 수 있는 IP 범위를 지정하는 데 사용됩니다.
- 이는 클러스터 간의 **서비스 디스커버리**를 설정할 때, 다른 클러스터의 **서비스 IP 주소 범위**를 지정하는 데 필요합니다. 예를 들어, 두 클러스터가 서로 통신하려면 각 클러스터의 **서비스 IP 범위**를 알아야 하므로, 이 정보를 `ServiceEntry`에 명시적으로 제공하는 것입니다.

### **예시:**

```
apiVersion: networking.istio.io/v1alpha3
kind: ServiceEntry
metadata:
  name: app-serviceentry
  namespace: apps
spec:
  hosts:
  - "app.apps.svc.cluster.secondary"  # 다른 클러스터의 app 서비스 주소
  location: MESH_INTERNAL
  resolution: DNS
  addresses:
  - "10.10.0.0/16"  # 다른 클러스터의 IP 주소 범위 (내부 서비스 IP 범위)

```

위의 예시에서 `addresses`는 **클러스터 B (secondary)**의 서비스가 위치한 **내부 네트워크 범위**를 지정하는 것입니다. `10.10.0.0/16`은 **클러스터 B**에서 사용하는 **IP 주소 범위**일 수 있습니다. 이 범위는 실제 클러스터 설정에 따라 달라집니다.

따라서, `ServiceEntry`에서 **노드의 IP**는 지정하지 않고, 클러스터 간의 **서비스가 통신하는 내부 IP 범위**를 지정하는 것입니다.

### **2. Gateway 설정 (secondary 클러스터에 대한 설정)**

**멀티 클러스터 환경에서 Gateway는 각 클러스터마다 만들어야 합니다**. 즉, `cluster.primary`에만 Gateway를 설정하는 것이 아니라, `cluster.secondary`**에도 Gateway를 설정해야** 합니다. 그렇지 않으면 `cluster.secondary`로 오는 외부 트래픽을 처리할 방법이 없기 때문입니다.

**각 클러스터에 Gateway를 설정해야 하는 이유**:

- Istio의 **Gateway**는 주로 **Ingress 트래픽**을 처리하는 데 사용됩니다. 클러스터가 서로 **독립적인 인프라**에서 실행되기 때문에, 각 클러스터에 대한 **외부 트래픽의 수신**을 위한 별도의 Gateway가 필요합니다.
- **Gateway**는 외부에서 들어오는 요청을 **클러스터 내부 서비스**로 라우팅하는 역할을 하므로, 각 클러스터마다 외부 요청을 처리할 수 있는 Gateway가 필요합니다.

### **예시:**

`cluster.primary`에 설정한 Gateway에 추가하여 **`cluster.secondary`**에도 Gateway를 설정하는 방법은 아래와 같습니다.

### **클러스터 A (`cluster.primary`)의 Gateway:**

```
apiVersion: networking.istio.io/v1alpha3
kind: Gateway
metadata:
  name: app-gateway
  namespace: apps
spec:
  selector:
    istio: ingressgateway  # Istio의 기본 ingress gateway를 사용
  servers:
  - port:
      number: 80
      name: http
      protocol: HTTP
    hosts:
    - "app.example.com"  # 외부에서 접근할 도메인

```

### **클러스터 B (`cluster.secondary`)의 Gateway:**

```
apiVersion: networking.istio.io/v1alpha3
kind: Gateway
metadata:
  name: app-gateway
  namespace: apps
spec:
  selector:
    istio: ingressgateway  # Istio의 기본 ingress gateway를 사용
  servers:
  - port:
      number: 80
      name: http
      protocol: HTTP
    hosts:
    - "app.example.com"  # 외부에서 접근할 도메인

```

### **3. 추가적인 설정**

`cluster.primary`와 `cluster.secondary`에서 모두 **Gateway**를 설정해야 한다는 점을 염두에 두고, 외부 트래픽을 각 클러스터로 라우팅하려면 **VirtualService**를 통해 각 클러스터에 트래픽을 분배하는 설정이 필요합니다. 이때 **트래픽 분배 정책**은 각 클러스터에 배포된 서비스를 기준으로 설정됩니다.

---

### **최종 구성 예시**

### **1. ServiceEntry (클러스터 간 통신 설정)**

각 클러스터의 서비스를 서로 인식할 수 있도록 **ServiceEntry**를 설정합니다.

```
apiVersion: networking.istio.io/v1alpha3
kind: ServiceEntry
metadata:
  name: app-serviceentry
  namespace: apps
spec:
  hosts:
  - "app.apps.svc.cluster.secondary"  # 클러스터 B에서 app 서비스
  location: MESH_INTERNAL
  resolution: DNS
  addresses:
  - "10.10.0.0/16"  # 클러스터 B의 IP 주소 범위

```

### **2. VirtualService (트래픽 분배 설정)**

`cluster.primary`와 `cluster.secondary`로 트래픽을 분배합니다.

```
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: app-virtualservice
  namespace: apps
spec:
  hosts:
  - "app.example.com"  # 외부에서 접근할 도메인
  gateways:
  - app-gateway  # 위에서 설정한 Gateway를 참조
  http:
  - route:
    - destination:
        host: app.apps.svc.cluster.primary  # 클러스터 A 서비스
        subset: v1
      weight: 50
    - destination:
        host: app.apps.svc.cluster.secondary  # 클러스터 B 서비스
        subset: v1
      weight: 50

```

### **3. DestinationRule (서비스의 버전 정의)**

```
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: app-destinationrule-primary
  namespace: apps
spec:
  host: app.apps.svc.cluster.primary
  subsets:
  - name: v1
    labels:
      version: v1

```

### **4. 클러스터 B (secondary)에서의 DestinationRule**

```
apiVersion: networking.istio.io/v1alpha3
kind: DestinationRule
metadata:
  name: app-destinationrule-secondary
  namespace: apps
spec:
  host: app.apps.svc.cluster.secondary
  subsets:
  - name: v1
    labels:
      version: v1

```

---

### **결론**

1. `ServiceEntry`**의** `addresses` **필드는 노드의 IP가 아니라 클러스터 간 서비스 통신을 위한 내부 서비스 IP 범위를 설정합니다.**
2. *`Gateway`는 각 클러스터마다 별도로 설정해야 하며, **`cluster.primary`**와** `cluster.secondary` **각 클러스터에서 외부 트래픽을 처리할 수 있도록 각각 설정해야 합니다.**
3. 이를 통해 멀티 클러스터 환경에서 트래픽 분배, 서비스 디스커버리, 고가용성을 설정할 수 있습니다.
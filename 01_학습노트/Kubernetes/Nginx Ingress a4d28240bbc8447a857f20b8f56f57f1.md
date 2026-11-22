# Nginx Ingress

설명 : [https://blog.naver.com/alice_k106/221502890249](https://blog.naver.com/alice_k106/221502890249)

**yaml을 통한 설치**

```bash
# Nginx Ingress Controller 설치
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/baremetal/deploy.yaml
```

**Ingress Controller에 접근하기 위한 Endpoint IP**

Nginx Ingress Controller를 정의한 후 외부 노출을 위해 별도의 서비스를 생성해야 함

온프레미스 환경에서는 NodePort의 서비스를 구성함 (가장 직관적)

```bash
apiVersion: v1
kind: Service
metadata:
  name: ingress-nginx
  namespace: ingress-nginx
spec:
  ports:
    - name: service
      port: 80
      targetPort: 80
      nodePort: 30080
  selector:
    app.kubernetes.io/name: ingress-nginx
    app.kubernetes.io/part-of: ingress-nginx
  type: NodePort
```

Nginx Official quick-start guide: [https://kubernetes.github.io/ingress-nginx/deploy/#quick-start](https://kubernetes.github.io/ingress-nginx/deploy/#quick-start)

intstall nginx ingress controller

```bash
helm upgrade --install nginx ingress-nginx \
  --repo https://kubernetes.github.io/ingress-nginx
  #--namespace monitoring --create-namespace
 
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
 
helm repo update
 
helm install ingress-nginx ingress-nginx/ingress-nginx
 
helm delete nginx
#--namespace monitoring
```

Default Backend

Ingress에 정의되지 않은 경로로 요청이 들어올 경우의 랜딩페이지

```yaml
apiVersion: v1
kind: Service
metadata:
  namespace: default
  name: default-http-backend
spec:
  type: ClusterIP
  ports:
  - port: 80
    protocol: TCP
    targetPort: 8080
  # Matches Deployment
  selector:
    app: default-http-backend
---
apiVersion: extensions/v1beta1
kind: Deployment
metadata:
  namespace: default
  name: default-http-backend
spec:
  replicas: 2
  template:
    metadata:
      # Matches Deployment
      labels:
        app: default-http-backend
    spec:
      terminationGracePeriodSeconds: 60
      containers:
      - name: default-http-backend
        # Any image is permissable as long as:
        # 1. It serves a 404 page at /
        # 2. It serves 200 on a /healthz endpoint
        image: gcr.io/google_containers/defaultbackend:1.0
        livenessProbe:
          httpGet:
            path: /healthz
            port: 8080
            scheme: HTTP
          initialDelaySeconds: 30
          timeoutSeconds: 5
        ports:
        - containerPort: 8080
        resources:
          limits:
            cpu: 10m
            memory: 20Mi
          requests:
            cpu: 10m
            memory: 20Mi
```

Nginx-Ingress 설정단을 바꿔준 후 재생성 (kubectl apply -f mandatory.yaml)

```yaml
...
containers:
- name: nginx-ingress-controller
	...
	args:
	- --default-backend-service=default/default-http-backend

```
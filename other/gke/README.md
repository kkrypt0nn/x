# GKE Deployment with IAM & KSA

Since Azure didn't want me to get started with their free plan, I decided to do exactly what I planned to test on Azure, but on Google Cloud Platform.

Instead of using Bicep, I will be using Terraform.

## Defintions

As both cloud environments don't use the same terminologies, here is the naming

| Azure                                                       | GCP Equivalent                                     |
| ----------------------------------------------------------- | -------------------------------------------------- |
| AKS (Azure Kubernetes Service)                              | GKE (Google Kubernetes Engine)                     |
| Key Vault                                                   | Secret Manager                                     |
| Managed Identity / Workload Identity / Federated Credential | Workload Identity Federation (IAM + Kubernetes SA) |

## Structure

```
.
├── app
│   ├── main.py               # Entry point for the application
│   └── requirements.txt      # Python dependencies
├── Dockerfile                # Container image definition
├── infrastructure.tf         # Terraform configuration for infrastructure provisioning
├── k8s
│   ├── deployment.yml        # Kubernetes Deployment configuration
│   ├── ingress.yml           # Kubernetes Ingress rules
│   └── service.yml           # Kubernetes Service definition
└── README.md                 # This file :)
```

## Deploy

To deploy the following steps have to be done

### 1. Login to GCP

```bash
gcloud auth login
gcloud config set project gke-test-459719
gcloud auth application-default login
```

### 2. Enable the GCP services

```bash
gcloud services enable \
  container.googleapis.com \
  iam.googleapis.com \
  secretmanager.googleapis.com \
  cloudresourcemanager.googleapis.com
```

### 3. Apply the Terraform infrastructure

```bash
terraform init
terraform plan
terraform apply
```

### 4. Get into the created cluster to create the KSA

```bash
gcloud components install gke-gcloud-auth-plugin
gcloud config set container/use_application_default_credentials true

gcloud container clusters get-credentials python-secret-app-cluster \
  --region us-central1 \
  --project gke-test-459719

kubectl get nodes
```

### 5. Install the NGINX Ingress in GKE

```bash
gcloud container clusters get-credentials python-secret-app-cluster --region us-central1 --project gke-test-459719

helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update

helm install nginx-ingress ingress-nginx/ingress-nginx --namespace ingress-nginx --create-namespace
```

### 6. Build and push the Docker image

```bash
docker build --platform linux/amd64,linux/arm64 -t gcr.io/gke-test-459719/python-secret-app:latest .
gcloud auth configure-docker
docker push gcr.io/gke-test-459719/python-secret-app:latest

# To check if the image has been pushed
gcloud container images list --project gke-test-459719
```

### 7. Deploy the image to GKE

```bash
gcloud container clusters get-credentials python-secret-app-cluster --region us-central1 --project gke-test-459719

kubectl apply -f k8s/deployment.yml
kubectl apply -f k8s/service.yml
kubectl apply -f k8s/ingress.yml
```

### 8. Destroy the Terraform infrastructure

```bash
terraform plan -destroy
terraform apply -destroy
```

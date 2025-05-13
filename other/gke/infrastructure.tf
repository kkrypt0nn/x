# Set the variables
variable "project_id" {
  default = "gke-test-459719"
}

variable "region" {
  default = "us-central1"
}

# Configure the providers
provider "google" {
  project = var.project_id
  region  = var.region
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

# Create a GKE cluster without the default node pool
resource "google_container_cluster" "gke" {
  name                = "python-secret-app-cluster"
  location            = var.region
  deletion_protection = false

  remove_default_node_pool  = true
  initial_node_count        = 1

  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }
}

# Add a custom node pool to the cluster
resource "google_container_node_pool" "primary_nodes" {
  name       = "primary-node-pool"
  location   = var.region
  cluster    = google_container_cluster.gke.name
  node_count = 1

  node_config {
    machine_type = "e2-medium"
    disk_type    = "pd-standard"
    disk_size_gb = 50
    
    # Allow full access to all Cloud APIs
    oauth_scopes = ["https://www.googleapis.com/auth/cloud-platform"]

    workload_metadata_config {
      mode = "GKE_METADATA"
    }
  }
}

# Create a secret in Secret Manager
resource "google_secret_manager_secret" "foo" {
  secret_id = "foo"

  replication {
    auto {}
  }
}

# Add a version of the secret with data
resource "google_secret_manager_secret_version" "foo_version" {
  secret      = google_secret_manager_secret.foo.id
  secret_data = "bar baz"
}

# Create a Google Cloud service account for the app
resource "google_service_account" "k8s_sa" {
  account_id   = "k8s-python-app"
  display_name = "KSA for Python Secrets App"
}

# Grant the service account access to Secret Manager
resource "google_project_iam_member" "secret_access" {
  project = var.project_id
  role    = "roles/secretmanager.secretAccessor"
  member  = "serviceAccount:${google_service_account.k8s_sa.email}"
}

# Create a Kubernetes service account and associate it with the Google service account
resource "kubernetes_service_account" "ksa" {
  metadata {
    name      = "python-app-ksa"
    namespace = "default"

    annotations = {
      "iam.gke.io/gcp-service-account" = google_service_account.k8s_sa.email
    }
  }
}

# Allow the Kubernetes service account to impersonate the Google service account via Workload Identity
resource "google_service_account_iam_member" "workload_identity_user" {
  service_account_id = google_service_account.k8s_sa.id
  role               = "roles/iam.workloadIdentityUser"
  member             = "serviceAccount:${var.project_id}.svc.id.goog[default/python-app-ksa]"
}

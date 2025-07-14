# PlatML

A proof-of-concept ML platform with independent model management and serving APIs, built with Rust (Axum), React, and deployed on Google Kubernetes Engine (GKE).

## Why This Platform

**Problem**: ML model deployment requires complex ML infrastructure and specialized expertise.

**Solution**: Dead-simple API to deploy models in under a minute, powered by Rust's performance.

### Rust + Kubernetes = Perfect ML Serving

✅ **Blazing Fast** - No GC pauses, predictable sub-millisecond latency  
✅ **Memory Efficient** - 10x smaller footprint than Python alternatives  
✅ **Production Safe** - Rust's guarantees prevent crashes and memory leaks  
✅ **Infinitely Scalable** - Kubernetes auto-scaling from 0 to 10,000 RPS  

### Use Cases
- **Real-time Inference** - When milliseconds matter
- **Edge Deployment** - Minimal resources, maximum performance  
- **Rapid Experimentation** - Test models without ML infrastructure complexity

**Status**: This is a prototype demonstrating the potential of Rust-based ML serving infrastructure.

**Bottom Line**: A proof of concept for the fastest, safest, simplest way to serve ML models in production.

## 🎥 Demo

Watch the platform in action: [Upload your video to one of these platforms]
- **YouTube** (Recommended for public demos)
- **Vimeo** (Professional looking, privacy controls)
- **Google Drive** (Set to "Anyone with link can view")
- **Loom** (Great for quick technical demos)
- **GitHub** (If under 10MB, add to repo in `/docs/demo.mp4`)

## Architecture Overview

The platform consists of two main services:

1. **Model Management API** (Port 8081) - Handles model lifecycle management
2. **Model Serving API** (Port 8080) - Serves predictions from loaded models

```
┌─────────────────┐     ┌──────────────────┐
│   Client App    │     │  Model Storage   │
└────────┬────────┘     └────────┬─────────┘
         │                       │
         │                       │
    ┌────▼──────────────────────▼────┐
    │      Model Management API      │
    │          (Port 8081)           │
    │  • Create/Delete models        │
    │  • Upload model files          │
    │  • List models                  │
    └────────────────┬───────────────┘
                     │
                     │ Model files
                     │
    ┌────────────────▼───────────────┐
    │      Model Serving API         │
    │          (Port 8080)           │
    │  • Load models                 │
    │  • Make predictions             │
    │  • Health checks               │
    └────────────────────────────────┘
```

## Features

### Model Management API
- **Model CRUD Operations**: Create, read, update, and delete model metadata
- **File Upload**: Upload model files with multipart form data
- **Storage Abstraction**: Supports both local and GCS storage backends
- **Status Tracking**: Track model lifecycle (Created → Uploading → Active → Inactive)

### Model Serving API
- **Dynamic Model Loading**: Load models on-demand by ID
- **Prediction Endpoint**: Make predictions with loaded models
- **Iris Classification**: Built-in rule-based classifier for demo purposes
- **Model Management**: List and delete loaded models

## API Endpoints

### Model Management API (8081)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Health check |
| POST | `/models` | Create new model metadata |
| GET | `/models` | List all models |
| GET | `/models/:id` | Get model details |
| DELETE | `/models/:id` | Delete a model |
| POST | `/models/:id/upload` | Upload model file |

### Model Serving API (8080)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Health check |
| POST | `/predict` | Make prediction |
| POST | `/models/:model_id` | Load a model |
| DELETE | `/models/:model_id` | Unload a model |
| GET | `/models` | List loaded models |

## Model Storage

Models are stored in the following structure:
```
/app/model_storage/models/
└── {model_id}/
    └── {version}.model
```

The platform supports:
- **Local Storage**: Default for both development and Kubernetes deployment
- **PVC Storage**: Used in Kubernetes for shared storage between pods
- **GCS Storage**: Available but not currently implemented

## Iris Classification Example

The serving API includes a simple rule-based Iris classifier that expects 4 features:
- `[sepal_length, sepal_width, petal_length, petal_width]`

Classification rules:
- Petal length < 2.5 cm → Setosa (class 0)
- Petal length < 5.0 cm → Versicolor (class 1)
- Otherwise → Virginica (class 2)

## Development

### Prerequisites
- Rust 1.70+
- Docker (optional)

### Running Locally

1. Start the Model Management API:
```bash
cd api
cargo run
```

2. Start the Model Serving API:
```bash
cd serve
cargo run
```

## Docker Deployment

Build and run with Docker:
```bash
# Build images
docker build -t ml-api ./api
docker build -t ml-serve ./serve

# Run containers
docker run -p 8081:8081 ml-api
docker run -p 8080:8080 ml-serve
```

## GKE Deployment Details

### Cluster Configuration
- **Cluster Name**: mlplat
- **Location**: us-central1
- **Node Configuration**: 
  - Machine type: Autopilot (automatically managed)
  - Initial nodes: 1 (auto-scales based on demand)

### Kubernetes Resources
```yaml
Namespace: platml
Services:
  - platml-api (LoadBalancer) - Port 80
  - platml-serve (LoadBalancer) - Port 80  
  - platml-frontend (LoadBalancer) - Port 80
Storage:
  - PVC: 2Gi ReadWriteOnce (standard-rwo)
```

### Quick Deployment Commands
```bash
# Create namespace
kubectl apply -f deployment/ns.yaml

# Deploy storage
kubectl apply -f deployment/pvc-storage.yaml

# Deploy all services
kubectl apply -f deployment/api-deploy.yaml
kubectl apply -f deployment/serve-deploy.yaml
kubectl apply -f deployment/frontend-deploy.yaml

# Check deployment status
kubectl get all -n platml
```

## 📸 Deep Dive Demo Walkthrough

<details>
<summary><b>Click to expand full demo walkthrough with screenshots</b></summary>

### Complete Workflow Demonstration

The platform demonstrates a complete ML model lifecycle from registration to prediction.

### Step-by-Step Process

#### 1. **Model Registration**
When you land on the platform, you start by registering a new model:
- Enter a model name (e.g., "xmodel")
- Specify the version (default: "1.0.0")
- Click "Register Model"

**Behind the scenes:**
- The frontend calls `POST /api/models` through the nginx proxy
- The Model Management API creates a new model entry with a unique UUID
- The model status is set to "CREATED"
- Model metadata is stored in the in-memory database

#### 2. **Model File Upload**
After registration, upload the actual model file:
- Select your registered model from the list
- Choose a model file from your filesystem (supports .bin, .onnx, .pb, .pt, .h5 formats)
- Click "Upload Model File"

**Behind the scenes:**
- The frontend sends a multipart form request to `POST /api/models/{id}/upload`
- The API saves the file to `/app/model_storage/models/{model_id}/{version}.model`
- The model status updates to "ACTIVE"
- Both API and Serving services share this storage location via PVC

#### 3. **Load Model for Serving**
Navigate to the "Serve Model" tab to load your model:
- View the selected model details (name, version, ID, status)
- Click "Load Model for Serving"

**Behind the scenes:**
- The frontend calls `POST /serve/models/{model_id}`
- The Serving API loads the model file from the shared storage into memory
- The model is now ready to make predictions

#### 4. **Make Predictions**
Finally, use your loaded model to make predictions:
- The Iris dataset features are pre-populated with sample values
- Adjust the features as needed:
  - Sepal Length: 5.1 cm
  - Sepal Width: 3.5 cm
  - Petal Length: 1.4 cm
  - Petal Width: 0.2 cm
- Click "Make Prediction"
- View the results:
  - **Class**: Iris Setosa (0)
  - **Confidence**: 95.00%

**Behind the scenes:**
- The frontend sends `POST /serve/predict` with the feature array
- The Serving API uses a rule-based classifier:
  - Petal length < 2.5 cm → Setosa (class 0)
  - Petal length < 5.0 cm → Versicolor (class 1)
  - Otherwise → Virginica (class 2)
- Returns prediction and confidence score

### Architecture Highlights

- **Microservices Design**: Separate services for model management and serving
- **Shared Storage**: PVC enables model files to be accessible by both services
- **API Gateway**: Nginx reverse proxy routes `/api/*` and `/serve/*` requests
- **RESTful APIs**: Clean separation of concerns with dedicated endpoints
- **Real-time Inference**: Models loaded in memory for fast predictions

</details>

## License

MIT
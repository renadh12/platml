# ML Platform

A microservices-based machine learning platform built with Rust and Axum, designed for model management and serving.

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

## Usage Flow

### 1. Create a Model
```bash
curl -X POST http://localhost:8081/models \
  -H "Content-Type: application/json" \
  -d '{
    "name": "iris-classifier",
    "version": "1.0.0"
  }'
```

### 2. Upload Model File
```bash
curl -X POST http://localhost:8081/models/{model_id}/upload \
  -F "file=@model.pkl"
```

### 3. Load Model for Serving
```bash
curl -X POST http://localhost:8080/models/{model_id}
```

### 4. Make Predictions
```bash
curl -X POST http://localhost:8080/predict \
  -H "Content-Type: application/json" \
  -d '{
    "features": [5.1, 3.5, 1.4, 0.2]
  }'
```

## Model Storage

Models are stored in the following structure:
```
/app/model_storage/models/
└── {model_id}/
    └── {version}.model
```

The platform supports:
- **Local Storage**: For development and testing
- **GCS Storage**: For production deployments (configured via environment)

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

### Environment Variables

- `STORAGE_TYPE`: Set to "gcs" for Google Cloud Storage (default: "local")
- `GCS_BUCKET`: GCS bucket name for model storage

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

## License

MIT
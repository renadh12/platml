import React, { useState, useRef } from 'react';

function ModelUpload({ model, onModelUploaded }) {
  const [file, setFile] = useState(null);
  const [status, setStatus] = useState({ message: '', type: '' });
  const [isLoading, setIsLoading] = useState(false);
  const fileInputRef = useRef(null);

  const handleFileChange = (e) => {
    if (e.target.files.length > 0) {
      setFile(e.target.files[0]);
    }
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = (e) => {
    e.preventDefault();
    e.stopPropagation();
    
    if (e.dataTransfer.files.length > 0) {
      setFile(e.dataTransfer.files[0]);
    }
  };

  const handleUpload = async (e) => {
    e.preventDefault();
    
    if (!file) {
      setStatus({
        message: 'Please select a file to upload',
        type: 'error'
      });
      return;
    }
    
    setIsLoading(true);
    setStatus({ message: '', type: '' });
    
    const formData = new FormData();
    formData.append('file', file);
    
    try {
      const response = await fetch(`http://localhost:8081/models/${model.id}/upload`, {
        method: 'POST',
        body: formData,
      });
      
      const data = await response.json();
      
      if (response.ok) {
        setStatus({
          message: `Model file uploaded successfully to: ${data.gcs_path}`,
          type: 'success'
        });
        setFile(null);
        if (fileInputRef.current) {
          fileInputRef.current.value = '';
        }
        if (onModelUploaded) onModelUploaded(model);
      } else {
        setStatus({
          message: `Failed to upload model file: ${data.message || 'Unknown error'}`,
          type: 'error'
        });
      }
    } catch (error) {
      setStatus({
        message: `Error: ${error.message}`,
        type: 'error'
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="card">
      <h2>Upload Model File</h2>
      
      <div>
        <h3>Selected Model</h3>
        <p><strong>Name:</strong> {model.name}</p>
        <p><strong>Version:</strong> {model.version}</p>
        <p><strong>ID:</strong> {model.id}</p>
      </div>
      
      {status.message && (
        <div className={`alert alert-${status.type}`}>
          {status.message}
        </div>
      )}
      
      <form onSubmit={handleUpload}>
        <div 
          className="file-upload"
          onDragOver={handleDragOver}
          onDrop={handleDrop}
          onClick={() => fileInputRef.current.click()}
        >
          <p>Drag and drop your model file here, or click to select a file</p>
          <p>Supported formats: .bin, .onnx, .pb, .pt, .h5</p>
          <input
            type="file"
            ref={fileInputRef}
            onChange={handleFileChange}
            style={{ display: 'none' }}
          />
          {file && (
            <p>Selected file: <strong>{file.name}</strong> ({(file.size / 1024).toFixed(2)} KB)</p>
          )}
        </div>
        
        <button type="submit" disabled={!file || isLoading}>
          {isLoading ? 'Uploading...' : 'Upload Model File'}
        </button>
      </form>
    </div>
  );
}

export default ModelUpload;
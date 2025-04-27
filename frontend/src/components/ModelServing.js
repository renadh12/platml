import React, { useState } from 'react';

function ModelServing({ model, onModelLoaded }) {
  const [status, setStatus] = useState({ message: '', type: '' });
  const [isLoading, setIsLoading] = useState(false);

  const loadModel = async () => {
    setIsLoading(true);
    setStatus({ message: '', type: '' });
    
    try {
      const response = await fetch(`http://localhost:8080/models/${model.id}`, {
        method: 'POST',
      });
      
      const text = await response.text();
      
      if (response.ok) {
        setStatus({
          message: `Model loaded successfully: ${text}`,
          type: 'success'
        });
        if (onModelLoaded) onModelLoaded(model);
      } else {
        setStatus({
          message: `Failed to load model: ${text || 'Unknown error'}`,
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

  const checkModelStatus = async () => {
    setIsLoading(true);
    
    try {
      const response = await fetch(`http://localhost:8080/models`, {
        method: 'GET',
      });
      
      const data = await response.json();
      
      if (response.ok) {
        // Check if our model is loaded
        if (data && data.models && data.models.length > 0) {
          const loadedModel = data.models.find(m => m.id === model.id);
          if (loadedModel) {
            setStatus({
              message: `Model is currently loaded and ready for predictions`,
              type: 'success'
            });
            if (onModelLoaded) onModelLoaded(model);
          } else {
            setStatus({
              message: 'Model is not currently loaded',
              type: 'error'
            });
          }
        } else {
          setStatus({
            message: 'No models are currently loaded',
            type: 'error'
          });
        }
      } else {
        setStatus({
          message: `Failed to check model status: ${data.message || 'Unknown error'}`,
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
      <h2>Load Model for Serving</h2>
      
      <div>
        <h3>Selected Model</h3>
        <p><strong>Name:</strong> {model.name}</p>
        <p><strong>Version:</strong> {model.version}</p>
        <p><strong>ID:</strong> {model.id}</p>
        <p><strong>Status:</strong> {model.status}</p>
      </div>
      
      {status.message && (
        <div className={`alert alert-${status.type}`}>
          {status.message}
        </div>
      )}
      
      <div style={{ display: 'flex', gap: '10px', justifyContent: 'center' }}>
        <button onClick={loadModel} disabled={isLoading}>
          {isLoading ? 'Loading...' : 'Load Model for Serving'}
        </button>
        
        <button onClick={checkModelStatus} disabled={isLoading} className="secondary">
          Check Model Status
        </button>
      </div>
      
      <div className="note" style={{ marginTop: '20px', fontStyle: 'italic' }}>
        <p>This will load the model into the serving application memory.</p>
        <p>Once loaded, you can make predictions with the model.</p>
      </div>
    </div>
  );
}

export default ModelServing;
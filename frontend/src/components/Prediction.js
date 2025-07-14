import React, { useState } from 'react';

function Prediction({ model }) {
  const [features, setFeatures] = useState(['5.1', '3.5', '1.4', '0.2']);
  const [result, setResult] = useState(null);
  const [status, setStatus] = useState({ message: '', type: '' });
  const [isLoading, setIsLoading] = useState(false);
  
  // Feature labels for Iris dataset
  const featureLabels = [
    "Sepal Length (cm)",
    "Sepal Width (cm)",
    "Petal Length (cm)",
    "Petal Width (cm)"
  ];

  const addFeature = () => {
    setFeatures([...features, '0.0']);
  };

  const removeFeature = (index) => {
    const newFeatures = [...features];
    newFeatures.splice(index, 1);
    setFeatures(newFeatures);
  };

  const handleFeatureChange = (index, value) => {
    const newFeatures = [...features];
    newFeatures[index] = value;
    setFeatures(newFeatures);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    setStatus({ message: '', type: '' });
    setResult(null);
    
    // Convert string features to numbers
    const numericFeatures = features.map(f => parseFloat(f));
    
    try {
      const response = await fetch('/serve/predict', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ features: numericFeatures }),
      });
      
      const data = await response.json();
      
      if (response.ok) {
        setResult(data);
        setStatus({
          message: 'Prediction completed successfully',
          type: 'success'
        });
      } else {
        setStatus({
          message: `Failed to get prediction: ${data.message || 'Unknown error'}`,
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

  // Helper function to get class name from prediction value
  const getIrisClassName = (prediction) => {
    if (prediction === 0) return "Iris Setosa";
    if (prediction === 1) return "Iris Versicolor";
    if (prediction === 2) return "Iris Virginica";
    return "Unknown";
  };

  return (
    <div className="card">
      <h2>Make Predictions</h2>
      
      <div>
        <h3>Active Model</h3>
        <p><strong>Name:</strong> {model.name}</p>
        <p><strong>Version:</strong> {model.version}</p>
        <p><strong>ID:</strong> {model.id}</p>
      </div>
      
      {status.message && (
        <div className={`alert alert-${status.type}`}>
          {status.message}
        </div>
      )}
      
      <form onSubmit={handleSubmit} className="prediction-form">
        <h3>Input Features</h3>
        
        {features.map((feature, index) => (
          <div key={index} className="prediction-input">
            <label htmlFor={`feature-${index}`}>
              {index < featureLabels.length ? featureLabels[index] : `Feature ${index + 1}`}:
            </label>
            <input
              type="number"
              id={`feature-${index}`}
              value={feature}
              onChange={(e) => handleFeatureChange(index, e.target.value)}
              step="0.1"
              required
            />
            <button 
              type="button" 
              onClick={() => removeFeature(index)}
              className="secondary"
              disabled={features.length <= 1}
            >
              Remove
            </button>
          </div>
        ))}
        
        <div>
          <button type="button" onClick={addFeature}>
            Add Feature
          </button>
        </div>
        
        <div style={{ marginTop: '20px' }}>
          <button type="submit" disabled={isLoading}>
            {isLoading ? 'Predicting...' : 'Make Prediction'}
          </button>
        </div>
      </form>
      
      {result && (
        <div className="prediction-result">
          <h3>Prediction Result</h3>
          <p><strong>Class:</strong> {getIrisClassName(result.prediction)} ({result.prediction})</p>
          <p><strong>Confidence:</strong> {(result.confidence * 100).toFixed(2)}%</p>
        </div>
      )}
      
      <div style={{ marginTop: '20px', fontSize: '0.9em', fontStyle: 'italic' }}>
        <p><strong>Sample Iris Dataset Values:</strong></p>
        <p>Setosa: [5.1, 3.5, 1.4, 0.2]</p>
        <p>Versicolor: [6.0, 2.9, 4.5, 1.5]</p>
        <p>Virginica: [6.7, 3.1, 5.6, 2.4]</p>
      </div>
    </div>
  );
}

export default Prediction;
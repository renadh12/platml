import React, { useState } from 'react';

function ModelRegistration({ onModelCreated }) {
  const [modelData, setModelData] = useState({
    name: '',
    version: '1.0.0'
  });
  const [status, setStatus] = useState({ message: '', type: '' });
  const [isLoading, setIsLoading] = useState(false);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setModelData(prevData => ({
      ...prevData,
      [name]: value
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    setStatus({ message: '', type: '' });

    try {
      const response = await fetch('http://34.135.45.185/models', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(modelData),
      });

      const data = await response.json();
      
      if (response.ok) {
        setStatus({
          message: `Model registered successfully with ID: ${data.id}`,
          type: 'success'
        });
        setModelData({ name: '', version: '1.0.0' });
        if (onModelCreated) onModelCreated(data);
      } else {
        setStatus({
          message: `Failed to register model: ${data.message || 'Unknown error'}`,
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
      <h2>Register New Model</h2>
      
      {status.message && (
        <div className={`alert alert-${status.type}`}>
          {status.message}
        </div>
      )}
      
      <form onSubmit={handleSubmit}>
        <div>
          <label htmlFor="name">Model Name:</label>
          <input
            type="text"
            id="name"
            name="name"
            value={modelData.name}
            onChange={handleChange}
            required
            placeholder="Enter model name"
          />
        </div>
        
        <div>
          <label htmlFor="version">Version:</label>
          <input
            type="text"
            id="version"
            name="version"
            value={modelData.version}
            onChange={handleChange}
            required
            placeholder="1.0.0"
          />
        </div>
        
        <button type="submit" disabled={isLoading}>
          {isLoading ? 'Registering...' : 'Register Model'}
        </button>
      </form>
    </div>
  );
}

export default ModelRegistration;
import React, { useState } from 'react';

function ModelList({ models, onSelectModel, refreshModels }) {
  const [selectedId, setSelectedId] = useState(null);
  const [status, setStatus] = useState({ message: '', type: '' });
  const [isLoading, setIsLoading] = useState(false);

  const handleSelect = (model) => {
    setSelectedId(model.id);
    if (onSelectModel) onSelectModel(model);
  };

  const handleDelete = async (id, e) => {
    e.stopPropagation(); // Prevent row selection when clicking delete
    
    if (!window.confirm('Are you sure you want to delete this model?')) {
      return;
    }
    
    setIsLoading(true);
    
    try {
      const response = await fetch(`http://localhost:8081/models/${id}`, {
        method: 'DELETE'
      });
      
      if (response.ok) {
        setStatus({
          message: 'Model deleted successfully',
          type: 'success'
        });
        refreshModels();
      } else {
        const errorData = await response.json();
        setStatus({
          message: `Failed to delete model: ${errorData.message || 'Unknown error'}`,
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
      <h2>Available Models</h2>
      
      {status.message && (
        <div className={`alert alert-${status.type}`}>
          {status.message}
        </div>
      )}
      
      {models.length === 0 ? (
        <p>No models available. Click "Register Model" to create one.</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Version</th>
              <th>Status</th>
              <th>Created At</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {models.map(model => (
              <tr 
                key={model.id} 
                onClick={() => handleSelect(model)}
                className={selectedId === model.id ? 'selected' : ''}
                style={{ cursor: 'pointer' }}
              >
                <td>{model.id}</td>
                <td>{model.name}</td>
                <td>{model.version}</td>
                <td>{model.status}</td>
                <td>{new Date(model.created_at).toLocaleString()}</td>
                <td>
                  <button 
                    className="secondary"
                    onClick={(e) => handleDelete(model.id, e)}
                    disabled={isLoading}
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      
      <button onClick={refreshModels} disabled={isLoading}>
        Refresh List
      </button>
    </div>
  );
}

export default ModelList;
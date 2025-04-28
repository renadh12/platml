import React, { useState, useEffect } from 'react';
import './App.css';
import ModelRegistration from './components/ModelRegistration';
import ModelList from './components/ModelList';
import ModelUpload from './components/ModelUpload';
import ModelServing from './components/ModelServing';
import Prediction from './components/Prediction';

function App() {
  const [models, setModels] = useState([]);
  const [selectedModel, setSelectedModel] = useState(null);
  const [loadedModel, setLoadedModel] = useState(null);
  const [activeTab, setActiveTab] = useState('models');

  // Fetch models on component mount
  useEffect(() => {
    fetchModels();
  }, []);

  const fetchModels = async () => {
    try {
      const response = await fetch('http://34.135.45.185/models');
      const data = await response.json();
      console.log("Fetched models data:", data); // Debug output
      
      // Check if data is already an array, otherwise use empty array
      const modelArray = Array.isArray(data) ? data : [];
      setModels(modelArray);
    } catch (error) {
      console.error('Error fetching models:', error);
    }
  };

  const handleModelCreated = () => {
    fetchModels();
    setActiveTab('models');
  };

  const handleSelectModel = (model) => {
    setSelectedModel(model);
    setActiveTab('upload');
  };

  const handleModelUploaded = () => {
    fetchModels();
    setActiveTab('serving');
  };

  const handleModelLoaded = (model) => {
    setLoadedModel(model);
    setActiveTab('predict');
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>ML Platform</h1>
        <nav>
          <button 
            className={activeTab === 'models' ? 'active' : ''} 
            onClick={() => setActiveTab('models')}
          >
            Models
          </button>
          <button 
            className={activeTab === 'register' ? 'active' : ''} 
            onClick={() => setActiveTab('register')}
          >
            Register Model
          </button>
          {selectedModel && (
            <button 
              className={activeTab === 'upload' ? 'active' : ''} 
              onClick={() => setActiveTab('upload')}
            >
              Upload Model
            </button>
          )}
          {selectedModel && (
            <button 
              className={activeTab === 'serving' ? 'active' : ''} 
              onClick={() => setActiveTab('serving')}
            >
              Serve Model
            </button>
          )}
          {loadedModel && (
            <button 
              className={activeTab === 'predict' ? 'active' : ''} 
              onClick={() => setActiveTab('predict')}
            >
              Predict
            </button>
          )}
        </nav>
      </header>

      <main className="App-content">
        {activeTab === 'models' && (
          <ModelList 
            models={models} 
            onSelectModel={handleSelectModel} 
            refreshModels={fetchModels}
          />
        )}
        
        {activeTab === 'register' && (
          <ModelRegistration onModelCreated={handleModelCreated} />
        )}
        
        {activeTab === 'upload' && selectedModel && (
          <ModelUpload 
            model={selectedModel} 
            onModelUploaded={handleModelUploaded} 
          />
        )}
        
        {activeTab === 'serving' && selectedModel && (
          <ModelServing 
            model={selectedModel} 
            onModelLoaded={handleModelLoaded} 
          />
        )}
        
        {activeTab === 'predict' && loadedModel && (
          <Prediction model={loadedModel} />
        )}
      </main>
    </div>
  );
}

export default App;
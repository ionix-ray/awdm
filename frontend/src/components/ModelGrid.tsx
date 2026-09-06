import React from 'react';
import { useModelStore, selectFilteredModels } from '@store/modelStore';
import { ModelCard } from './ModelCard';

export const ModelGrid: React.FC = () => {
  const { loading, error } = useModelStore();
  const filteredModels = selectFilteredModels();

  if (loading) {
    return (
      <div
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          padding: '4rem',
        }}
      >
        <div className="cds--loading" style={{ width: '48px', height: '48px' }}>
          <svg viewBox="0 0 48 48" className="cds--loading__stroke">
            <circle cx="24" cy="24" r="18" fill="none" stroke="#0f62fe" strokeWidth="4" />
          </svg>
        </div>
        <span style={{ marginLeft: '1rem', fontSize: '1rem', color: '#393939' }}>
          Loading models...
        </span>
      </div>
    );
  }

  if (error) {
    return (
      <div
        style={{
          background: '#fff1f1',
          border: '1px solid #da1e28',
          borderRadius: '8px',
          padding: '1.5rem',
          textAlign: 'center',
          color: '#da1e28',
        }}
      >
        <h3 style={{ margin: '0 0 0.5rem' }}>Error Loading Data</h3>
        <p style={{ margin: 0 }}>{error}</p>
      </div>
    );
  }

  if (filteredModels.length === 0) {
    return (
      <div
        style={{
          background: '#f4f4f4',
          borderRadius: '8px',
          padding: '3rem',
          textAlign: 'center',
        }}
      >
        <h3 style={{ margin: '0 0 0.5rem', color: '#393939' }}>No models found</h3>
        <p style={{ margin: 0, color: '#6f6f6f' }}>
          Try adjusting your filters or search query
        </p>
      </div>
    );
  }

  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(auto-fill, minmax(350px, 1fr))',
        gap: '1.5rem',
      }}
    >
      {filteredModels.map((model) => (
        <ModelCard key={model.modelId} model={model} />
      ))}
    </div>
  );
};

import React from 'react';
import { CheckmarkOutline, WarningAlt, Information } from '@carbon/icons-react';
import type { Model } from '@types';
import { 
  formatDate, 
  formatRelativeDate, 
  getPriorityColor, 
  getPriorityLabel,
  getConfidenceLevel,
  getConfidenceColor,
} from '@utils/helpers';

interface ModelCardProps {
  model: Model;
}

export const ModelCard: React.FC<ModelCardProps> = ({ model }) => {
  const confidenceLevel = getConfidenceLevel(model.validationFlags.confidenceScore);
  const confidenceColor = getConfidenceColor(model.validationFlags.confidenceScore);

  return (
    <article
      className="model-card"
      style={{
        background: '#ffffff',
        borderRadius: '12px',
        padding: '1.5rem',
        boxShadow: '0 2px 8px rgba(0,0,0,0.08)',
        transition: 'transform 0.2s, box-shadow 0.2s',
        display: 'flex',
        flexDirection: 'column',
        gap: '1rem',
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.transform = 'translateY(-4px)';
        e.currentTarget.style.boxShadow = '0 4px 16px rgba(0,0,0,0.12)';
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.transform = 'translateY(0)';
        e.currentTarget.style.boxShadow = '0 2px 8px rgba(0,0,0,0.08)';
      }}
    >
      {/* Header */}
      <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div style={{ flex: 1 }}>
          <h3 
            style={{ 
              margin: '0 0 0.25rem', 
              fontSize: '1.25rem', 
              fontWeight: 600,
              color: '#161616',
            }}
          >
            {model.name}
          </h3>
          <p 
            style={{ 
              margin: 0, 
              fontSize: '0.875rem', 
              color: '#6f6f6f',
              textTransform: 'capitalize',
            }}
          >
            by {model.provider}
          </p>
        </div>
        <span
          style={{
            background: `var(--cds-${getPriorityColor(model.priority)}, #ffebd9)`,
            color: model.priority === 'critical' ? '#da1e28' : 
                   model.priority === 'high' ? '#ba4e00' : 
                   model.priority === 'medium' ? '#f1c21b' : '#198038',
            padding: '0.25rem 0.75rem',
            borderRadius: '12px',
            fontSize: '0.75rem',
            fontWeight: 600,
            textTransform: 'uppercase',
          }}
        >
          {getPriorityLabel(model.priority)}
        </span>
      </header>

      {/* Release Info */}
      <div style={{ display: 'flex', gap: '1rem', fontSize: '0.75rem', color: '#6f6f6f' }}>
        <span>Released: {formatDate(model.releaseDate)}</span>
        <span>•</span>
        <span>{formatRelativeDate(model.releaseDate)}</span>
      </div>

      {/* Badges */}
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
        {(model.isFree || model.freeTier.available) && (
          <span
            style={{
              background: '#defbe6',
              color: '#198038',
              padding: '0.25rem 0.75rem',
              borderRadius: '8px',
              fontSize: '0.75rem',
              fontWeight: 500,
              display: 'flex',
              alignItems: 'center',
              gap: '0.25rem',
            }}
          >
            <CheckmarkOutline size={12} />
            Free Tier
          </span>
        )}
        {model.availability.openWeights && (
          <span
            style={{
              background: '#edf5ff',
              color: '#0f62fe',
              padding: '0.25rem 0.75rem',
              borderRadius: '8px',
              fontSize: '0.75rem',
              fontWeight: 500,
              display: 'flex',
              alignItems: 'center',
              gap: '0.25rem',
            }}
          >
            <Information size={12} />
            Open Weights
          </span>
        )}
        {model.validationFlags.communityVerified && (
          <span
            style={{
              background: '#f0f0f0',
              color: '#393939',
              padding: '0.25rem 0.75rem',
              borderRadius: '8px',
              fontSize: '0.75rem',
              fontWeight: 500,
              display: 'flex',
              alignItems: 'center',
              gap: '0.25rem',
            }}
          >
            <CheckmarkOutline size={12} />
            Community Verified
          </span>
        )}
      </div>

      {/* Specifications */}
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.75rem', fontSize: '0.875rem' }}>
        <div>
          <span style={{ color: '#6f6f6f', display: 'block', marginBottom: '0.25rem' }}>Architecture</span>
          <span style={{ fontWeight: 500, color: '#161616' }}>{model.specifications.architecture}</span>
        </div>
        <div>
          <span style={{ color: '#6f6f6f', display: 'block', marginBottom: '0.25rem' }}>Modality</span>
          <span style={{ fontWeight: 500, color: '#161616' }}>
            {model.specifications.modality.join(', ')}
          </span>
        </div>
        <div>
          <span style={{ color: '#6f6f6f', display: 'block', marginBottom: '0.25rem' }}>Parameters</span>
          <span style={{ fontWeight: 500, color: '#161616' }}>{model.specifications.parameters}</span>
        </div>
        <div>
          <span style={{ color: '#6f6f6f', display: 'block', marginBottom: '0.25rem' }}>License</span>
          <span style={{ fontWeight: 500, color: '#161616' }}>
            {model.availability.license || 'Unknown'}
          </span>
        </div>
      </div>

      {/* Categories */}
      {model.categories.length > 0 && (
        <div>
          <span style={{ color: '#6f6f6f', fontSize: '0.75rem', display: 'block', marginBottom: '0.5rem' }}>
            Categories
          </span>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
            {model.categories.map((cat) => (
              <span
                key={cat}
                style={{
                  background: '#f4f4f4',
                  color: '#393939',
                  padding: '0.25rem 0.5rem',
                  borderRadius: '4px',
                  fontSize: '0.75rem',
                  textTransform: 'capitalize',
                }}
              >
                {cat}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Confidence Score */}
      <div 
        style={{
          background: '#f8f8f8',
          borderRadius: '8px',
          padding: '0.75rem',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
      >
        <span style={{ fontSize: '0.875rem', color: '#6f6f6f' }}>Confidence Score</span>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <div
            style={{
              width: '100px',
              height: '8px',
              background: '#e0e0e0',
              borderRadius: '4px',
              overflow: 'hidden',
            }}
          >
            <div
              style={{
                width: `${model.validationFlags.confidenceScore}%`,
                height: '100%',
                background: `var(--cds-${confidenceColor}, #0f62fe)`,
                borderRadius: '4px',
                transition: 'width 0.3s',
              }}
            />
          </div>
          <span 
            style={{ 
              fontSize: '0.875rem', 
              fontWeight: 600,
              color: `var(--cds-${confidenceColor}, #0f62fe)`,
            }}
          >
            {model.validationFlags.confidenceScore}%
          </span>
        </div>
      </div>

      {/* Anomaly Warning */}
      {model.validationFlags.anomalyDetected && (
        <div
          style={{
            background: '#fff1f1',
            border: '1px solid #da1e28',
            borderRadius: '8px',
            padding: '0.75rem',
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
            fontSize: '0.75rem',
            color: '#da1e28',
          }}
        >
          <WarningAlt size={16} />
          <span>Anomaly detected in data validation</span>
        </div>
      )}

      {/* Sources */}
      {model.sources.length > 0 && (
        <div style={{ fontSize: '0.75rem', color: '#6f6f6f' }}>
          Sources: {model.sources.join(', ')}
        </div>
      )}
    </article>
  );
};

import React from 'react';
import { 
  SearchBar, 
  FilterPanel, 
  ModelGrid, 
  Pagination, 
  StatsBar 
} from '@components';
import { useModelData } from '@hooks';

const App: React.FC = () => {
  const { loading, error } = useModelData();

  return (
    <div className="app">
      <header 
        style={{
          background: 'linear-gradient(135deg, #0f62fe 0%, #0043ce 100%)',
          color: 'white',
          padding: '2rem 0',
          marginBottom: '2rem',
        }}
      >
        <div 
          style={{
            maxWidth: '1400px',
            margin: '0 auto',
            padding: '0 2rem',
          }}
        >
          <h1 
            style={{
              margin: '0 0 0.5rem',
              fontSize: '2rem',
              fontWeight: 700,
            }}
          >
            AI Model Intelligence Dashboard
          </h1>
          <p 
            style={{
              margin: 0,
              fontSize: '1rem',
              opacity: 0.9,
              maxWidth: '600px',
            }}
          >
            Real-time tracking of AI model releases, free tiers, and updates for developers and researchers
          </p>
        </div>
      </header>

      <main
        style={{
          maxWidth: '1400px',
          margin: '0 auto',
          padding: '0 2rem 4rem',
        }}
      >
        <StatsBar />
        
        <div 
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginBottom: '1.5rem',
            flexWrap: 'wrap',
            gap: '1rem',
          }}
        >
          <h2 
            style={{
              margin: 0,
              fontSize: '1.5rem',
              fontWeight: 600,
              color: '#161616',
            }}
          >
            Browse Models
          </h2>
          <SearchBar />
        </div>

        <FilterPanel />
        
        <ModelGrid />
        
        <Pagination />
      </main>

      <footer
        style={{
          background: '#161616',
          color: '#ffffff',
          padding: '2rem',
          marginTop: '4rem',
        }}
      >
        <div
          style={{
            maxWidth: '1400px',
            margin: '0 auto',
            textAlign: 'center',
          }}
        >
          <p style={{ margin: '0 0 0.5rem', fontSize: '0.875rem' }}>
            AI Model Intelligence Dashboard
          </p>
          <p 
            style={{ 
              margin: 0, 
              fontSize: '0.75rem', 
              opacity: 0.7,
            }}
          >
            Built with React, TypeScript, and Carbon Design System
          </p>
          <p 
            style={{ 
              margin: '0.5rem 0 0', 
              fontSize: '0.75rem', 
              opacity: 0.5,
            }}
          >
            Data updated via GitHub Actions • Hosted on GitHub Pages
          </p>
        </div>
      </footer>
    </div>
  );
};

export default App;

import React, { useEffect } from 'react';
import { createRoot } from 'react-dom/client';
import '@carbon/styles/index.scss';
import './styles/App.scss';
import App from './App';

const container = document.getElementById('root');
if (!container) throw new Error('Root element not found');

const root = createRoot(container);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

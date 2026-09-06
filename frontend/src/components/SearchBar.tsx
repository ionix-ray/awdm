import React from 'react';
import { Search, Filter, Reset } from '@carbon/icons-react';
import { useDebounce } from '@hooks';
import { useModelStore } from '@store/modelStore';

interface SearchBarProps {
  placeholder?: string;
}

export const SearchBar: React.FC<SearchBarProps> = ({ placeholder = 'Search models...' }) => {
  const [localValue, setLocalValue] = React.useState('');
  const debouncedValue = useDebounce(localValue, 300);
  const { updateFilters } = useModelStore();

  React.useEffect(() => {
    updateFilters({ search: debouncedValue });
  }, [debouncedValue, updateFilters]);

  const handleClear = () => {
    setLocalValue('');
    updateFilters({ search: '' });
  };

  return (
    <div className="search-bar-container" style={{ position: 'relative', width: '100%', maxWidth: '600px' }}>
      <div 
        className="cds--search cds--search--lg" 
        style={{ 
          background: '#f4f4f4',
          borderRadius: '8px',
          display: 'flex',
          alignItems: 'center',
          padding: '0 1rem',
        }}
      >
        <Search 
          size={20} 
          style={{ marginRight: '0.5rem', color: '#393939' }} 
        />
        <input
          className="cds--search-input"
          type="text"
          value={localValue}
          onChange={(e) => setLocalValue(e.target.value)}
          placeholder={placeholder}
          aria-label="Search models"
          style={{
            border: 'none',
            background: 'transparent',
            flex: 1,
            padding: '1rem 0',
            fontSize: '1rem',
            outline: 'none',
          }}
        />
        {localValue && (
          <button
            onClick={handleClear}
            aria-label="Clear search"
            style={{
              background: 'none',
              border: 'none',
              cursor: 'pointer',
              padding: '0.5rem',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <Reset size={16} style={{ color: '#393939' }} />
          </button>
        )}
      </div>
    </div>
  );
};

import React from 'react';
import { Filter as FilterIcon, Reset } from '@carbon/icons-react';
import { useModelStore, selectUniqueProviders, selectUniqueCategories } from '@store/modelStore';

export const FilterPanel: React.FC = () => {
  const { filters, updateFilters, resetFilters } = useModelStore();
  const providers = selectUniqueProviders();
  const categories = selectUniqueCategories();
  const [isExpanded, setIsExpanded] = React.useState(true);

  const toggleProvider = (provider: string) => {
    const current = filters.providers;
    const updated = current.includes(provider)
      ? current.filter((p) => p !== provider)
      : [...current, provider];
    updateFilters({ providers: updated });
  };

  const toggleCategory = (category: string) => {
    const current = filters.categories;
    const updated = current.includes(category)
      ? current.filter((c) => c !== category)
      : [...current, category];
    updateFilters({ categories: updated });
  };

  return (
    <div 
      className="filter-panel"
      style={{
        background: '#ffffff',
        borderRadius: '12px',
        padding: '1.5rem',
        marginBottom: '1.5rem',
        boxShadow: '0 2px 6px rgba(0,0,0,0.1)',
      }}
    >
      <div 
        style={{ 
          display: 'flex', 
          justifyContent: 'space-between', 
          alignItems: 'center',
          marginBottom: '1rem',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <FilterIcon size={20} />
          <h3 style={{ margin: 0, fontSize: '1.125rem', fontWeight: 600 }}>Filters</h3>
        </div>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            style={{
              background: 'none',
              border: '1px solid #8d8d8d',
              borderRadius: '4px',
              padding: '0.5rem 1rem',
              cursor: 'pointer',
              fontSize: '0.875rem',
            }}
          >
            {isExpanded ? 'Collapse' : 'Expand'}
          </button>
          <button
            onClick={resetFilters}
            style={{
              background: '#f4f4f4',
              border: 'none',
              borderRadius: '4px',
              padding: '0.5rem 1rem',
              cursor: 'pointer',
              fontSize: '0.875rem',
              display: 'flex',
              alignItems: 'center',
              gap: '0.25rem',
            }}
          >
            <Reset size={16} />
            Reset
          </button>
        </div>
      </div>

      {isExpanded && (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '1.5rem' }}>
          {/* Providers */}
          <div>
            <h4 style={{ margin: '0 0 0.75rem', fontSize: '0.875rem', fontWeight: 600, color: '#393939' }}>
              Providers
            </h4>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              {providers.map((provider) => (
                <label
                  key={provider}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '0.5rem',
                    cursor: 'pointer',
                    fontSize: '0.875rem',
                  }}
                >
                  <input
                    type="checkbox"
                    checked={filters.providers.includes(provider)}
                    onChange={() => toggleProvider(provider)}
                    style={{
                      width: '16px',
                      height: '16px',
                      accentColor: '#0f62fe',
                    }}
                  />
                  <span style={{ textTransform: 'capitalize' }}>{provider}</span>
                </label>
              ))}
            </div>
          </div>

          {/* Categories */}
          <div>
            <h4 style={{ margin: '0 0 0.75rem', fontSize: '0.875rem', fontWeight: 600, color: '#393939' }}>
              Categories
            </h4>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              {categories.map((category) => (
                <label
                  key={category}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '0.5rem',
                    cursor: 'pointer',
                    fontSize: '0.875rem',
                  }}
                >
                  <input
                    type="checkbox"
                    checked={filters.categories.includes(category)}
                    onChange={() => toggleCategory(category)}
                    style={{
                      width: '16px',
                      height: '16px',
                      accentColor: '#0f62fe',
                    }}
                  />
                  <span style={{ textTransform: 'capitalize' }}>{category}</span>
                </label>
              ))}
            </div>
          </div>

          {/* Quick Filters */}
          <div>
            <h4 style={{ margin: '0 0 0.75rem', fontSize: '0.875rem', fontWeight: 600, color: '#393939' }}>
              Quick Filters
            </h4>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.5rem',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                }}
              >
                <input
                  type="checkbox"
                  checked={filters.freeOnly}
                  onChange={(e) => updateFilters({ freeOnly: e.target.checked })}
                  style={{
                    width: '16px',
                    height: '16px',
                    accentColor: '#0f62fe',
                  }}
                />
                <span>Free models only</span>
              </label>
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.5rem',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                }}
              >
                <input
                  type="checkbox"
                  checked={filters.openWeightsOnly}
                  onChange={(e) => updateFilters({ openWeightsOnly: e.target.checked })}
                  style={{
                    width: '16px',
                    height: '16px',
                    accentColor: '#0f62fe',
                  }}
                />
                <span>Open weights only</span>
              </label>
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.5rem',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                }}
              >
                <input
                  type="checkbox"
                  checked={filters.newReleasesOnly}
                  onChange={(e) => updateFilters({ newReleasesOnly: e.target.checked })}
                  style={{
                    width: '16px',
                    height: '16px',
                    accentColor: '#0f62fe',
                  }}
                />
                <span>New releases (30 days)</span>
              </label>
            </div>
          </div>

          {/* Sorting */}
          <div>
            <h4 style={{ margin: '0 0 0.75rem', fontSize: '0.875rem', fontWeight: 600, color: '#393939' }}>
              Sort By
            </h4>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              <select
                value={filters.sortBy}
                onChange={(e) => updateFilters({ sortBy: e.target.value as any })}
                style={{
                  padding: '0.5rem',
                  borderRadius: '4px',
                  border: '1px solid #8d8d8d',
                  fontSize: '0.875rem',
                  background: 'white',
                }}
              >
                <option value="relevance">Relevance</option>
                <option value="releaseDate">Release Date</option>
                <option value="confidence">Confidence Score</option>
                <option value="name">Name (A-Z)</option>
              </select>
              <select
                value={filters.sortOrder}
                onChange={(e) => updateFilters({ sortOrder: e.target.value as any })}
                style={{
                  padding: '0.5rem',
                  borderRadius: '4px',
                  border: '1px solid #8d8d8d',
                  fontSize: '0.875rem',
                  background: 'white',
                }}
              >
                <option value="desc">Descending</option>
                <option value="asc">Ascending</option>
              </select>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

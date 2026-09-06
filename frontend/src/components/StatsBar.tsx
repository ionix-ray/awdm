import React from 'react';
import { useModelStore } from '@store/modelStore';
import { Information, CheckmarkFilled, TrendingUp } from '@carbon/icons-react';

export const StatsBar: React.FC = () => {
  const { data } = useModelStore();

  if (!data) return null;

  const stats = [
    {
      label: 'Total Models',
      value: data.summary.totalModels,
      icon: Information,
      color: '#0f62fe',
      background: '#edf5ff',
    },
    {
      label: 'Free Models',
      value: data.summary.freeModels,
      icon: CheckmarkFilled,
      color: '#198038',
      background: '#defbe6',
    },
    {
      label: 'New (7 days)',
      value: data.summary.newReleasesLast7Days,
      icon: TrendingUp,
      color: '#ba4e00',
      background: '#ffebd9',
    },
    {
      label: 'Providers',
      value: data.summary.uniqueProviders,
      icon: Information,
      color: '#8a3ffc',
      background: '#f3efff',
    },
    {
      label: 'High Confidence',
      value: data.summary.highConfidenceModels,
      icon: CheckmarkFilled,
      color: '#0f62fe',
      background: '#edf5ff',
    },
  ];

  return (
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))',
        gap: '1rem',
        marginBottom: '2rem',
      }}
    >
      {stats.map((stat, index) => {
        const Icon = stat.icon;
        return (
          <div
            key={index}
            style={{
              background: stat.background,
              borderRadius: '12px',
              padding: '1.25rem',
              display: 'flex',
              flexDirection: 'column',
              gap: '0.5rem',
              transition: 'transform 0.2s',
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.transform = 'translateY(-2px)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.transform = 'translateY(0)';
            }}
          >
            <div
              style={{
                width: '32px',
                height: '32px',
                background: stat.color,
                borderRadius: '8px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Icon size={20} style={{ color: 'white' }} />
            </div>
            <div>
              <div
                style={{
                  fontSize: '1.75rem',
                  fontWeight: 700,
                  color: '#161616',
                  lineHeight: 1,
                }}
              >
                {stat.value}
              </div>
              <div
                style={{
                  fontSize: '0.75rem',
                  color: '#6f6f6f',
                  marginTop: '0.25rem',
                }}
              >
                {stat.label}
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
};

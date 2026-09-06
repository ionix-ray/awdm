import React from 'react';
import { ChevronLeft, ChevronRight } from '@carbon/icons-react';
import { usePagination } from '@hooks';

export const Pagination: React.FC = () => {
  const {
    currentPage,
    totalPages,
    goToPage,
    nextPage,
    prevPage,
    canGoNext,
    canGoPrev,
  } = usePagination();

  if (totalPages <= 1) return null;

  const getPageNumbers = () => {
    const pages: (number | string)[] = [];
    const maxVisible = 5;

    if (totalPages <= maxVisible) {
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      if (currentPage <= 3) {
        for (let i = 1; i <= 4; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      } else if (currentPage >= totalPages - 2) {
        pages.push(1);
        pages.push('...');
        for (let i = totalPages - 3; i <= totalPages; i++) pages.push(i);
      } else {
        pages.push(1);
        pages.push('...');
        for (let i = currentPage - 1; i <= currentPage + 1; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      }
    }

    return pages;
  };

  return (
    <nav
      className="pagination"
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        gap: '0.5rem',
        padding: '2rem 0',
        marginTop: '1rem',
      }}
      aria-label="Pagination navigation"
    >
      <button
        onClick={prevPage}
        disabled={!canGoPrev}
        aria-label="Previous page"
        style={{
          background: canGoPrev ? '#ffffff' : '#f4f4f4',
          border: '1px solid #8d8d8d',
          borderRadius: '8px',
          padding: '0.75rem',
          cursor: canGoPrev ? 'pointer' : 'not-allowed',
          opacity: canGoPrev ? 1 : 0.5,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          transition: 'all 0.2s',
        }}
        onMouseEnter={(e) => {
          if (canGoPrev) {
            e.currentTarget.style.background = '#f4f4f4';
          }
        }}
        onMouseLeave={(e) => {
          if (canGoPrev) {
            e.currentTarget.style.background = '#ffffff';
          }
        }}
      >
        <ChevronLeft size={20} />
      </button>

      <div style={{ display: 'flex', gap: '0.25rem' }}>
        {getPageNumbers().map((page, index) =>
          typeof page === 'number' ? (
            <button
              key={index}
              onClick={() => goToPage(page)}
              aria-label={`Page ${page}`}
              aria-current={page === currentPage ? 'page' : undefined}
              style={{
                background: page === currentPage ? '#0f62fe' : '#ffffff',
                color: page === currentPage ? '#ffffff' : '#161616',
                border: '1px solid #8d8d8d',
                borderRadius: '8px',
                minWidth: '40px',
                height: '40px',
                padding: '0.5rem',
                cursor: 'pointer',
                fontWeight: page === currentPage ? 600 : 400,
                transition: 'all 0.2s',
              }}
              onMouseEnter={(e) => {
                if (page !== currentPage) {
                  e.currentTarget.style.background = '#f4f4f4';
                }
              }}
              onMouseLeave={(e) => {
                if (page !== currentPage) {
                  e.currentTarget.style.background = '#ffffff';
                }
              }}
            >
              {page}
            </button>
          ) : (
            <span
              key={index}
              style={{
                display: 'flex',
                alignItems: 'center',
                padding: '0 0.5rem',
                color: '#6f6f6f',
              }}
            >
              {page}
            </span>
          )
        )}
      </div>

      <button
        onClick={nextPage}
        disabled={!canGoNext}
        aria-label="Next page"
        style={{
          background: canGoNext ? '#ffffff' : '#f4f4f4',
          border: '1px solid #8d8d8d',
          borderRadius: '8px',
          padding: '0.75rem',
          cursor: canGoNext ? 'pointer' : 'not-allowed',
          opacity: canGoNext ? 1 : 0.5,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          transition: 'all 0.2s',
        }}
        onMouseEnter={(e) => {
          if (canGoNext) {
            e.currentTarget.style.background = '#f4f4f4';
          }
        }}
        onMouseLeave={(e) => {
          if (canGoNext) {
            e.currentTarget.style.background = '#ffffff';
          }
        }}
      >
        <ChevronRight size={20} />
      </button>
    </nav>
  );
};

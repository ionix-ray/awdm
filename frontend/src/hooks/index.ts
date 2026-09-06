import { useEffect, useCallback, useState } from 'react';
import { useModelStore, selectFilteredModels, selectPaginatedModels, selectTotalPages } from '@store/modelStore';

export const useDebounce = <T,>(value: T, delay: number): T => {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
};

export const useModelData = () => {
  const { fetchModelData, loading, error, data } = useModelStore();

  useEffect(() => {
    fetchModelData();
  }, [fetchModelData]);

  return { loading, error, data };
};

export const useFilteredModels = () => {
  return selectFilteredModels();
};

export const usePaginatedModels = () => {
  const models = selectPaginatedModels();
  const totalPages = selectTotalPages();
  return { models, totalPages };
};

export const usePagination = () => {
  const { currentPage, itemsPerPage, setCurrentPage } = useModelStore();
  const totalPages = selectTotalPages();

  const goToPage = useCallback((page: number) => {
    const clampedPage = Math.max(1, Math.min(page, totalPages));
    setCurrentPage(clampedPage);
  }, [totalPages, setCurrentPage]);

  const nextPage = useCallback(() => {
    goToPage(currentPage + 1);
  }, [currentPage, goToPage]);

  const prevPage = useCallback(() => {
    goToPage(currentPage - 1);
  }, [currentPage, goToPage]);

  const canGoNext = currentPage < totalPages;
  const canGoPrev = currentPage > 1;

  return {
    currentPage,
    totalPages,
    itemsPerPage,
    goToPage,
    nextPage,
    prevPage,
    canGoNext,
    canGoPrev,
  };
};

import { create } from 'zustand';
import type { ModelData, FilterState, Model } from '@types';

interface ModelState {
  data: ModelData | null;
  loading: boolean;
  error: string | null;
  filters: FilterState;
  currentPage: number;
  itemsPerPage: number;
  
  // Actions
  setData: (data: ModelData) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  updateFilters: (filters: Partial<FilterState>) => void;
  resetFilters: () => void;
  setCurrentPage: (page: number) => void;
  fetchModelData: () => Promise<void>;
}

const defaultFilters: FilterState = {
  search: '',
  providers: [],
  categories: [],
  freeOnly: false,
  openWeightsOnly: false,
  newReleasesOnly: false,
  minConfidence: 0,
  sortBy: 'relevance',
  sortOrder: 'desc',
};

export const useModelStore = create<ModelState>((set, get) => ({
  data: null,
  loading: true,
  error: null,
  filters: defaultFilters,
  currentPage: 1,
  itemsPerPage: 12,

  setData: (data) => set({ data, loading: false, error: null }),
  
  setLoading: (loading) => set({ loading }),
  
  setError: (error) => set({ error, loading: false }),
  
  updateFilters: (newFilters) => {
    set((state) => ({
      filters: { ...state.filters, ...newFilters },
      currentPage: 1,
    }));
  },
  
  resetFilters: () => {
    set({ filters: defaultFilters, currentPage: 1 });
  },
  
  setCurrentPage: (page) => set({ currentPage: page }),

  fetchModelData: async () => {
    set({ loading: true, error: null });
    try {
      const response = await fetch('/model-status.json');
      if (!response.ok) {
        throw new Error(`Failed to fetch model data: ${response.status}`);
      }
      const data: ModelData = await response.json();
      set({ data, loading: false });
    } catch (err) {
      set({ 
        error: err instanceof Error ? err.message : 'Unknown error occurred',
        loading: false 
      });
    }
  },
}));

// Selector helpers
export const selectFilteredModels = (): Model[] => {
  const state = useModelStore.getState();
  const { data, filters } = state;
  
  if (!data) return [];
  
  let filtered = [...data.models];
  
  // Search filter
  if (filters.search) {
    const searchLower = filters.search.toLowerCase();
    filtered = filtered.filter(
      (model) =>
        model.name.toLowerCase().includes(searchLower) ||
        model.provider.toLowerCase().includes(searchLower) ||
        model.modelId.toLowerCase().includes(searchLower)
    );
  }
  
  // Provider filter
  if (filters.providers.length > 0) {
    filtered = filtered.filter((model) =>
      filters.providers.includes(model.provider)
    );
  }
  
  // Category filter
  if (filters.categories.length > 0) {
    filtered = filtered.filter((model) =>
      filters.categories.some((cat) => model.categories.includes(cat))
    );
  }
  
  // Free tier filter
  if (filters.freeOnly) {
    filtered = filtered.filter((model) => model.isFree || model.freeTier.available);
  }
  
  // Open weights filter
  if (filters.openWeightsOnly) {
    filtered = filtered.filter((model) => model.availability.openWeights);
  }
  
  // New releases filter (last 30 days)
  if (filters.newReleasesOnly) {
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    filtered = filtered.filter(
      (model) => new Date(model.releaseDate) >= thirtyDaysAgo
    );
  }
  
  // Confidence filter
  if (filters.minConfidence > 0) {
    filtered = filtered.filter(
      (model) => model.validationFlags.confidenceScore >= filters.minConfidence
    );
  }
  
  // Sorting
  filtered.sort((a, b) => {
    let comparison = 0;
    
    switch (filters.sortBy) {
      case 'relevance':
        comparison = (b._computed?.relevanceScore || 0) - (a._computed?.relevanceScore || 0);
        break;
      case 'releaseDate':
        comparison = new Date(b.releaseDate).getTime() - new Date(a.releaseDate).getTime();
        break;
      case 'confidence':
        comparison = b.validationFlags.confidenceScore - a.validationFlags.confidenceScore;
        break;
      case 'name':
        comparison = a.name.localeCompare(b.name);
        break;
    }
    
    return filters.sortOrder === 'asc' ? -comparison : comparison;
  });
  
  return filtered;
};

export const selectPaginatedModels = (): Model[] => {
  const filtered = selectFilteredModels();
  const { currentPage, itemsPerPage } = useModelStore.getState();
  const startIndex = (currentPage - 1) * itemsPerPage;
  return filtered.slice(startIndex, startIndex + itemsPerPage);
};

export const selectTotalPages = (): number => {
  const filtered = selectFilteredModels();
  const { itemsPerPage } = useModelStore.getState();
  return Math.ceil(filtered.length / itemsPerPage);
};

export const selectUniqueProviders = (): string[] => {
  const state = useModelStore.getState();
  if (!state.data) return [];
  return Object.keys(state.data.providers);
};

export const selectUniqueCategories = (): string[] => {
  const state = useModelStore.getState();
  if (!state.data) return [];
  return Object.keys(state.data.summary.topCategories);
};

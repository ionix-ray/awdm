export interface FreeTier {
  available: boolean;
  limits: {
    rpm?: number;
    rpd?: number;
    tokens?: number;
  };
  conditions: string;
}

export interface Specifications {
  parameters: string;
  contextWindow: number | null;
  architecture: string;
  modality: string[];
}

export interface Availability {
  hosted: string[];
  openWeights: boolean;
  license: string;
}

export interface Benchmark {
  source: string;
  scores: Record<string, number>;
}

export interface NewsReference {
  title: string;
  url: string;
  source: string;
  publishedAt: string;
  sentiment: 'positive' | 'neutral' | 'negative';
}

export interface ValidationFlags {
  communityVerified: boolean;
  officialAnnouncement: boolean;
  anomalyDetected: boolean;
  confidenceScore: number;
}

export interface ComputedFields {
  relevanceScore: number;
  daysSinceRelease: number;
  isNewRelease: boolean;
  hasFreeTier: boolean;
  generatedAt: string;
}

export interface Model {
  modelId: string;
  name: string;
  provider: string;
  tracked: boolean;
  categories: string[];
  priority: 'low' | 'medium' | 'high' | 'critical';
  releaseDate: string;
  lastCheckedAt: string;
  isFree: boolean;
  freeTier: FreeTier;
  specifications: Specifications;
  availability: Availability;
  benchmarks: Record<string, Benchmark>;
  newsReferences: NewsReference[];
  validationFlags: ValidationFlags;
  sources: string[];
  _computed?: ComputedFields;
}

export interface Summary {
  totalModels: number;
  freeModels: number;
  newReleasesLast7Days: number;
  highConfidenceModels: number;
  uniqueProviders: number;
  topCategories: Record<string, number>;
}

export interface Metadata {
  generatedAt: string;
  version: number;
  schedule: string;
  sources: string[];
}

export interface ModelData {
  metadata: Metadata;
  summary: Summary;
  providers: Record<string, number>;
  models: Model[];
}

export interface FilterState {
  search: string;
  providers: string[];
  categories: string[];
  freeOnly: boolean;
  openWeightsOnly: boolean;
  newReleasesOnly: boolean;
  minConfidence: number;
  sortBy: 'relevance' | 'releaseDate' | 'confidence' | 'name';
  sortOrder: 'asc' | 'desc';
}

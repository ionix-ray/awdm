# AI Model Intelligence Dashboard - Comprehensive Implementation Plan

## Executive Summary

A production-grade, single-page application (SPA) hosted on GitHub Pages that provides real-time intelligence on AI model releases, free tiers, new providers, and updates. Built with a robust CI/CD pipeline using GitHub Actions, featuring automated data fetching, validation, and deployment.

---

## 1. System Architecture

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Actions Pipeline                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ Data Sources │  │ Validation   │  │ JSON Build Generator │  │
│  │ Scraper      │→ │ Engine       │→ │ & PR Creation        │  │
│  └──────────────┘  └──────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Pages Frontend                         │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  React + TypeScript + Carbon DS                          │  │
│  │  - Search & Filter                                       │  │
│  │  - Pagination                                            │  │
│  │  - Responsive Design                                     │  │
│  │  - Microsoft Clarity Analytics                           │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Data Flow

1. **Scheduled Trigger** (Once/Twice Daily via cron)
2. **Data Fetching** from multiple sources:
   - Hugging Face API (primary)
   - GitHub Releases API
   - RSS Feeds (official blogs)
   - Community sources (Reddit API - optional)
3. **Validation Engine**:
   - Schema validation
   - Cross-source verification
   - Optional AI-based fact-checking (via free API)
   - Anomaly detection
4. **JSON Generation** with computed insights
5. **PR Creation** for data changes
6. **Merge & Deploy** to GitHub Pages

---

## 2. Data Sources Strategy

### 2.1 Primary Sources (Free & Open)

| Source | Type | Rate Limit | Authentication |
|--------|------|------------|----------------|
| Hugging Face API | REST | 50 req/hour | None (public) |
| GitHub Releases API | REST | 60 req/hour | None (public) |
| RSS Feeds | XML | None | None |
| LLM Price Tracker | JSON | None | Public GitHub |

### 2.2 Optional Enhanced Sources (Require API Keys)

| Source | Purpose | Free Tier | Notes |
|--------|---------|-----------|-------|
| OpenCode API | Fact validation | Yes | User must provide key |
| Reddit API | Community sentiment | 60 req/min | OAuth required |

### 2.3 Source Priority & Fallback

```
Priority 1: Hugging Face API (authoritative for open weights)
Priority 2: Official Provider Blogs (RSS)
Priority 3: GitHub Releases (for version tracking)
Priority 4: Community Verification (Reddit, Discord summaries)
```

---

## 3. JSON Schema Design

### 3.1 Master Schema (`model-status.json`)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "AI Model Status Dashboard Data",
  "type": "object",
  "required": ["metadata", "summary", "models"],
  "properties": {
    "metadata": {
      "generatedAt": {"type": "string", "format": "date-time"},
      "version": {"type": "integer"},
      "schedule": {"type": "string"},
      "sources": {"type": "array", "items": {"type": "string"}},
      "nextUpdate": {"type": "string", "format": "date-time"}
    },
    "summary": {
      "totalModels": {"type": "integer"},
      "freeModels": {"type": "integer"},
      "newReleasesLast7Days": {"type": "integer"},
      "newReleasesLast30Days": {"type": "integer"},
      "highConfidenceModels": {"type": "integer"},
      "uniqueProviders": {"type": "integer"},
      "topCategories": {"type": "object"},
      "averageConfidenceScore": {"type": "number"},
      "modelsWithFreeTier": {"type": "integer"},
      "openWeightModels": {"type": "integer"}
    },
    "providers": {"type": "object", "additionalProperties": {"type": "integer"}},
    "models": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["modelId", "name", "provider"],
        "properties": {
          "modelId": {"type": "string"},
          "name": {"type": "string"},
          "provider": {"type": "string"},
          "tracked": {"type": "boolean"},
          "categories": {"type": "array", "items": {"type": "string"}},
          "priority": {"type": "string", "enum": ["critical", "high", "medium", "low"]},
          "releaseDate": {"type": "string", "format": "date-time"},
          "lastCheckedAt": {"type": "string", "format": "date-time"},
          "isFree": {"type": "boolean"},
          "freeTier": {
            "type": "object",
            "properties": {
              "available": {"type": "boolean"},
              "limits": {
                "type": "object",
                "properties": {
                  "rpm": {"type": "number"},
                  "rpd": {"type": "number"},
                  "tokens": {"type": "number"}
                }
              },
              "conditions": {"type": "string"}
            }
          },
          "specifications": {
            "type": "object",
            "properties": {
              "parameters": {"type": "string"},
              "contextWindow": {"type": "number"},
              "architecture": {"type": "string"},
              "modality": {"type": "array", "items": {"type": "string"}}
            }
          },
          "benchmarks": {
            "type": "object",
            "properties": {
              "source": {"type": "string"},
              "scores": {"type": "object"}
            }
          },
          "availability": {
            "type": "object",
            "properties": {
              "hosted": {"type": "array", "items": {"type": "string"}},
              "openWeights": {"type": "boolean"},
              "license": {"type": "string"}
            }
          },
          "newsReferences": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "title": {"type": "string"},
                "url": {"type": "string", "format": "uri"},
                "source": {"type": "string"},
                "publishedAt": {"type": "string", "format": "date-time"},
                "sentiment": {"type": "string", "enum": ["positive", "neutral", "negative"]}
              }
            }
          },
          "validationFlags": {
            "type": "object",
            "properties": {
              "communityVerified": {"type": "boolean"},
              "officialAnnouncement": {"type": "boolean"},
              "anomalyDetected": {"type": "boolean"},
              "confidenceScore": {"type": "number", "minimum": 0, "maximum": 100},
              "aiValidated": {"type": "boolean"},
              "validationSource": {"type": "string"}
            }
          },
          "sources": {"type": "array", "items": {"type": "string"}},
          "_computed": {
            "type": "object",
            "properties": {
              "relevanceScore": {"type": "number"},
              "daysSinceRelease": {"type": "number"},
              "isNewRelease": {"type": "boolean"},
              "hasFreeTier": {"type": "boolean"},
              "isOpenWeight": {"type": "boolean"},
              "generatedAt": {"type": "string", "format": "date-time"}
            }
          }
        }
      }
    }
  }
}
```

---

## 4. GitHub Actions Pipeline Design

### 4.1 Workflow Configuration

```yaml
# Key optimizations for cost & efficiency:
# 1. Run once daily (or twice max) - cron: '0 6 * * *' and '0 18 * * *'
# 2. Use caching for API responses
# 3. Incremental updates only when changes detected
# 4. Concurrency control to prevent duplicate runs
# 5. Manual trigger option for testing
```

### 4.2 Job Breakdown

**Job 1: Validate** (2-3 min)
- Checkout repo
- Setup Python
- Install dependencies
- Validate existing registry against schemas

**Job 2: Fetch Data** (5-7 min)
- Fetch from Hugging Face API (with rate limiting)
- Fetch from GitHub Releases
- Parse RSS feeds
- Cache responses
- Update metadata files

**Job 3: Validate & Enrich** (3-4 min)
- Schema validation
- Cross-source verification
- Optional AI fact-checking (if API key provided)
- Anomaly detection
- Compute relevance scores

**Job 4: Generate & Commit** (2-3 min)
- Generate `model-status.json`
- Create PR if changes detected
- Auto-merge if validation passes

**Job 5: Build & Deploy** (3-5 min)
- Build frontend (Vite)
- Deploy to GitHub Pages

**Total Estimated Time**: 15-22 minutes per run
**Daily Cost**: ~30-45 minutes of GitHub Actions usage (well within free tier)

---

## 5. Frontend Architecture

### 5.1 Technology Stack

- **Framework**: React 18 + TypeScript
- **Build Tool**: Vite
- **UI Library**: Carbon Design System (with custom theme)
- **State Management**: Zustand
- **Routing**: React Router (hash-based for GitHub Pages)
- **Styling**: SCSS + Carbon Components
- **Fonts**: IBM Plex Sans, IBM Plex Mono, Astra (via Google Fonts)
- **Analytics**: Microsoft Clarity

### 5.2 Component Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── Layout/
│   │   │   ├── Header.tsx
│   │   │   ├── Footer.tsx
│   │   │   └── MainLayout.tsx
│   │   ├── ModelCard/
│   │   │   ├── ModelCard.tsx
│   │   │   ├── ModelSpecs.tsx
│   │   │   ├── FreeTierBadge.tsx
│   │   │   └── ValidationIndicator.tsx
│   │   ├── SearchFilter/
│   │   │   ├── SearchBar.tsx
│   │   │   ├── FilterPanel.tsx
│   │   │   └── SortDropdown.tsx
│   │   ├── Pagination/
│   │   │   └── Pagination.tsx
│   │   ├── Dashboard/
│   │   │   ├── StatsOverview.tsx
│   │   │   ├── ProviderBreakdown.tsx
│   │   │   └── CategoryDistribution.tsx
│   │   └── common/
│   │       ├── LoadingSpinner.tsx
│   │       ├── ErrorBoundary.tsx
│   │       └── EmptyState.tsx
│   ├── hooks/
│   │   ├── useModels.ts
│   │   ├── useSearch.ts
│   │   ├── useFilter.ts
│   │   └── usePagination.ts
│   ├── store/
│   │   └── modelStore.ts
│   ├── utils/
│   │   ├── formatters.ts
│   │   ├── validators.ts
│   │   └── constants.ts
│   ├── types/
│   │   └── index.ts
│   ├── styles/
│   │   ├── _variables.scss
│   │   ├── _carbon-theme.scss
│   │   └── main.scss
│   ├── App.tsx
│   └── main.tsx
├── public/
│   ├── model-status.json
│   └── index.html
└── vite.config.ts
```

### 5.3 Key Features

1. **Search**: Debounced search across model name, provider, categories
2. **Filters**:
   - Free tier only
   - Open weights only
   - Release date range
   - Provider selection
   - Category selection
   - Confidence score threshold
3. **Sorting**:
   - Relevance score
   - Release date (newest first)
   - Name (A-Z)
   - Provider
4. **Pagination**: Configurable page size (10, 25, 50, 100)
5. **Responsive Design**: Mobile-first, breakpoints at 640px, 1024px, 1280px
6. **Accessibility**: WCAG 2.1 AA compliant, keyboard navigation
7. **Performance**: Lazy loading, code splitting, memoization

---

## 6. Security & Best Practices

### 6.1 Security Measures

1. **No API Keys in Frontend**: All API calls server-side (GitHub Actions)
2. **GitHub Secrets**: Store any API keys in repository secrets
3. **Input Sanitization**: DOMPurify for any user-generated content
4. **Content Security Policy**: Strict CSP headers via GitHub Pages
5. **Dependency Scanning**: Dependabot enabled
6. **No eval() or dangerous HTML rendering**

### 6.2 OWASP Top 10 Mitigation

| Vulnerability | Mitigation Strategy |
|---------------|---------------------|
| A01: Broken Access Control | N/A (static site) |
| A02: Cryptographic Failures | HTTPS enforced by GitHub Pages |
| A03: Injection | No server-side code, input sanitization |
| A04: Insecure Design | Schema validation, data abstraction |
| A05: Security Misconfiguration | Default deny, minimal permissions |
| A06: Vulnerable Components | Dependabot, regular updates |
| A07: Auth Failures | N/A (no authentication) |
| A08: Software Integrity Failures | SHA verification, signed commits |
| A09: Logging Failures | Action logs retained, no sensitive data |
| A10: SSRF | N/A (no server-side requests from frontend) |

### 6.3 Error Handling Strategy

1. **Graceful Degradation**: Show cached data if fetch fails
2. **Partial Data Fallback**: Render available data, show warnings
3. **User-Friendly Messages**: Clear error states, retry options
4. **Maintainer Alerts**: GitHub Issues auto-created on critical failures
5. **Comprehensive Logging**: Structured logs in GitHub Actions

---

## 7. Test Cases (BDD/TDD)

### 7.1 Feature Scenarios (Gherkin)

```gherkin
Feature: Model Discovery
  Scenario: User views latest model releases
    Given the dashboard is loaded
    When the user sorts by "Release Date"
    Then models released in last 7 days appear first
    And each model shows a "New" badge

  Scenario: User filters for free models
    Given the dashboard is loaded
    When the user enables "Free Tier Only" filter
    Then only models with free tier are displayed
    And free tier limits are visible on each card

  Scenario: User searches for specific model
    Given the dashboard is loaded
    When the user types "Llama" in search
    Then models matching "Llama" are shown
    And results are ranked by relevance

  Scenario: GitHub Action completes efficiently
    Given the workflow is triggered
    When the action runs
    Then it completes under 25 minutes
    And API rate limits are not exceeded
    And cache is utilized for repeated requests
```

### 7.2 Unit Tests

```typescript
// Example test cases
describe('ModelCard', () => {
  it('renders model name and provider', () => {});
  it('displays free tier badge when available', () => {});
  it('shows confidence score indicator', () => {});
});

describe('useSearch hook', () => {
  it('filters models by name', () => {});
  it('filters models by provider', () => {});
  it('handles debouncing correctly', () => {});
});

describe('Data Validator', () => {
  it('validates JSON schema', () => {});
  it('detects anomalies in benchmarks', () => {});
  it('flags unverified claims', () => {});
});
```

---

## 8. Implementation Phases

### Phase 1: Foundation (Days 1-2)
- [x] Project structure setup
- [ ] Frontend scaffolding (Vite + React + TS)
- [ ] Carbon DS integration with custom theme
- [ ] Base layout components
- [ ] TypeScript types from JSON schema

### Phase 2: Data Pipeline Enhancement (Days 3-5)
- [ ] Enhance Hugging Face scraper
- [ ] Add GitHub Releases scraper
- [ ] Implement RSS feed parser
- [ ] Add caching mechanism
- [ ] Implement rate limiting

### Phase 3: Validation Engine (Days 6-8)
- [ ] Schema validation (existing)
- [ ] Cross-source verification
- [ ] Optional AI fact-checking integration
- [ ] Anomaly detection rules
- [ ] Confidence scoring algorithm

### Phase 4: Frontend Components (Days 9-12)
- [ ] ModelCard component
- [ ] SearchBar with debouncing
- [ ] FilterPanel with multiple criteria
- [ ] Pagination component
- [ ] Dashboard stats overview
- [ ] Responsive grid layout

### Phase 5: State Management (Days 13-14)
- [ ] Zustand store setup
- [ ] Custom hooks (useModels, useSearch, useFilter, usePagination)
- [ ] URL state synchronization
- [ ] Deep linking support

### Phase 6: Optimization & Polish (Days 15-17)
- [ ] GitHub Actions optimization
- [ ] Incremental builds
- [ ] PWA capabilities
- [ ] Performance optimization (Lighthouse)
- [ ] Accessibility audit
- [ ] Microsoft Clarity integration
- [ ] Error boundaries and fallbacks

### Phase 7: Testing & Documentation (Days 18-20)
- [ ] Unit tests (Vitest)
- [ ] Integration tests
- [ ] E2E tests (Playwright - optional)
- [ ] README documentation
- [ ] Contributing guidelines
- [ ] API documentation

---

## 9. Local Simulation Script

Save development time by simulating GitHub Actions locally:

```bash
#!/bin/bash
# scripts/simulate-ci.sh

set -euo pipefail

echo "=== Simulating GitHub Actions Pipeline ==="

# Step 1: Validate
echo "[1/5] Validating registry..."
bash .github/scripts/validate-registry.sh

# Step 2: Fetch Data (local mode)
echo "[2/5] Fetching model data..."
bash .github/scripts/fetch-model-data.sh

# Step 3: Generate JSON
echo "[3/5] Generating model-status.json..."
bash .github/scripts/generate-model-json.sh

# Step 4: Build Frontend
echo "[4/5] Building frontend..."
cd frontend && npm ci && npm run build

# Step 5: Preview
echo "[5/5] Starting local preview..."
npm run preview

echo "=== Simulation Complete ==="
```

---

## 10. Success Metrics

### Technical Metrics
- GitHub Actions runtime < 25 minutes
- API rate limit utilization < 80%
- Frontend Lighthouse score > 90
- Page load time < 2 seconds
- Zero security vulnerabilities

### User Metrics (via Microsoft Clarity)
- Average session duration > 2 minutes
- Bounce rate < 40%
- Return visitor rate > 30%
- Search usage > 50% of sessions
- Filter usage > 40% of sessions

### Content Metrics
- Models tracked: 50+ (Month 1), 200+ (Month 3)
- Data freshness: < 24 hours old
- Validation accuracy: > 95%
- Community contributions: 10+ PRs/month

---

## 11. Risk Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| API Rate Limits | High | Medium | Caching, incremental updates, multiple sources |
| GitHub Actions Quota | High | Low | Optimize runtime, run once daily |
| Data Accuracy | High | Medium | Multi-source validation, community flagging |
| Frontend Performance | Medium | Low | Code splitting, lazy loading, optimization |
| Security Vulnerabilities | High | Low | Regular audits, Dependabot, minimal dependencies |
| Low Adoption | High | Medium | SEO optimization, community engagement, social sharing |

---

## 12. Next Steps

1. **Immediate**: Review and approve this plan
2. **Day 1**: Setup frontend scaffolding
3. **Day 2**: Implement core components
4. **Day 3**: Enhance data pipeline
5. **Day 4**: Add validation engine
6. **Day 5**: Integrate and test
7. **Day 6**: Deploy beta version
8. **Day 7**: Gather feedback and iterate

---

## Appendix A: Font Configuration

```css
/* Google Fonts import */
@import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500;600&display=swap');

/* Custom font stack */
:root {
  --font-sans: 'IBM Plex Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --font-mono: 'IBM Plex Mono', 'Fira Code', monospace;
}
```

## Appendix B: Carbon Theme Customization

```scss
// Rounded corners instead of sharp Carbon defaults
$carbon--border-radius: 0.5rem;

// Custom color palette (if needed)
$custom-primary: #0f62fe; // Keep Carbon blue
$custom-success: #198038; // Keep Carbon green
```

## Appendix C: Microsoft Clarity Setup

```typescript
// Add to index.html or via useEffect
<script type="text/javascript">
  (function(c,l,a,r,i,t,y){
    c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
    t=l.createElement(r);t.async=1;t.src="https://www.clarity.ms/tag/"+i;
    y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
  })(window, document, "clarity", "script", "YOUR_CLARITY_ID");
</script>
```

---

**Document Version**: 1.0  
**Last Updated**: 2026-09-04  
**Status**: Ready for Implementation

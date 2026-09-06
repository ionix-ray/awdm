# Test Cases - AI Model Intelligence Dashboard

## BDD Feature Scenarios (Gherkin)

### Feature: Model Discovery & Search

```gherkin
Feature: Model Discovery
  As an AI researcher or developer
  I want to discover and filter AI models efficiently
  So that I can find the best models for my needs

  Scenario: User views latest model releases
    Given the dashboard is loaded with model data
    When the user sorts by "Release Date" (newest first)
    Then models released in the last 7 days appear at the top
    And each new model displays a "New" badge
    And the release date is shown in human-readable format

  Scenario: User filters for free models only
    Given the dashboard is loaded with model data
    When the user enables the "Free Tier Only" filter
    Then only models with isFree=true OR freeTier.available=true are displayed
    And each card shows the free tier limits (RPM, RPD, tokens)
    And the count of filtered results is displayed
    And the URL updates to reflect the filter state (?free=true)

  Scenario: User searches for a specific model by name
    Given the dashboard is loaded with 50+ models
    When the user types "Llama" in the search bar
    Then models with "Llama" in name or modelId are shown
    And results are ranked by relevance score
    And the search is debounced by 300ms
    And matching text is highlighted in results

  Scenario: User searches for a specific provider
    Given the dashboard is loaded with model data
    When the user types "Meta" in the search bar
    Then all models from provider "meta-llama" are shown
    And the provider name is displayed on each card
    And the filter chip shows "Provider: Meta"

  Scenario: User combines multiple filters
    Given the dashboard is loaded with model data
    When the user applies:
      | Filter | Value |
      | Free Tier Only | true |
      | Open Weights | true |
      | Category | LLM |
      | Min Confidence | 80 |
    Then only models matching ALL criteria are shown
    And active filters are displayed as removable chips
    And the URL encodes all filter states
    And clearing a filter updates results immediately

  Scenario: User paginates through results
    Given the dashboard has 150 models
    When the user views page 1 with 25 items per page
    Then exactly 25 models are displayed
    And pagination controls show "Page 1 of 6"
    When the user clicks "Next Page"
    Then models 26-50 are displayed
    And the URL updates to ?page=2
    When the user changes page size to 50
    Then the total pages recalculates to 3
    And the user stays on page 1

  Scenario: User shares a filtered view
    Given the user has applied filters and search
    When the user copies the URL
    And shares it with another user
    Then the second user sees the exact same filtered view
    And all state is preserved in URL parameters

  Scenario: User sorts by different criteria
    Given the dashboard is loaded with model data
    When the user selects sort by "Relevance Score"
    Then models are sorted by _computed.relevanceScore descending
    When the user selects sort by "Name (A-Z)"
    Then models are sorted alphabetically by name
    When the user selects sort by "Provider"
    Then models are grouped and sorted by provider name
    And the sort selection persists across navigation
```

### Feature: Model Details & Insights

```gherkin
Feature: Model Details
  As a developer evaluating models
  I want to see detailed model information
  So that I can make informed decisions

  Scenario: User views model card details
    Given the user is viewing a model card
    Then the following information is visible:
      | Field | Source |
      | Model Name | model.name |
      | Provider | model.provider |
      | Release Date | model.releaseDate |
      | Parameters | model.specifications.parameters |
      | Context Window | model.specifications.contextWindow |
      | Architecture | model.specifications.architecture |
      | Modality | model.specifications.modality |
      | License | model.availability.license |
      | Open Weights | model.availability.openWeights |

  Scenario: User checks free tier availability
    Given the model has a free tier
    When the user views the model card
    Then a green "Free" badge is displayed
    And hovering shows tooltip with limits:
      | Limit | Display |
      | RPM | "X requests/minute" |
      | RPD | "Y requests/day" |
      | Tokens | "Z tokens/month" |
    And conditions are shown if present

  Scenario: User evaluates model confidence
    Given the model has validation flags
    When the user views the confidence indicator
    Then a visual score (0-100) is displayed
    And the color reflects confidence level:
      | Score Range | Color |
      | 90-100 | Green |
      | 70-89 | Yellow |
      | 50-69 | Orange |
      | 0-49 | Red |
    And verification badges are shown:
      | Flag | Icon |
      | communityVerified | ✓ Community |
      | officialAnnouncement | 📢 Official |
      | aiValidated | 🤖 AI Checked |

  Scenario: User expands model details
    Given the user is viewing a compact model card
    When the user clicks "Expand" or the card
    Then additional details slide into view:
      - Full benchmark scores
      - News references with links
      - Hosting providers list
      - Source attributions
    And the card height animates smoothly
    And an "Collapse" button appears

  Scenario: User sees anomaly warnings
    Given the model has anomalyDetected=true
    When the user views the model card
    Then a warning icon is displayed
    And hovering shows: "Anomaly detected in benchmarks"
    And the confidence score is capped at 50
    And a tooltip explains the anomaly type

  Scenario: User views news references
    Given the model has newsReferences
    When the user expands the model card
    Then news items are listed with:
      - Title (clickable link)
      - Source name
      - Published date (relative time)
      - Sentiment indicator (positive/neutral/negative)
    And clicking opens the news in a new tab
    And external links have rel="noopener noreferrer"
```

### Feature: Dashboard Analytics

```gherkin
Feature: Dashboard Overview
  As an AI enthusiast
  I want to see high-level statistics
  So that I understand the current landscape

  Scenario: User views summary statistics
    Given the dashboard is loaded
    Then the stats overview shows:
      | Metric | Calculation |
      | Total Models | models.length |
      | Free Models | count(isFree=true) |
      | New Releases (7d) | count(daysSinceRelease <= 7) |
      | High Confidence | count(confidenceScore >= 80) |
      | Unique Providers | unique(providers).length |
      | Open Weight Models | count(openWeights=true) |

  Scenario: User views provider distribution
    Given the dashboard has model data
    When the user views the provider breakdown chart
    Then a bar/pie chart shows models per provider
    And providers are sorted by count descending
    And clicking a provider filters to that provider
    And percentages are displayed on hover

  Scenario: User views category distribution
    Given the dashboard has categorized models
    When the user views the category breakdown
    Then categories are displayed with counts:
      | Category | Count | Percentage |
      | LLM | X | Y% |
      | Vision | X | Y% |
      | Multimodal | X | Y% |
    And multi-category models are counted in each
    And clicking a category filters to that category

  Scenario: User sees trending models
    Given there are models with high relevance scores
    When the user views the "Trending" section
    Then top 5 models by relevanceScore are shown
    And each shows why it's trending:
      - "Released 3 days ago"
      - "High community verification"
      - "Free tier available"
    And clicking navigates to the filtered view

  Scenario: User views recent updates
    Given models have lastCheckedAt timestamps
    When the user views the "Recently Updated" section
    Then models sorted by lastCheckedAt descending are shown
    And relative time is displayed ("2 hours ago")
    And a tooltip shows the exact timestamp
```

### Feature: GitHub Actions Pipeline

```gherkin
Feature: Automated Data Updates
  As a maintainer
  I want the pipeline to run reliably
  So that data stays fresh without manual intervention

  Scenario: Scheduled workflow triggers
    Given it is 6:00 AM UTC
    When the cron scheduler triggers
    Then the workflow starts automatically
    And all jobs execute in sequence
    And completion notification is logged

  Scenario: Manual workflow trigger
    Given a user has write access to the repo
    When the user clicks "Run workflow" in Actions tab
    And optionally checks "Force refresh"
    Then the workflow starts immediately
    And cache is cleared if force_refresh=true

  Scenario: Workflow respects rate limits
    Given Hugging Face API has 50 req/hour limit
    When the fetch-data job runs
    Then API calls are throttled to stay under limit
    And cached responses are used when available
    And a warning is logged if approaching limit
    And the job fails gracefully if rate limited

  Scenario: Workflow uses caching efficiently
    Given previous workflow runs have cached data
    When the current workflow runs
    Then cache is checked before making API calls
    And only changed data is refetched
    And cache TTL is respected (1 hour default)
    And cache hit ratio is logged

  Scenario: Workflow detects changes
    Given the workflow has fetched new data
    When the commit-check step runs
    Then git diff compares old vs new JSON
    If no changes:
      And has_changes=false is set
      And subsequent jobs are skipped
      And log shows "No changes detected"
    If changes exist:
      And has_changes=true is set
      And commit is created with message
      And PR is opened (if configured)

  Scenario: Workflow validates data integrity
    Given new model data has been fetched
    When the validate job runs
    Then JSON schema validation is performed
    And all required fields are checked
    And data types are validated
    If validation fails:
      And workflow fails with error message
      And no deployment occurs
      And maintainer is notified via issue

  Scenario: Workflow completes within time budget
    Given the workflow starts
    When all jobs complete
    Then total runtime is < 25 minutes
    And each job logs its duration
    And slow jobs are flagged for optimization
    And timeout is set to 30 minutes as safety

  Scenario: Workflow handles failures gracefully
    Given an API source is unavailable
    When the fetch-data job encounters an error
    Then the error is caught and logged
    And partial data is still processed
    And a warning is added to metadata
    And the workflow continues if non-critical
    Or fails fast if critical source missing

  Scenario: Concurrency control prevents duplicates
    Given a workflow run is in progress
    When another trigger occurs (manual or scheduled)
    Then the new run waits (cancel-in-progress=false)
    Or is cancelled if configured
    And only one run modifies data at a time
    And race conditions are prevented
```

### Feature: Security & Error Handling

```gherkin
Feature: Security & Resilience
  As a user
  I want the application to be secure and reliable
  So that I can trust the information

  Scenario: Frontend loads without API keys
    Given the frontend is deployed
    When the page loads
    Then no API keys are present in source code
    And network tab shows only static asset requests
    And data is loaded from local model-status.json
    And console shows no security warnings

  Scenario: XSS prevention
    Given malicious script in model name: "<script>alert('x')</script>"
    When the model card renders
    Then the script tag is escaped as HTML entities
    And no alert popup appears
    And the name displays as plain text
    And DOMPurify sanitizes any user content

  Scenario: Graceful degradation on data fetch failure
    Given the model-status.json file is missing or corrupted
    When the frontend tries to load data
    Then an error boundary catches the exception
    And a user-friendly message is shown: "Data unavailable"
    And a retry button is provided
    And cached data is used if available
    And the error is logged to console

  Scenario: Partial data rendering
    Given some model fields are missing
    When the model card renders
    Then available fields are displayed normally
    And missing fields show "N/A" or are hidden
    And no JavaScript errors occur
    And layout doesn't break

  Scenario: Rate limit handling
    Given the API returns 429 Too Many Requests
    When the GitHub Action fetches data
    Then the request is retried after delay
    And cached data is used as fallback
    And a warning is logged
    And the workflow continues with partial data

  Scenario: Invalid JSON schema detection
    Given malformed JSON is generated
    When the validation step runs
    Then jsonschema validation fails
    And specific error messages are shown
    And the workflow stops before deployment
    And maintainers are notified

  Scenario: Network timeout handling
    Given an API endpoint times out (>30s)
    When the fetch-data job runs
    Then the timeout is caught
    And the source is marked as unavailable
    And other sources continue fetching
    And metadata includes source status
```

---

## Unit Test Specifications

### Component Tests

#### ModelCard Component
```typescript
describe('ModelCard', () => {
  test('renders model name and provider correctly', () => {
    const model = {
      name: 'Llama-3.2-1B',
      provider: 'meta-llama',
      modelId: 'meta-llama/Llama-3.2-1B'
    };
    render(<ModelCard model={model} />);
    expect(screen.getByText('Llama-3.2-1B')).toBeInTheDocument();
    expect(screen.getByText('meta-llama')).toBeInTheDocument();
  });

  test('displays free tier badge when isFree=true', () => {
    const model = { isFree: true, freeTier: { available: true } };
    render(<ModelCard model={model} />);
    expect(screen.getByText('Free')).toBeInTheDocument();
  });

  test('hides free tier badge when isFree=false', () => {
    const model = { isFree: false, freeTier: { available: false } };
    render(<ModelCard model={model} />);
    expect(screen.queryByText('Free')).not.toBeInTheDocument();
  });

  test('shows confidence score with correct color', () => {
    const model = { validationFlags: { confidenceScore: 95 } };
    render(<ModelCard model={model} />);
    const indicator = screen.getByTestId('confidence-indicator');
    expect(indicator).toHaveClass('confidence-high'); // green
  });

  test('expands/collapses on click', async () => {
    const model = { /* full model data */ };
    render(<ModelCard model={model} />);
    const card = screen.getByTestId('model-card');
    expect(card).toHaveClass('collapsed');
    
    await userEvent.click(screen.getByText('Expand'));
    expect(card).toHaveClass('expanded');
    
    await userEvent.click(screen.getByText('Collapse'));
    expect(card).toHaveClass('collapsed');
  });

  test('displays "New" badge for recent releases', () => {
    const model = {
      releaseDate: new Date().toISOString(),
      _computed: { daysSinceRelease: 3, isNewRelease: true }
    };
    render(<ModelCard model={model} />);
    expect(screen.getByText('New')).toBeInTheDocument();
  });

  test('sanitizes HTML in model name', () => {
    const model = { name: '<script>alert("x")</script>Test' };
    render(<ModelCard model={model} />);
    expect(screen.queryByText('<script>')).not.toBeInTheDocument();
    expect(screen.getByText(/Test/)).toBeInTheDocument();
  });
});
```

#### SearchBar Component
```typescript
describe('SearchBar', () => {
  test('filters models by name match', () => {
    const models = [
      { name: 'Llama-3.2-1B' },
      { name: 'Gemma-2-2B' },
      { name: 'Mistral-7B' }
    ];
    render(<SearchBar models={models} searchTerm="Llama" />);
    expect(screen.getByText('Llama-3.2-1B')).toBeInTheDocument();
    expect(screen.queryByText('Gemma-2-2B')).not.toBeInTheDocument();
  });

  test('debounces search input by 300ms', async () => {
    const mockOnSearch = vi.fn();
    render(<SearchBar onSearch={mockOnSearch} debounceMs={300} />);
    
    await userEvent.type(screen.getByRole('searchbox'), 'Llama');
    expect(mockOnSearch).not.toHaveBeenCalled();
    
    vi.advanceTimersByTime(300);
    expect(mockOnSearch).toHaveBeenCalledWith('Llama');
  });

  test('clears search on X button click', async () => {
    const mockOnSearch = vi.fn();
    render(<SearchBar onSearch={mockOnSearch} />);
    
    await userEvent.type(screen.getByRole('searchbox'), 'Test');
    await userEvent.click(screen.getByLabelText('Clear search'));
    
    expect(mockOnSearch).toHaveBeenCalledWith('');
    expect(screen.getByRole('searchbox')).toHaveValue('');
  });

  test('highlights matching text in results', () => {
    const models = [{ name: 'Llama-3.2-1B' }];
    render(<SearchBar models={models} searchTerm="Llama" highlight />);
    expect(document.querySelector('mark')).toHaveTextContent('Llama');
  });
});
```

#### FilterPanel Component
```typescript
describe('FilterPanel', () => {
  test('toggles free tier filter', async () => {
    const mockOnFilterChange = vi.fn();
    render(<FilterPanel onFilterChange={mockOnFilterChange} />);
    
    const freeCheckbox = screen.getByLabelText('Free Tier Only');
    await userEvent.click(freeCheckbox);
    
    expect(mockOnFilterChange).toHaveBeenCalledWith({ freeOnly: true });
  });

  test('selects multiple categories', async () => {
    const mockOnFilterChange = vi.fn();
    render(<FilterPanel 
      categories={['LLM', 'Vision', 'Audio']} 
      onFilterChange={mockOnFilterChange} 
    />);
    
    await userEvent.click(screen.getByText('LLM'));
    await userEvent.click(screen.getByText('Vision'));
    
    expect(mockOnFilterChange).toHaveBeenCalledWith({
      categories: ['LLM', 'Vision']
    });
  });

  test('sets confidence score range', async () => {
    const mockOnFilterChange = vi.fn();
    render(<FilterPanel onFilterChange={mockOnFilterChange} />);
    
    await userEvent.type(screen.getByLabelText('Min Confidence'), '80');
    
    expect(mockOnFilterChange).toHaveBeenCalledWith({ minConfidence: 80 });
  });

  test('clears all filters', async () => {
    const mockOnFilterChange = vi.fn();
    render(<FilterPanel onFilterChange={mockOnFilterChange} />);
    
    // Apply some filters
    await userEvent.click(screen.getByLabelText('Free Tier Only'));
    
    // Clear all
    await userEvent.click(screen.getByText('Clear All'));
    
    expect(mockOnFilterChange).toHaveBeenCalledWith({
      freeOnly: false,
      categories: [],
      minConfidence: 0
    });
  });
});
```

#### Pagination Component
```typescript
describe('Pagination', () => {
  test('calculates correct page count', () => {
    render(<Pagination totalItems={150} itemsPerPage={25} currentPage={1} />);
    expect(screen.getByText('Page 1 of 6')).toBeInTheDocument();
  });

  test('navigates to next page', async () => {
    const mockOnPageChange = vi.fn();
    render(<Pagination 
      totalItems={100} 
      itemsPerPage={10} 
      currentPage={1}
      onPageChange={mockOnPageChange}
    />);
    
    await userEvent.click(screen.getByText('Next'));
    expect(mockOnPageChange).toHaveBeenCalledWith(2);
  });

  test('navigates to previous page', async () => {
    const mockOnPageChange = vi.fn();
    render(<Pagination 
      totalItems={100} 
      itemsPerPage={10} 
      currentPage={3}
      onPageChange={mockOnPageChange}
    />);
    
    await userEvent.click(screen.getByText('Previous'));
    expect(mockOnPageChange).toHaveBeenCalledWith(2);
  });

  test('changes page size', async () => {
    const mockOnPageSizeChange = vi.fn();
    render(<Pagination 
      totalItems={100} 
      itemsPerPage={10} 
      currentPage={1}
      onPageSizeChange={mockOnPageSizeChange}
    />);
    
    await userEvent.selectOptions(screen.getByLabelText('Items per page'), '50');
    expect(mockOnPageSizeChange).toHaveBeenCalledWith(50);
  });

  test('disables previous button on first page', () => {
    render(<Pagination totalItems={100} itemsPerPage={10} currentPage={1} />);
    expect(screen.getByText('Previous')).toBeDisabled();
  });

  test('disables next button on last page', () => {
    render(<Pagination totalItems={100} itemsPerPage={10} currentPage={10} />);
    expect(screen.getByText('Next')).toBeDisabled();
  });
});
```

### Hook Tests

#### useModels Hook
```typescript
describe('useModels', () => {
  test('loads models from JSON file', async () => {
    const { result } = renderHook(() => useModels());
    
    await waitFor(() => {
      expect(result.current.models).toHaveLength(3); // from mock data
    });
    
    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBe(null);
  });

  test('handles loading error gracefully', async () => {
    server.use(rest.get('/model-status.json', (req, res, ctx) => {
      return res(ctx.status(404));
    }));
    
    const { result } = renderHook(() => useModels());
    
    await waitFor(() => {
      expect(result.current.error).toBeTruthy();
    });
    
    expect(result.current.loading).toBe(false);
  });

  test('refetches on demand', async () => {
    const { result } = renderHook(() => useModels());
    await waitFor(() => expect(result.current.models).toHaveLength(3));
    
    act(() => {
      result.current.refetch();
    });
    
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });
});
```

#### useSearch Hook
```typescript
describe('useSearch', () => {
  test('filters models by search term', () => {
    const models = [
      { name: 'Llama-3.2-1B', provider: 'meta-llama' },
      { name: 'Gemma-2-2B', provider: 'google' }
    ];
    
    const { result } = renderHook(() => useSearch(models, 'Llama'));
    
    expect(result.current.filteredModels).toHaveLength(1);
    expect(result.current.filteredModels[0].name).toBe('Llama-3.2-1B');
  });

  test('searches across multiple fields', () => {
    const models = [
      { name: 'Model A', provider: 'meta-llama' },
      { name: 'Model B', provider: 'google' }
    ];
    
    const { result } = renderHook(() => useSearch(models, 'google'));
    
    expect(result.current.filteredModels).toHaveLength(1);
    expect(result.current.filteredModels[0].provider).toBe('google');
  });

  test('returns all models when search is empty', () => {
    const models = [{ name: 'Test' }];
    const { result } = renderHook(() => useSearch(models, ''));
    
    expect(result.current.filteredModels).toEqual(models);
  });

  test('updates URL with search params', () => {
    const { result } = renderHook(() => useSearch([], 'test', { updateUrl: true }));
    
    expect(window.location.search).toContain('q=test');
  });
});
```

### Utility Function Tests

#### Schema Validator
```typescript
describe('validateModelSchema', () => {
  test('passes valid model object', () => {
    const model = {
      modelId: 'test/model',
      name: 'Test Model',
      provider: 'test',
      tracked: true
    };
    
    const result = validateModelSchema(model);
    expect(result.valid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  test('fails when required field missing', () => {
    const model = {
      name: 'Test Model',
      // missing modelId
    };
    
    const result = validateModelSchema(model);
    expect(result.valid).toBe(false);
    expect(result.errors).toContainEqual(expect.stringContaining('modelId'));
  });

  test('fails when type mismatch', () => {
    const model = {
      modelId: 'test/model',
      name: 123, // should be string
      provider: 'test'
    };
    
    const result = validateModelSchema(model);
    expect(result.valid).toBe(false);
    expect(result.errors).toContainEqual(expect.stringContaining('string'));
  });
});
```

#### Relevance Score Calculator
```typescript
describe('calculateRelevanceScore', () => {
  test('gives high score to new, free, verified models', () => {
    const model = {
      isFree: true,
      validationFlags: { communityVerified: true, confidenceScore: 95 },
      _computed: { daysSinceRelease: 5 }
    };
    
    const score = calculateRelevanceScore(model);
    expect(score).toBeGreaterThan(80);
  });

  test('gives low score to old, paid, unverified models', () => {
    const model = {
      isFree: false,
      validationFlags: { communityVerified: false, confidenceScore: 30 },
      _computed: { daysSinceRelease: 500 }
    };
    
    const score = calculateRelevanceScore(model);
    expect(score).toBeLessThan(40);
  });

  test('caps score at 100', () => {
    const model = {
      isFree: true,
      validationFlags: { communityVerified: true, officialAnnouncement: true, confidenceScore: 100 },
      _computed: { daysSinceRelease: 1 }
    };
    
    const score = calculateRelevanceScore(model);
    expect(score).toBeLessThanOrEqual(100);
  });
});
```

---

## Integration Tests

```typescript
describe('Dashboard Integration', () => {
  test('full user flow: search, filter, paginate', async () => {
    render(<Dashboard />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });
    
    // Apply filter
    await userEvent.click(screen.getByLabelText('Free Tier Only'));
    expect(screen.getAllByTestId('model-card')).toHaveLength(3);
    
    // Search
    await userEvent.type(screen.getByRole('searchbox'), 'Llama');
    expect(screen.getByText('Llama-3.2-1B')).toBeInTheDocument();
    
    // Change page size
    await userEvent.selectOptions(screen.getByLabelText('Items per page'), '10');
    
    // Navigate to page 2
    await userEvent.click(screen.getByText('Next'));
    
    // Verify URL state
    expect(window.location.search).toContain('free=true');
    expect(window.location.search).toContain('q=Llama');
    expect(window.location.search).toContain('page=2');
  });
});
```

---

## Performance Tests

```typescript
describe('Performance', () => {
  test('renders 100 models in under 2 seconds', async () => {
    const largeDataset = generateModels(100);
    
    const start = performance.now();
    render(<Dashboard initialModels={largeDataset} />);
    
    await waitFor(() => {
      expect(screen.getAllByTestId('model-card')).toHaveLength(100);
    });
    
    const end = performance.now();
    expect(end - start).toBeLessThan(2000);
  });

  test('search responds within 100ms after debounce', async () => {
    const models = generateModels(50);
    render(<SearchBar models={models} />);
    
    const start = performance.now();
    await userEvent.type(screen.getByRole('searchbox'), 'Test');
    vi.advanceTimersByTime(300); // debounce
    
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });
    
    const end = performance.now();
    expect(end - start - 300).toBeLessThan(100);
  });
});
```

---

**Total Test Coverage Target**: >90%  
**Critical Paths**: 100% covered  
**Edge Cases**: All documented scenarios tested

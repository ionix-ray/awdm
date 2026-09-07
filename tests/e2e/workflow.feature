Feature: AI Model Validation Pipeline
  As an AI researcher or developer
  I want to validate AI model information automatically
  So that I can trust the data presented on the dashboard

  Background:
    Given the configuration file config.toml is properly set up
    And GitHub secrets OPENCODE_API_TOKEN is configured
    And the model registry contains pending models to validate

  Scenario: Validate new model release in zen mode (cost-optimized)
    Given a new model "Gemma-2-9B" is detected from HuggingFace
    When AI validation runs in "zen" mode
    Then the model should be verified with confidence score >= 70
    And free tier information should be extracted
    And anomalies should be flagged if detected
    And the validation result should be stored in JSON format

  Scenario: Comprehensive validation in go mode
    Given a new model "Llama-3.1-405B" is detected
    When AI validation runs in "go" mode
    Then all specifications should be verified
    And benchmark scores should be validated for anomalies
    And cross-source verification should be performed
    And news sentiment should be analyzed
    And confidence score should reflect comprehensive analysis

  Scenario: Free tier detection and validation
    Given a model claims to have free tier access
    When AI validates pricing information
    Then free limits (RPM, RPD, tokens) should be extracted
    And conditions should be documented
    And hidden restrictions should be identified
    And the information should be marked as verified or unverified

  Scenario: Provider fallback mechanism
    Given the primary AI provider (OpenCode) is unavailable
    When validation request is made
    Then the system should try the next priority provider
    And continue until a provider responds
    Or fail gracefully after all providers are exhausted
    And log the failure for monitoring

  Scenario: Rate limiting protection
    Given multiple models need validation
    When API calls approach rate limit
    Then the system should implement exponential backoff
    And queue remaining requests
    And stay within free tier limits
    And log rate limit events

  Scenario: Schema validation of AI output
    Given AI returns validation response
    When output is received
    Then JSON structure should match expected schema
    And all required fields must be present
    And data types should be validated
    And invalid responses should trigger retry

  Scenario: Atomic data write operation
    Given validated model data is ready to save
    When writing to model-status.json
    Then data should be written to temporary file first
    And validated before replacing live file
    And backup should be created
    And rollback should occur on failure

  Scenario: Manual workflow trigger with custom parameters
    Given a user triggers the GitHub Action manually
    When they specify force_refresh=true and ai_mode="go"
    Then cache should be cleared
    And comprehensive validation should run
    And all models should be re-validated
    And results should be pushed to repository

Feature: Dashboard Frontend
  As a dashboard user
  I want to browse and filter AI models efficiently
  So that I can find relevant models for my needs

  Scenario: Search for specific model
    Given the dashboard is loaded with model data
    When I search for "Llama"
    Then only models containing "Llama" in name or provider should appear
    And search should be case-insensitive
    And results should update in real-time

  Scenario: Filter by free tier only
    Given the dashboard displays mixed free and paid models
    When I enable "Free Only" filter
    Then only models with is_free=true should be visible
    And free tier details should be highlighted
    And count should update to show filtered results

  Scenario: Filter by minimum confidence score
    Given models have varying confidence scores
    When I set minimum confidence to 80%
    Then only models with confidence >= 80 should appear
    And low-confidence models should be hidden
    And warning should show if no models match

  Scenario: Pagination through results
    Given there are more than 12 models in results
    When I view page 1
    Then exactly 12 models should be displayed
    And "Next" button should be enabled
    When I click "Next"
    Then the next 12 models should appear
    And page indicator should update

  Scenario: Responsive design on mobile
    Given I'm viewing the dashboard on a mobile device (375px width)
    Then the layout should adapt to single column
    And all features should remain accessible
    And text should be readable without zooming

  Scenario: Error handling when data fails to load
    Given the model-status.json file is unavailable
    When the dashboard attempts to load
    Then a user-friendly error message should display
    And suggestion to refresh should be shown
    And console should log detailed error for debugging

Feature: Security and Compliance
  As a security-conscious user
  I want the dashboard to follow security best practices
  So that my data and experience are protected

  Scenario: Content Security Policy enforcement
    Given the dashboard loads in browser
    Then CSP headers should prevent inline scripts from untrusted sources
    And only approved CDNs should be allowed
    And WASM execution should be explicitly permitted

  Scenario: XSS prevention
    Given AI-generated content may contain malicious scripts
    When content is rendered
    Then all HTML should be sanitized
    And script tags should be escaped
    And no arbitrary code execution should occur

  Scenario: API key protection
    Given API keys are required for AI validation
    When GitHub Actions run
    Then keys should be retrieved from GitHub Secrets
    And never exposed in frontend code
    And not logged in build output

  Scenario: Data integrity verification
    Given model data is updated periodically
    When new data is generated
    Then SHA256 hash should be computed
    And integrity should be verifiable
    And tampering should be detectable

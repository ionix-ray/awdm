import { sanitizeHtml } from './helpers';

export const escapeHtml = (text: string): string => {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
};

export const validateUrl = (url: string): boolean => {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
};

export const sanitizeInput = (input: string): string => {
  // Remove potential XSS vectors
  const sanitized = input
    .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
    .replace(/javascript:/gi, '')
    .replace(/on\w+\s*=/gi, '');
  
  return escapeHtml(sanitized);
};

export const validateJsonSchema = (data: unknown): boolean => {
  if (!data || typeof data !== 'object') return false;
  
  const obj = data as Record<string, unknown>;
  
  // Basic schema validation
  if (!obj.metadata || typeof obj.metadata !== 'object') return false;
  if (!obj.summary || typeof obj.summary !== 'object') return false;
  if (!Array.isArray(obj.models)) return false;
  
  return true;
};

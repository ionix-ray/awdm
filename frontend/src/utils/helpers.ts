import { format, differenceInDays } from 'date-fns';
import type { Model } from '@types';

export const formatDate = (dateString: string): string => {
  try {
    return format(new Date(dateString), 'MMM dd, yyyy');
  } catch {
    return 'Unknown';
  }
};

export const formatRelativeDate = (dateString: string): string => {
  try {
    const days = differenceInDays(new Date(), new Date(dateString));
    if (days === 0) return 'Today';
    if (days === 1) return 'Yesterday';
    if (days < 7) return `${days} days ago`;
    if (days < 30) return `${Math.floor(days / 7)} weeks ago`;
    if (days < 365) return `${Math.floor(days / 30)} months ago`;
    return `${Math.floor(days / 365)} years ago`;
  } catch {
    return 'Unknown';
  }
};

export const getPriorityColor = (priority: Model['priority']): string => {
  switch (priority) {
    case 'critical':
      return 'red-60';
    case 'high':
      return 'orange-50';
    case 'medium':
      return 'yellow-50';
    case 'low':
      return 'green-50';
    default:
      return 'gray-50';
  }
};

export const getPriorityLabel = (priority: Model['priority']): string => {
  return priority.charAt(0).toUpperCase() + priority.slice(1);
};

export const truncateText = (text: string, maxLength: number): string => {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength - 3) + '...';
};

export const sanitizeHtml = (html: string): string => {
  const div = document.createElement('div');
  div.textContent = html;
  return div.innerHTML;
};

export const calculateRelevanceScore = (model: Model): number => {
  let score = 0;
  
  // Recency boost (newer models get higher scores)
  const daysSinceRelease = differenceInDays(new Date(), new Date(model.releaseDate));
  if (daysSinceRelease < 7) score += 30;
  else if (daysSinceRelease < 30) score += 20;
  else if (daysSinceRelease < 90) score += 10;
  
  // Free tier boost
  if (model.isFree || model.freeTier.available) score += 15;
  
  // Open weights boost
  if (model.availability.openWeights) score += 15;
  
  // Confidence score contribution (max 25 points)
  score += (model.validationFlags.confidenceScore / 100) * 25;
  
  // Priority boost
  switch (model.priority) {
    case 'critical':
      score += 15;
      break;
    case 'high':
      score += 10;
      break;
    case 'medium':
      score += 5;
      break;
  }
  
  return Math.min(100, Math.round(score));
};

export const getConfidenceLevel = (score: number): 'high' | 'medium' | 'low' => {
  if (score >= 80) return 'high';
  if (score >= 50) return 'medium';
  return 'low';
};

export const getConfidenceColor = (score: number): string => {
  if (score >= 80) return 'green-60';
  if (score >= 50) return 'yellow-60';
  return 'red-60';
};

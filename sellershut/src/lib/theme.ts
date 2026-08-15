export type Theme = 'light' | 'dark' | 'system';

const STORAGE_KEY = 'theme';

export function getTheme(): Theme {
  if (typeof localStorage === 'undefined') {
    return 'system';
  }

  const stored = localStorage.getItem(STORAGE_KEY);

  if (stored === 'light' || stored === 'dark' || stored === 'system') {
    return stored;
  }

  return 'system';
}

export function applyTheme(theme: Theme): void {
  const html = document.documentElement;
  html.classList.remove('light', 'dark');

  if (theme === 'light') {
    html.classList.add('light');
  } else if (theme === 'dark') {
    html.classList.add('dark');
  }
}

export function setTheme(theme: Theme): void {
  localStorage.setItem(STORAGE_KEY, theme);

  applyTheme(theme);
}

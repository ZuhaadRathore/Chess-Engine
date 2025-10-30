import React from 'react';
import { useTheme, Theme } from '../contexts/ThemeContext';
import './ThemeToggle.css';

interface ThemeToggleProps {
  variant?: 'inline' | 'floating';
}

const ThemeToggle: React.FC<ThemeToggleProps> = ({ variant = 'inline' }) => {
  const { theme, setTheme } = useTheme();

  const getNextTheme = (currentTheme: Theme): Theme => {
    switch (currentTheme) {
      case 'light':
        return 'dark';
      case 'dark':
        return 'system';
      case 'system':
        return 'light';
      default:
        return 'light';
    }
  };

  const handleToggle = () => {
    const nextTheme = getNextTheme(theme);
    setTheme(nextTheme);
  };

  const getThemeIcon = (currentTheme: Theme): string => {
    switch (currentTheme) {
      case 'light':
        return '☀️';
      case 'dark':
        return '🌙';
      case 'system':
        return '🔄';
      default:
        return '☀️';
    }
  };

  const getThemeLabel = (currentTheme: Theme): string => {
    switch (currentTheme) {
      case 'light':
        return 'Light mode';
      case 'dark':
        return 'Dark mode';
      case 'system':
        return 'System theme';
      default:
        return 'Light mode';
    }
  };

  return (
    <button
      className={`theme-toggle ${variant}`}
      onClick={handleToggle}
      aria-label={`Toggle theme (current: ${getThemeLabel(theme)})`}
      title={getThemeLabel(theme)}
    >
      <span className="theme-toggle-icon">{getThemeIcon(theme)}</span>
    </button>
  );
};

export default ThemeToggle;

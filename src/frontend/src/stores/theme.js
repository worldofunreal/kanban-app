import { ref, watch } from 'vue'

const currentTheme = ref(localStorage.getItem('theme') || 'default')
const currentColor = ref(localStorage.getItem('color') || 'neutral')

const themes = {
  default: {
    name: 'Default',
    description: 'Clean and minimal design',
    radius: '0.5rem'
  },
  'new-york': {
    name: 'New York',
    description: 'Rounded and modern design',
    radius: '0.75rem'
  }
}

const colors = {
  neutral: {
    name: 'Neutral',
    primary: '222.2 47.4% 11.2%',
    primaryForeground: '210 40% 98%'
  },
  slate: {
    name: 'Slate',
    primary: '222.2 47.4% 11.2%',
    primaryForeground: '210 40% 98%'
  },
  gray: {
    name: 'Gray',
    primary: '220 9% 46%',
    primaryForeground: '0 0% 98%'
  },
  zinc: {
    name: 'Zinc',
    primary: '240 5% 34%',
    primaryForeground: '0 0% 98%'
  },
  stone: {
    name: 'Stone',
    primary: '25 5% 45%',
    primaryForeground: '0 0% 98%'
  },
  red: {
    name: 'Red',
    primary: '0 84% 60%',
    primaryForeground: '0 0% 98%'
  },
  orange: {
    name: 'Orange',
    primary: '24 95% 53%',
    primaryForeground: '0 0% 98%'
  },
  amber: {
    name: 'Amber',
    primary: '38 92% 50%',
    primaryForeground: '0 0% 98%'
  },
  yellow: {
    name: 'Yellow',
    primary: '48 96% 53%',
    primaryForeground: '0 0% 98%'
  },
  lime: {
    name: 'Lime',
    primary: '82 76% 36%',
    primaryForeground: '0 0% 98%'
  },
  green: {
    name: 'Green',
    primary: '142 76% 36%',
    primaryForeground: '0 0% 98%'
  },
  emerald: {
    name: 'Emerald',
    primary: '156 64% 41%',
    primaryForeground: '0 0% 98%'
  },
  teal: {
    name: 'Teal',
    primary: '173 58% 39%',
    primaryForeground: '0 0% 98%'
  },
  cyan: {
    name: 'Cyan',
    primary: '187 75% 36%',
    primaryForeground: '0 0% 98%'
  },
  sky: {
    name: 'Sky',
    primary: '199 89% 48%',
    primaryForeground: '0 0% 98%'
  },
  blue: {
    name: 'Blue',
    primary: '221 83% 53%',
    primaryForeground: '0 0% 98%'
  },
  indigo: {
    name: 'Indigo',
    primary: '238 83% 77%',
    primaryForeground: '0 0% 98%'
  },
  violet: {
    name: 'Violet',
    primary: '262 83% 58%',
    primaryForeground: '0 0% 98%'
  },
  purple: {
    name: 'Purple',
    primary: '263 70% 50%',
    primaryForeground: '0 0% 98%'
  },
  fuchsia: {
    name: 'Fuchsia',
    primary: '292 84% 61%',
    primaryForeground: '0 0% 98%'
  },
  pink: {
    name: 'Pink',
    primary: '330 81% 60%',
    primaryForeground: '0 0% 98%'
  },
  rose: {
    name: 'Rose',
    primary: '346 77% 49%',
    primaryForeground: '0 0% 98%'
  }
}

export function useThemeStore() {
  const setTheme = (theme) => {
    currentTheme.value = theme
    localStorage.setItem('theme', theme)
    
    // Update CSS variables
    const root = document.documentElement
    const themeConfig = themes[theme]
    if (themeConfig) {
      root.style.setProperty('--radius', themeConfig.radius)
    }
  }

  const setColor = (color) => {
    currentColor.value = color
    localStorage.setItem('color', color)
    
    // Update CSS variables
    const root = document.documentElement
    const colorConfig = colors[color]
    if (colorConfig) {
      root.style.setProperty('--primary', colorConfig.primary)
      root.style.setProperty('--primary-foreground', colorConfig.primaryForeground)
    }
  }

  const toggleDarkMode = () => {
    const isDark = document.documentElement.classList.contains('dark')
    if (isDark) {
      document.documentElement.classList.remove('dark')
      localStorage.setItem('darkMode', 'false')
    } else {
      document.documentElement.classList.add('dark')
      localStorage.setItem('darkMode', 'true')
    }
  }

  // Initialize theme on mount
  const initTheme = () => {
    const savedTheme = localStorage.getItem('theme') || 'default'
    const savedColor = localStorage.getItem('color') || 'neutral'
    const savedDarkMode = localStorage.getItem('darkMode') === 'true'
    
    setTheme(savedTheme)
    setColor(savedColor)
    
    if (savedDarkMode) {
      document.documentElement.classList.add('dark')
    }
  }

  return {
    currentTheme,
    currentColor,
    themes,
    colors,
    setTheme,
    setColor,
    toggleDarkMode,
    initTheme
  }
} 
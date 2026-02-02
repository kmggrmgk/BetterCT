import * as React from 'react';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import CssBaseline from '@mui/material/CssBaseline';

const PRIMARY_COLOR = '#4876EF';
const SECONDARY_COLOR = '#00D3AB';

export default function AppTheme({ children, disableCustomTheme }: { children: React.ReactNode; disableCustomTheme?: boolean }) {
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');
  
  const theme = React.useMemo(() => {
    if (disableCustomTheme) {
      return createTheme();
    }
    
    return createTheme({
      palette: {
        mode: prefersDarkMode ? 'dark' : 'light',
        primary: {
          main: PRIMARY_COLOR,
        },
        secondary: {
          main: SECONDARY_COLOR,
        },
      },
      typography: {
        fontFamily: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
        h1: {
          fontSize: 'clamp(2.625rem, 10vw, 4rem)',
          fontWeight: 800,
          lineHeight: 1.1,
        },
        h2: {
          fontSize: 'clamp(1.5rem, 5vw, 2.5rem)',
          fontWeight: 700,
          lineHeight: 1.2,
        },
        h3: {
          fontSize: 'clamp(1.25rem, 3vw, 2rem)',
          fontWeight: 600,
          lineHeight: 1.3,
        },
        h4: {
          fontSize: 'clamp(1.125rem, 2.5vw, 1.75rem)',
          fontWeight: 600,
          lineHeight: 1.4,
        },
        h5: {
          fontSize: '1.125rem',
          fontWeight: 600,
        },
        h6: {
          fontSize: '1rem',
          fontWeight: 600,
        },
        body1: {
          fontSize: '1rem',
          lineHeight: 1.6,
        },
        body2: {
          fontSize: '0.875rem',
          lineHeight: 1.5,
        },
        subtitle1: {
          fontSize: '1rem',
          fontWeight: 500,
        },
        subtitle2: {
          fontSize: '0.875rem',
          fontWeight: 500,
        },
        caption: {
          fontSize: '0.75rem',
        },
      },
      shape: {
        borderRadius: 8,
      },
      components: {
        MuiButton: {
          styleOverrides: {
            root: {
              textTransform: 'none',
              fontWeight: 600,
              borderRadius: 8,
            },
          },
        },
        MuiCard: {
          styleOverrides: {
            root: {
              borderRadius: 12,
              boxShadow: '0 4px 12px rgba(0, 0, 0, 0.1)',
            },
          },
        },
        MuiAppBar: {
          styleOverrides: {
            root: {
              boxShadow: 'none',
              borderBottom: '1px solid',
              borderColor: 'divider',
            },
          },
        },
      },
    });
  }, [disableCustomTheme, prefersDarkMode]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline enableColorScheme />
      {children}
    </ThemeProvider>
  );
}
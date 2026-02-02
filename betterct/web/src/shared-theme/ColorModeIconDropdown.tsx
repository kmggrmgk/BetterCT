import * as React from 'react';
import ToggleButtonGroup from '@mui/material/ToggleButtonGroup';
import ToggleButton from '@mui/material/ToggleButton';
import WbSunnyRoundedIcon from '@mui/icons-material/WbSunnyRounded';
import ModeNightRoundedIcon from '@mui/icons-material/ModeNightRounded';
import SettingsBrightnessRoundedIcon from '@mui/icons-material/SettingsBrightnessRounded';

interface ColorModeIconDropdownProps {
  size?: 'small' | 'medium' | 'large';
}

export default function ColorModeIconDropdown({ size = 'medium' }: ColorModeIconDropdownProps) {
  const [mode, setMode] = React.useState<'light' | 'dark' | 'system'>('system');
  
  const handleModeChange = (
    event: React.MouseEvent<HTMLElement>,
    newMode: 'light' | 'dark' | 'system' | null,
  ) => {
    if (newMode !== null) {
      setMode(newMode);
      // 这里可以添加实际的主题切换逻辑
      localStorage.setItem('theme-mode', newMode);
    }
  };

  return (
    <ToggleButtonGroup
      value={mode}
      exclusive
      onChange={handleModeChange}
      aria-label="Theme mode"
      size={size}
    >
      <ToggleButton value="light" aria-label="Light mode">
        <WbSunnyRoundedIcon fontSize={size} />
      </ToggleButton>
      <ToggleButton value="system" aria-label="System mode">
        <SettingsBrightnessRoundedIcon fontSize={size} />
      </ToggleButton>
      <ToggleButton value="dark" aria-label="Dark mode">
        <ModeNightRoundedIcon fontSize={size} />
      </ToggleButton>
    </ToggleButtonGroup>
  );
}
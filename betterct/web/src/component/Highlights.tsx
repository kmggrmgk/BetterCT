import * as React from 'react';
import Box from '@mui/material/Box';
import Card from '@mui/material/Card';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
import AutoFixHighRoundedIcon from '@mui/icons-material/AutoFixHighRounded';
import ConstructionRoundedIcon from '@mui/icons-material/ConstructionRounded';
import QueryStatsRoundedIcon from '@mui/icons-material/QueryStatsRounded';
import SettingsSuggestRoundedIcon from '@mui/icons-material/SettingsSuggestRounded';
import SupportAgentRoundedIcon from '@mui/icons-material/SupportAgentRounded';
import ThumbUpAltRoundedIcon from '@mui/icons-material/ThumbUpAltRounded';

const items = [
    {
        icon: <SettingsSuggestRoundedIcon />,
        title: '适应性性能',
        description:
            '我们的产品能够轻松适应您的需求，提高效率，简化您的工作。',
    },
    {
        icon: <ConstructionRoundedIcon />,
        title: '经久耐用',
        description:
            '体验无与伦比的耐用性，实现超越期望的持久价值投资。',
    },
    {
        icon: <ThumbUpAltRoundedIcon />,
        title: '出色的用户体验',
        description:
            '通过直观且易于使用的界面，将我们的产品融入您的日常生活。',
    },
    {
        icon: <AutoFixHighRoundedIcon />,
        title: '创新功能',
        description:
            '通过引领新标准的功能保持领先，更好地满足您不断变化的需求。',
    },
    {
        icon: <SupportAgentRoundedIcon />,
        title: '可靠的支持',
        description:
            '依靠我们响应迅速的客户支持，提供超越购买本身的帮助。',
    },
    {
        icon: <QueryStatsRoundedIcon />,
        title: '精益求精',
        description:
            '享受一款精心打造的产品，每一个细节都能显著提升您的整体体验。',
    },
];

export default function Highlights() {
    return (
        <Box
            id="highlights"
            sx={{
                pt: { xs: 4, sm: 12 },
                pb: { xs: 8, sm: 16 },
                color: 'white',
                bgcolor: 'grey.900',
            }}
        >
            <Container
                sx={{
                    position: 'relative',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    gap: { xs: 3, sm: 6 },
                }}
            >
                <Box
                    sx={{
                        width: { sm: '100%', md: '60%' },
                        textAlign: { sm: 'left', md: 'center' },
                    }}
                >
                    <Typography component="h2" variant="h4" gutterBottom>
                        亮点
                    </Typography>
                    <Typography variant="body1" sx={{ color: 'grey.400' }}>
                        探索我们的产品为何与众不同：适应性、耐用性、用户友好设计和创新。享受可靠的客户支持和每一个细节的精确性。
                    </Typography>
                </Box>
                <Grid container spacing={2}>
                    {items.map((item, index) => (
                        <Grid size={{ xs: 12, sm: 6, md: 4 }} key={index}>
                            <Stack
                                direction="column"
                                component={Card}
                                spacing={1}
                                useFlexGap
                                sx={{
                                    color: 'inherit',
                                    p: 3,
                                    height: '100%',
                                    borderColor: 'hsla(220, 25%, 25%, 0.3)',
                                    backgroundColor: 'grey.800',
                                }}
                            >
                                <Box sx={{ opacity: '50%' }}>{item.icon}</Box>
                                <div>
                                    <Typography gutterBottom sx={{ fontWeight: 'medium' }}>
                                        {item.title}
                                    </Typography>
                                    <Typography variant="body2" sx={{ color: 'grey.400' }}>
                                        {item.description}
                                    </Typography>
                                </div>
                            </Stack>
                        </Grid>
                    ))}
                </Grid>
            </Container>
        </Box>
    );
}
import * as React from 'react';
import { Typography, Container, Box } from '@mui/material';

export default function TestPage() {
    return (
        <Container>
            <Box sx={{ py: 8, textAlign: 'center' }}>
                <Typography variant="h2" component="h1" gutterBottom>
                    BetterCT 首页
                </Typography>
                <Typography variant="h5" component="h2" color="text.secondary">
                    欢迎使用 BetterCT 平台
                </Typography>
                <Typography variant="body1" sx={{ mt: 4 }}>
                    这是一个测试页面，用于确认应用能够正常渲染。
                </Typography>
            </Box>
        </Container>
    );
}
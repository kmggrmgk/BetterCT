import * as React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';
import IconButton from '@mui/material/IconButton';
import InputLabel from '@mui/material/InputLabel';
import Link from '@mui/material/Link';
import Stack from '@mui/material/Stack';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import FacebookIcon from '@mui/icons-material/GitHub';
import LinkedInIcon from '@mui/icons-material/LinkedIn';
import TwitterIcon from '@mui/icons-material/X';
import SitemarkIcon from './SitemarkIcon';

function Copyright() {
    return (
        <Typography variant="body2" sx={{ color: 'text.secondary', mt: 1 }}>
            {'Copyright © '}
            <Link color="text.secondary" href="https://www.istaroth.xin">
                Istaroth
            </Link>
            &nbsp;
            {new Date().getFullYear()}
        </Typography>
    );
}

export default function Footer() {
    return (
        <Container
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                gap: { xs: 4, sm: 8 },
                py: { xs: 8, sm: 10 },
                textAlign: { sm: 'center', md: 'left' },
            }}
        >
            <Box
                sx={{
                    display: 'flex',
                    flexDirection: { xs: 'column', sm: 'row' },
                    width: '100%',
                    justifyContent: 'space-between',
                }}
            >
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'column',
                        gap: 4,
                        minWidth: { xs: '100%', sm: '60%' },
                    }}
                >
                    <Box sx={{ width: { xs: '100%', sm: '60%' } }}>
                        <SitemarkIcon />
                        <Typography variant="body2" gutterBottom sx={{ fontWeight: 600, mt: 2 }}>
                            订阅新闻通讯
                        </Typography>
                        <Typography variant="body2" sx={{ color: 'text.secondary', mb: 2 }}>
                            订阅以获取每周更新。绝不会发送垃圾邮件！
                        </Typography>
                        <InputLabel htmlFor="email-newsletter">电子邮箱</InputLabel>
                        <Stack direction="row" spacing={1} useFlexGap>
                            <TextField
                                id="email-newsletter"
                                hiddenLabel
                                size="small"
                                variant="outlined"
                                fullWidth
                                aria-label="请输入您的电子邮箱地址"
                                placeholder="您的电子邮箱地址"
                                slotProps={{
                                    htmlInput: {
                                        autoComplete: 'off',
                                        'aria-label': '请输入您的电子邮箱地址',
                                    },
                                }}
                                sx={{ width: '250px' }}
                            />
                            <Button
                                variant="contained"
                                color="primary"
                                size="small"
                                sx={{ flexShrink: 0 }}
                            >
                                订阅
                            </Button>
                        </Stack>
                    </Box>
                </Box>
                <Box
                    sx={{
                        display: { xs: 'none', sm: 'flex' },
                        flexDirection: 'column',
                        gap: 1,
                    }}
                >
                    <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                        产品
                    </Typography>
                    <Link color="text.secondary" variant="body2" href="#">
                        特点
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        用户评价
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        亮点
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        定价
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        常见问题（FAQs）
                    </Link>
                </Box>
                <Box
                    sx={{
                        display: { xs: 'none', sm: 'flex' },
                        flexDirection: 'column',
                        gap: 1,
                    }}
                >
                    <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                        公司
                    </Typography>
                    <Link color="text.secondary" variant="body2" href="#">
                        关于我们
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        职员
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        媒体
                    </Link>
                </Box>
                <Box
                    sx={{
                        display: { xs: 'none', sm: 'flex' },
                        flexDirection: 'column',
                        gap: 1,
                    }}
                >
                    <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                        法律
                    </Typography>
                    <Link color="text.secondary" variant="body2" href="#">
                        条款
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        隐私
                    </Link>
                    <Link color="text.secondary" variant="body2" href="#">
                        联系
                    </Link>
                </Box>
            </Box>
            <Box
                sx={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    pt: { xs: 4, sm: 8 },
                    width: '100%',
                    borderTop: '1px solid',
                    borderColor: 'divider',
                }}
            >
                <div>
                    <Link color="text.secondary" variant="body2" href="#">
                        隐私政策
                    </Link>
                    <Typography sx={{ display: 'inline', mx: 0.5, opacity: 0.5 }}>
                        &nbsp;•&nbsp;
                    </Typography>
                    <Link color="text.secondary" variant="body2" href="#">
                        服务条款
                    </Link>
                    <Copyright />
                </div>
                <Stack
                    direction="row"
                    spacing={1}
                    useFlexGap
                    sx={{ justifyContent: 'left', color: 'text.secondary' }}
                >
                    <IconButton
                        color="inherit"
                        size="small"
                        href="https://github.com/SolomonCaptain"
                        aria-label="GitHub"
                        sx={{ alignSelf: 'center' }}
                    >
                        <FacebookIcon />
                    </IconButton>
                    <IconButton
                        color="inherit"
                        size="small"
                        href="https://x.com/Istahhffuuf"
                        aria-label="X"
                        sx={{ alignSelf: 'center' }}
                    >
                        <TwitterIcon />
                    </IconButton>
                    <IconButton
                        color="inherit"
                        size="small"
                        href="https://www.linkedin.com/company/mui/"
                        aria-label="LinkedIn"
                        sx={{ alignSelf: 'center' }}
                    >
                        <LinkedInIcon />
                    </IconButton>
                </Stack>
            </Box>
        </Container>
    );
}
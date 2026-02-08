import * as React from 'react';
import Card from '@mui/material/Card';
import CardHeader from '@mui/material/CardHeader';
import CardContent from '@mui/material/CardContent';
import Avatar from '@mui/material/Avatar';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Grid from '@mui/material/Grid';
import { useTheme } from '@mui/system';

const userTestimonials = [
    {
        avatar: <Avatar alt="雷米·夏普" src="/static/images/avatar/1.jpg" />,
        name: '雷米·夏普',
        occupation: '高级工程师',
        testimonial:
            "我完全喜欢这款产品的多功能性！无论是处理工作项目还是享受我最喜欢的爱好，它都能无缝适应我不断变化的需求。它的直观设计真正提升了我的日常生活，让任务变得更加高效和愉快。",
    },
    {
        avatar: <Avatar alt="特拉维斯·霍华德" src="/static/images/avatar/2.jpg" />,
        name: '特拉维斯·霍华德',
        occupation: '首席产品设计师',
        testimonial:
            "这个产品的一个突出特点是卓越的客户支持。根据我的经验，这个产品背后的团队反应迅速，提供了非常有用的帮助。知道他们坚决支持自己的产品令人感到安心。",
    },
    {
        avatar: <Avatar alt="辛迪·贝克" src="/static/images/avatar/3.jpg" />,
        name: '辛迪·贝克',
        occupation: '首席技术官',
        testimonial:
            '这款产品的简便性和用户友好性大大简化了我的生活。我感谢创作者提供了一个不仅满足而且超越用户期望的解决方案。',
    },
    {
        avatar: <Avatar alt="雷米·夏普" src="/static/images/avatar/4.jpg" />,
        name: '朱莉娅·斯图尔特',
        occupation: '高级工程师',
        testimonial:
            "我很欣赏这个产品设计中的细节关注。小小的细节能带来很大的不同，很明显创作者们专注于提供高品质的体验。",
    },
    {
        avatar: <Avatar alt="特拉维斯·霍华德" src="/static/images/avatar/5.jpg" />,
        name: '约翰·史密斯',
        occupation: '产品设计师',
        testimonial:
            "我尝试过其他类似的产品，但这款产品因其创新的功能而脱颖而出。很明显，制造商在设计解决方案时投入了很多心思，真正满足了用户的需求。",
    },
    {
        avatar: <Avatar alt="辛迪·贝克" src="/static/images/avatar/6.jpg" />,
        name: '丹尼尔·沃尔夫',
        occupation: '首席数据官',
        testimonial:
            "这款产品的质量超过了我的预期。它耐用、设计精良，而且经久耐用。绝对值得投资！",
    },
];

const whiteLogos = [
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/6560628e8573c43893fe0ace_Sydney-white.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f4d520d0517ae8e8ddf13_Bern-white.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f46794c159024c1af6d44_Montreal-white.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/61f12e891fa22f89efd7477a_TerraLight.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/6560a09d1f6337b1dfed14ab_colorado-white.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f5caa77bf7d69fb78792e_Ankara-white.svg',
];

const darkLogos = [
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/6560628889c3bdf1129952dc_Sydney-black.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f4d4d8b829a89976a419c_Bern-black.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f467502f091ccb929529d_Montreal-black.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/61f12e911fa22f2203d7514c_TerraDark.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/6560a0990f3717787fd49245_colorado-black.svg',
    'https://assets-global.website-files.com/61ed56ae9da9fd7e0ef0a967/655f5ca4e548b0deb1041c33_Ankara-black.svg',
];

const logoStyle = {
    width: '64px',
    opacity: 0.3,
};

export default function Testimonials() {
    const theme = useTheme();
    const logos = theme.palette.mode === 'light' ? darkLogos : whiteLogos;

    return (
        <Container
            id="testimonials"
            sx={{
                pt: { xs: 4, sm: 12 },
                pb: { xs: 8, sm: 16 },
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
                <Typography
                    component="h2"
                    variant="h4"
                    gutterBottom
                    sx={{ color: 'text.primary' }}
                >
                    客户评价
                </Typography>
                <Typography variant="body1" sx={{ color: 'text.secondary' }}>
                    看看我们的客户为何喜爱我们的产品。了解我们在效率、耐用性和客户满意度方面的卓越表现。加入我们，享受高品质、创新和可靠的支持。
                </Typography>
            </Box>
            <Grid container spacing={2}>
                {userTestimonials.map((testimonial, index) => (
                    <Grid size={{ xs: 12, sm: 6, md: 4 }} key={index} sx={{ display: 'flex' }}>
                        <Card
                            variant="outlined"
                            sx={{
                                display: 'flex',
                                flexDirection: 'column',
                                justifyContent: 'space-between',
                                flexGrow: 1,
                            }}
                        >
                            <CardContent>
                                <Typography
                                    variant="body1"
                                    gutterBottom
                                    sx={{ color: 'text.secondary' }}
                                >
                                    {testimonial.testimonial}
                                </Typography>
                            </CardContent>
                            <Box
                                sx={{
                                    display: 'flex',
                                    flexDirection: 'row',
                                    justifyContent: 'space-between',
                                }}
                            >
                                <CardHeader
                                    avatar={testimonial.avatar}
                                    title={testimonial.name}
                                    subheader={testimonial.occupation}
                                />
                                <img
                                    src={logos[index]}
                                    alt={`Logo ${index + 1}`}
                                    style={logoStyle}
                                />
                            </Box>
                        </Card>
                    </Grid>
                ))}
            </Grid>
        </Container>
    );
}
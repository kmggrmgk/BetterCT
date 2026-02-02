import * as React from 'react';
import { Divider } from "@mui/material";
import AppTheme from '../shared-theme/AppTheme';
import AppAppBar from '../component/AppAppBar';
import Hero from '../component/Hero';
import LogoCollection from '../component/LogoCollection';
import Highlights from '../component/Highlights';
import Pricing from '../component/Pricing';
import Features from '../component/Features';
import Testimonials from '../component/Testimonials';
import FAQ from '../component/FAQ';
import Footer from '../component/Footer';

export default function IndexPage(props: { disableCustomTheme?: boolean }) {
    return (
        <AppTheme {...props}>
            <Divider />
            <AppAppBar />
            <Hero />
            <div>
                <LogoCollection />
                <Features />
                <Divider />
                <Testimonials />
                <Divider />
                <Highlights />
                <Divider />
                <Pricing />
                <Divider />
                <FAQ />
                <Divider />
                <Footer />
            </div>
        </AppTheme>
    );
}
import './styles/App.css'
import { SignedIn, SignedOut, RedirectToSignIn } from "@clerk/clerk-react";
import {Route, Routes} from "react-router-dom";
import IndexPage from "./pages/IndexPage.tsx";
import ProjectManagementMod from "./modules/ProjectManagementMod.tsx";
import SignInPage from "./pages/SignInPage.tsx";
import SignUpPage from "./pages/SignUpPage.tsx";

function ProtectedRoutes({children}) {
    return <>
        <SignedIn>
            {children}
        </SignedIn>
        <SignedOut>
            <RedirectToSignIn />
        </SignedOut>
    </>
}

function App() {
    return <Routes>
        <Route path="/">
            <Route index element={<IndexPage />} />
            <Route path={"sign-in/*"} element={<SignInPage />} />
            <Route path={"sign-up/*"} element={<SignUpPage />} />
            <Route
                path={"project-management"}
                element={
                    <ProtectedRoutes>
                        <ProjectManagementMod />
                    </ProtectedRoutes>
                }
            />
        </Route>
    </Routes>;
}

export default App;
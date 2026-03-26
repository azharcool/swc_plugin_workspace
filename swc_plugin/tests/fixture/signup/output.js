import { SignUp } from "@znode/base-components/components/signup";
import getThemeCookieServer from "@znode/utils/theme-resolver/theme-resolver.server";
interface ISignUpPageProps {
    searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
    const searchParams = await props.searchParams;
    return <SignUp searchParams={searchParams}/>;
}
// Detect Signup in source code.
// Get Props of Signup component if exists.
// Create a Wrapper Component for Signup
// Add props to the wrapper component
// Add theme detection logic in the wrapper component
// Add Theme condition rendering logic in the wrapper component
// Once the wrapper component is created, with proper props what it needs, conditions, and rendering logic.
// Finally replace the Signup component with the Wrapper component in the source code.
async function ThemeWrapper__SignUp(props: Readonly<ISignUpPageProps>) {
    const theme = await getThemeCookieServer();
    if (theme === "custom1") {
        return <SignUp searchParams={props.searchParams}/>;
    }
    return <SignUp searchParams={props.searchParams}/>;
}

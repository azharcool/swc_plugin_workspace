import { SignUp } from "@znode/base-components/components/signup";
import getThemeCookieServer from "@znode/utils/theme-resolver/theme-resolver.server";
import { SignUp as SignUp__custom1 } from "@znode/custom1-package/component/signup";
interface ISignUpPageProps {
    searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
    return <SignUp searchParams={searchParams}/>;
}
async function ThemeWrapper__base(props: Readonly<ISignUpPageProps>) {
    const theme = await getThemeCookieServer();
    if (theme === "custom1") {
        return <SignUp searchParams={props.searchParams}/>;
    }
    return <SignUp searchParams={props.searchParams}/>;
}
async function ThemeWrapper__base() {
    const themeName = await getThemeCookieServer();
}

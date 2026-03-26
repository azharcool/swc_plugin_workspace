import { SignUp } from "@znode/base-components/components/signup";
import getThemeCookieServer from "@znode/utils/theme-resolver/theme-resolver.server";
import { SignUp as SignUp__custom1 } from "@znode/custom1-package/component/signup";
import { SignUp as SignUp__custom2 } from "@znode/custom2-package/component/signup";
interface ISignUpPageProps {
    searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
    const searchParams = await props.searchParams;
    return <div>
      <ThemeWrapper__SignUp searchParams={searchParams}/>;
    </div>;
}
async function ThemeWrapper__SignUp(props) {
    const themeName = await getThemeCookieServer();
    if (themeName === "custom1") {
        return <SignUp__custom1 {...props}/>;
    }
    if (themeName === "custom2") {
        return <SignUp__custom2 {...props}/>;
    }
    return <SignUp {...props}/>;
}

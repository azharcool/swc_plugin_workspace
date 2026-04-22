import { BreadCrumbs } from "@znode/base-components/common/breadcrumb";
import { ISearchUrl } from "@znode/types/common";
import { NextIntlClientProvider } from "next-intl";
import { SignUp } from "@znode/base-components/components/signup";
import { getResourceMessages } from "@znode/utils/server";
import getThemeCookieServer from "@znode/utils/theme-resolver/theme-resolver.server";
import { SignUp as SignUp__custom1 } from "@znode/custom1-package/component/signup";
import { SignUp as SignUp__custom2 } from "@znode/custom2-package/component/signup";
interface ISignUpPageProps {
    searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
    const searchParams = await props.searchParams;
    const breadCrumbsData = {
        title: "Create Account",
        routingLabel: "Home",
        routingPath: "/"
    };
    const registerMessages = await getResourceMessages("Register");
    const signUpMessages = await getResourceMessages("SignUp");
    const commonMessages = await getResourceMessages("Common");
    return <NextIntlClientProvider messages={{
        ...registerMessages,
        ...signUpMessages,
        ...commonMessages
    }}>
      <div>
        <BreadCrumbs customPath={breadCrumbsData}/>
        <ThemeWrapper__SignUp searchParams={searchParams}/>
      </div>
    </NextIntlClientProvider>;
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

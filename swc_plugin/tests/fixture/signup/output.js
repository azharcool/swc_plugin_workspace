import { BreadCrumbs } from "@znode/base-components/common/breadcrumb";
import { ISearchUrl } from "@znode/types/common";
import { NextIntlClientProvider } from "next-intl";
import { SignUp as BaseSignUp } from "@znode/base-components/components/signup";
import { SignUp as Custom1SignUp } from "@znode/custom1/components/signup";
import { getThemeCookieServer } from "@znode/utils/theme-cookie/theme-cookie.server";
import { getResourceMessages } from "@znode/utils/server";
interface ISignUpPageProps {
    searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
    const themeName = await getThemeCookieServer();
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
        {themeName === "custom1" ? <Custom1SignUp searchParams={searchParams}/> : <BaseSignUp searchParams={searchParams}/>}
      </div>
    </NextIntlClientProvider>;
}

import { SignUp } from "@znode/base-components/components/signup";

interface ISignUpPageProps {
  searchParams: Promise<ISearchUrl>;
}

export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
  return <SignUp searchParams={searchParams} />;
}




async function ThemeWrapper__base(props: Readonly<ISignUpPageProps>)  {
  const theme = await getThemeCookieServer();
  
  if(theme === "custom1") {
    return <SignUp searchParams={props.searchParams} />;
  }
  
  return <SignUp searchParams={props.searchParams} />;
}
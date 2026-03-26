import { SignUp } from "@znode/base-components/components/signup";
interface ISignUpPageProps {
  searchParams: Promise<ISearchUrl>;
}
export default async function SignUpPage(props: Readonly<ISignUpPageProps>) {
  const searchParams = await props.searchParams;

  return (
    <div>
      <SignUp searchParams={searchParams} />;
    </div>
  );
}

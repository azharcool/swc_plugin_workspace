import { NextIntlClientProvider } from "next-intl";
import Client from "./client";
import { fetchMessages, getPortalHeader } from "@znode/utils/server";
import { getPage } from "@znode/page-builder/utils/get-page";
import { IPageStructure } from "@znode/types/visual-editor";
import { ISearchParams } from "@znode/types/search-params";
import { NotFound } from "@znode/base-components/components/not-found";
import getThemeCookieServer from "@znode/utils/theme-resolver/theme-resolver.server";
import { Client as Client__custom1 } from "@znode/custom1-package/component/client";
import Client__custom2 from "@znode/custom2-package/component/client";
const localeMessages = [
    "Blog",
    "Product",
    "Barcode",
    "Common",
    "Layout",
    "DropDown",
    "Facet",
    "Pagination",
    "FacetChipList",
    "Addon",
    "Price",
    "StoreLocator",
    "Email",
    "Inventory",
    "WishList"
];
interface IHomePageProps {
    searchParams: Promise<ISearchParams>;
}
export default async function Home(props: Readonly<IHomePageProps>) {
    const searchParams = await props.searchParams;
    const themeName = (await getPortalHeader()).themeName || process.env.DEFAULT_THEME as string;
    const url = "home";
    const pageStructure: IPageStructure = await getPage({
        url,
        searchParams: searchParams,
        theme: themeName
    });
    if (!pageStructure.data.content.length) {
        return <NotFound/>;
    }
    const messages = await fetchMessages(localeMessages);
    return <NextIntlClientProvider messages={{
        ...messages
    }}>
      <ThemeWrapper__Client data={pageStructure.data} themeName={themeName || ""} configType="common"/>
    </NextIntlClientProvider>;
}
async function ThemeWrapper__Client(props) {
    const themeName = await getThemeCookieServer();
    if (themeName === "custom1") {
        return <Client__custom1 {...props}/>;
    }
    if (themeName === "custom2") {
        return <Client__custom2 {...props}/>;
    }
    return <Client {...props}/>;
}

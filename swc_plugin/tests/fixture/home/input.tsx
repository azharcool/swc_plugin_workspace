import { NextIntlClientProvider } from "next-intl";
import Client from "./client";
import { fetchMessages, getPortalHeader } from "@znode/utils/server";
import { getPage } from "@znode/page-builder/utils/get-page";
import { IPageStructure } from "@znode/types/visual-editor";
import { ISearchParams } from "@znode/types/search-params";
import { NotFound } from "@znode/base-components/components/not-found";

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
  "WishList",
];

interface IHomePageProps {
  searchParams: Promise<ISearchParams>;
}

export default async function Home(props: Readonly<IHomePageProps>) {
  const searchParams = await props.searchParams;
  const themeName = (await getPortalHeader()).themeName || (process.env.DEFAULT_THEME as string);
  const url = "home";
  const pageStructure: IPageStructure = await getPage({
    url,
    searchParams: searchParams,
    theme: themeName, // theme1, theme2
  });

  if (!pageStructure.data.content.length) {
    return <NotFound />;
  }

  const messages = await fetchMessages(localeMessages);

  return (
    <NextIntlClientProvider messages={{ ...messages }}>
      <Client data={pageStructure.data} themeName={themeName || ""} configType="common" />
    </NextIntlClientProvider>
  );
}


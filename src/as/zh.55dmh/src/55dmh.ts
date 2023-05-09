import {
  Chapter,
  Filter,
  FilterType,
  Html,
  HttpMethod,
  Listing,
  Manga,
  MangaContentRating,
  MangaPageResult,
  MangaStatus,
  MangaViewer,
  Page,
  Request,
  Source,
} from "aidoku-as/src";

const FILTER_CATEGORY = [
  "",
  "rexue",
  "wuxia",
  "kehuan",
  "tuili",
  "danmei",
  "kongbu",
  "shaonv",
  "lianai",
  "shenghuo",
  "zhanzheng",
];
const FILTER_AUDIENCE = ["", "ertong", "shaonian", "shaonv", "qingnian"];
const FILTER_STATUS = ["", "wanjie", "lianzai"];
const FILTER_SORT = ["click", "update", "post"];

export class Wdmh extends Source {
  genExploreURL(category: string, audience: string, status: string, sort: string, page: number): string {
    let url = "https://www.55dmh.com/list/";
    const keywords = [category, audience, status].filter((keyword: string) => !!keyword);
    if (keywords.length > 0) url += `${keywords.join("-")}/`;
    url += `${sort}/`;
    if (page > 1) url += `?page=${page as i32}`;
    return url;
  }

  genSearchURL(query: string, page: number): string {
    return `https://www.55dmh.com/search/?keywords=${encodeURI(query)}&page=${page as i32}`;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return Html.parse(String.UTF8.encode(request.string()));
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";
    let catetory = "";
    let audience = "";
    let status = "";
    let sort = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
      if (filter.type === FilterType.Select) {
        const index = filter.value.toInteger() as i32;
        if (filter.name === "题材") {
          catetory = FILTER_CATEGORY[index];
        }
        if (filter.name === "读者") {
          audience = FILTER_AUDIENCE[index];
        }
        if (filter.name === "进度") {
          status = FILTER_STATUS[index];
        }
      }
      if (filter.type === FilterType.Sort) {
        const value = filter.value.asObject();
        const index = value.get("index").toInteger() as i32;
        const ascending = value.get("ascending").toBool();
        sort = ascending ? `-${FILTER_SORT[index]}` : FILTER_SORT[index];
      }
    }

    const url =
      query === "" ? this.genExploreURL(catetory, audience, status, sort, page) : this.genSearchURL(query, page);
    const html = this.getHTML(url);
    const list = html.select("#dmList>ul>li").array();
    const hasMore = true;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item
        .select(".cover")
        .attr("href")
        .split("/")
        .filter((a: string) => !!a)
        .pop();
      const title = item.select("dl>dt>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.select(".cover>img").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaListing(listing: Listing, page: number): MangaPageResult {
    let key = "";

    if (listing.name === "最近更新") {
      key = "recent";
    }

    const url = `https://www.55dmh.com/main/${key}/`;
    const html = this.getHTML(url);
    const list = html.select(".updateList>ul>li>a[i]").array();
    const hasMore = false;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item
        .attr("href")
        .split("/")
        .filter((a: string) => !!a)
        .pop();
      const title = item.text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.attr("i");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://www.55dmh.com/manhua/${mangaId}/`;
    const html = this.getHTML(url);
    const title = html.select("h3[class]").text().trim().replace("简介：", "");
    const manga = new Manga(mangaId, title);
    manga.cover_url = html.select(".cover>img").attr("src");
    manga.author = html.select(".info>p:nth-child(2)>a").text().trim().split(",").join(", ");
    manga.artist = "";
    manga.description = html
      .select(".introduction>p:nth-child(1)")
      .text()
      .replace("漫画简介：", "")
      .replace("介绍:", "")
      .trim();
    manga.url = url;
    manga.categories = html
      .select(".info>p:nth-child(5)>a")
      .array()
      .map((a: Html) => a.text().trim());
    manga.status = MangaStatus.Unknown;
    manga.rating = MangaContentRating.Safe;
    manga.viewer = MangaViewer.RTL;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://www.55dmh.com/manhua/${mangaId}/`;
    const html = this.getHTML(url);
    const list = html.select("#chapter-list-1>li>a").array();
    const last = list.length - 1;
    const chapters: Chapter[] = [];

    for (let i = last; i >= 0; i--) {
      const item = list[last - i];
      const id = item.attr("href").split("/").pop().replace(".html", "");
      const ids = `${mangaId}/${id}`;
      const title = item.text().trim();
      const chapter = new Chapter(ids, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = `https://www.55dmh.com/manhua/${ids}.html`;
      chapters.push(chapter);
    }

    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://www.55dmh.com/manhua/${chapterId}.html`;
    const html = this.getHTML(url);
    const list = html.select("#imagesOld>img").array();
    const pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page(i as i32);
      page.url = item.attr("data-original");
      pages.push(page);
    }

    return pages;
  }
}

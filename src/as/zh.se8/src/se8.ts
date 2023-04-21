import {
  Chapter,
  Filter,
  FilterType,
  Html,
  HttpMethod,
  JSON,
  Manga,
  MangaContentRating,
  MangaPageResult,
  MangaStatus,
  MangaViewer,
  Page,
  Request,
  Source,
} from "aidoku-as/src";

const FILTER_TAG = [
  "",
  "61",
  "63",
  "62",
  "64",
  "11",
  "15",
  "17",
  "29",
  "31",
  "67",
  "68",
  "69",
  "75",
  "78",
  "84",
  "86",
  "87",
  "91",
  "98",
  "106",
  "114",
];
const FILTER_FINISH = ["", "1", "2"];
const FILTER_ORDER = ["hits", "addtime"];

export class Se8 extends Source {
  genExploreURL(tag: string, finish: string, order: string, page: number): string {
    let url = "https://se8.us/index.php/category";
    if (tag !== "") url += `/tags/${tag}`;
    if (finish !== "") url += `/finish/${finish}`;
    return url + `/order/${order}/page/${page as i32}`;
  }

  genSearchURL(query: string, page: number): string {
    return `https://se8.us/index.php/search/${encodeURI(query)}/${page as i32}`;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getJSON(url: string): JSON {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.json();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";
    let tag = "";
    let finish = "";
    let order = "hits";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
      if (filter.type === FilterType.Select) {
        const index = filter.value.toInteger() as i32;
        if (filter.name === "标签") {
          tag = FILTER_TAG[index];
        }
        if (filter.name === "进度") {
          finish = FILTER_FINISH[index];
        }
      }
      if (filter.type === FilterType.Sort) {
        const value = filter.value.asObject();
        const index = value.get("index").toInteger() as i32;
        order = FILTER_ORDER[index];
      }
    }

    const url = query === "" ? this.genExploreURL(tag, finish, order, page) : this.genSearchURL(query, page);
    const html = this.getHTML(url);
    const list = html.select(".common-comic-item").array();
    const hasMore = true;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.select("a").attr("href").split("/").pop();
      const title = item.select("p:nth-child(2)>a").text();
      const manga = new Manga(id, title);
      manga.cover_url = item.select("a>img").attr("data-original");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://se8.us/index.php/comic/${mangaId}`;
    const html = this.getHTML(url);
    const id = html.select(".j-user-collect").attr("data-id");
    const title = html.select(".j-comic-title").text();
    const manga = new Manga(id, title);
    manga.cover_url = html.select(".de-info__cover>img").attr("src");
    manga.author = html
      .select(".comic-author>.name>a")
      .text()
      .replaceAll("&amp", "&")
      .split("&")
      .filter((a: string) => !!a.trim())
      .join(", ");
    manga.artist = "";
    manga.description = html.select(".comic-intro>.intro").text().trim().replaceAll("&hellip", "…");
    manga.url = url;
    manga.categories = html
      .select(".comic-status>span:nth-child(1)>b>a")
      .array()
      .map((a: Html) => a.text().trim())
      .filter((a: string) => !!a);
    manga.status = MangaStatus.Ongoing;
    manga.rating = MangaContentRating.NSFW;
    manga.viewer = MangaViewer.Scroll;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://se8.us/index.php/api/comic/chapter?mid=${mangaId}`;
    const json = this.getJSON(url);
    const list = json.asObject().get("data").asArray().toArray();
    const chapters: Chapter[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i].asObject();
      const id = item.get("id").toString();
      const title = item
        .get("name")
        .toString()
        .trim()
        .replaceAll("&lt;", "<")
        .replaceAll("&gt;", ">")
        .replaceAll("&#40;", "(")
        .replaceAll("&#41;", ")")
        .replaceAll("&ldquo;", "“")
        .replaceAll("&rdquo;", "”")
        .replaceAll("&hellip;", "…")
        .replaceAll("&hearts;", "♥");
      const chapter = new Chapter(id, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = item.get("link").toString();
      chapters.push(chapter);
    }

    chapters.reverse();
    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://se8.us/index.php/chapter/${chapterId}`;
    const html = this.getHTML(url);
    const list = html.select("div[id^='pic']>img").array();
    const pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page(i);
      page.url = item.attr("data-original").trim();
      pages.push(page);
    }

    return pages;
  }
}

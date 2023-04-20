import {
  Chapter,
  Filter,
  FilterType,
  Html,
  HttpMethod,
  Manga,
  MangaContentRating,
  MangaPageResult,
  MangaStatus,
  MangaViewer,
  Page,
  Request,
  Source,
} from "aidoku-as/src";

const FILTER_TYPE = [
  "all",
  "lianai",
  "chunai",
  "gufeng",
  "yineng",
  "xuanyi",
  "juqing",
  "kehuan",
  "qihuan",
  "xuanhuan",
  "chuanyue",
  "maoxian",
  "tuili",
  "wuxia",
  "gedou",
  "zhanzheng",
  "rexie",
  "gaoxiao",
  "danuzhu",
  "dushi",
  "zongcai",
  "hougong",
  "richang",
  "hanman",
  "shaonian",
  "qita",
];
const FILTER_REGION = ["all", "cn", "jp", "kr", "en"];
const FILTER_STATE = ["all", "serial", "pub"];

export class Baozimh extends Source {
  genExploreURL(type: string, region: string, state: string, page: number): string {
    return `https://www.baozimh.com/classify?type=${type}&region=${region}&state=${state}&page=${page as i32}`;
  }

  genSearchURL(query: string): string {
    return `https://www.baozimh.com/search/?q=${encodeURI(query)}`;
  }

  transMangaStatus(status: string): MangaStatus {
    if (status === "连载中") return MangaStatus.Ongoing;
    if (status === "已完结") return MangaStatus.Completed;
    return MangaStatus.Unknown;
  }

  hasMorePages(chapterId: string, nextChapterId: string): boolean {
    return chapterId.split("_").slice(0, 2).join("_") === nextChapterId.split("_").slice(0, 2).join("_");
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";
    let type = "all";
    let region = "all";
    let state = "all";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
      if (filter.type === FilterType.Select) {
        const index = filter.value.toInteger() as i32;
        if (filter.name === "类型") {
          type = FILTER_TYPE[index];
        }
        if (filter.name === "地区") {
          region = FILTER_REGION[index];
        }
        if (filter.name === "状态") {
          state = FILTER_STATE[index];
        }
      }
    }

    const url = query === "" ? this.genExploreURL(type, region, state, page) : this.genSearchURL(query);
    const html = this.getHTML(url);
    const list = html.select(".pure-g>.comics-card").array();
    const hasMore = query === "";
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.select(".comics-card__info").attr("href").split("/").pop();
      const title = item.select(".comics-card__title").text();
      const manga = new Manga(id, title);
      manga.cover_url = item.select(".comics-card__poster>amp-img").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://www.baozimh.com/comic/${mangaId}`;
    const html = this.getHTML(url);
    const title = html.select("meta[name='og:novel:book_name']").attr("content");
    const manga = new Manga(mangaId, title);
    manga.cover_url = html.select("meta[name='og:image'").attr("content");
    manga.author = html.select("meta[name='og:novel:author']").attr("content");
    manga.artist = "";
    manga.description = html.select("meta[name='og:description']").attr("content").split(",").slice(2).join("");
    manga.url = url;
    manga.categories = html
      .select("meta[name='og:novel:category']")
      .attr("content")
      .split(",")
      .filter((a: string) => !a.startsWith("types"));
    manga.status = this.transMangaStatus(html.select("meta[name='og:novel:status']").attr("content"));
    manga.rating = MangaContentRating.Safe;
    manga.viewer = MangaViewer.RTL;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://www.baozimh.com/comic/${mangaId}`;
    const html = this.getHTML(url);
    const list = html.select("div[id^='chapter']>div>a").array();
    const chapters: Chapter[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const chapterId = item
        .attr("href")
        .split("&")
        .slice(1)
        .map((a: string) => a.split("=")[1])
        .join("_");
      const ids = `${mangaId}/${chapterId}`;
      const title = item.select("div>span").text();
      const chapter = new Chapter(ids, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = `https://cn.kukuc.co/comic/chapter/${ids}.html`;
      chapters.push(chapter);
    }

    chapters.reverse();
    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const ids = chapterId.split("/");
    return this.getPageListWithOffset(ids[0], ids[1], 0);
  }

  getPageListWithOffset(mangaId: string, chapterId: string, offset: number): Page[] {
    const url = `https://cn.kukuc.co/comic/chapter/${mangaId}/${chapterId}.html`;
    const html = this.getHTML(url);
    const list = html.select("amp-img[id^='chapter-img']").array();
    const nextChapterId = html.select("#next-chapter").attr("href").split("/").pop().replace(".html", "");
    let pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page((i + offset) as i32);
      page.url = item.attr("src");
      pages.push(page);
    }

    if (this.hasMorePages(chapterId, nextChapterId)) {
      pages = pages.concat(this.getPageListWithOffset(mangaId, nextChapterId, list.length));
    }

    return pages;
  }
}

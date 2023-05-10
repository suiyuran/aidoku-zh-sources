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

const FILTER_TAG = ["全部"];
const FILTER_AREA = ["-1", "1", "2"];
const FILTER_END = ["-1", "0", "1"];

export class Mxshm extends Source {
  genExploreURL(tag: string, area: string, end: string, page: number): string {
    return `http://www.mxshm.site/booklist?tag=${encodeURI(tag)}&area=${area}&end=${end}&page=${page as i32}`;
  }

  genSearchURL(query: string): string {
    return `http://www.mxshm.site/search?keyword=${encodeURI(query)}`;
  }

  transMangaStatus(status: string): MangaStatus {
    if (status === "连载中") return MangaStatus.Ongoing;
    if (status === "已完结") return MangaStatus.Completed;
    return MangaStatus.Unknown;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";
    let tag = "";
    let area = "";
    let end = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
      if (filter.type === FilterType.Select) {
        const index = filter.value.toInteger() as i32;
        if (filter.name === "题材") {
          tag = FILTER_TAG[index];
        }
        if (filter.name === "地区") {
          area = FILTER_AREA[index];
        }
        if (filter.name === "进度") {
          end = FILTER_END[index];
        }
      }
    }

    const notSearch = query === "";
    const url = notSearch ? this.genExploreURL(tag, area, end, page) : this.genSearchURL(query);
    const html = this.getHTML(url);
    const list = html.select(".mh-item").array();
    const hasMore = notSearch;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.select("a").attr("href").split("/").pop();
      const title = item.select(".mh-item-detali>h2>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.select("a>p").attr("style").replace("background-image: url(", "").replace(")", "");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `http://www.mxshm.site/book/${mangaId}`;
    const html = this.getHTML(url);
    const id = mangaId;
    const title = html.select(".info>h1").text().trim();
    const manga = new Manga(id, title);
    manga.cover_url = html.select(".banner_detail_form>.cover>img").attr("src");
    manga.author = html.select(".info>p:nth-child(4)").text().trim().replace("作者：", "").split("&").join(", ");
    manga.artist = "";
    manga.description = html.select(".info>.content>span>span").text().trim();
    manga.url = url;
    manga.categories = html
      .select(".info>p:nth-child(6)>span>a")
      .array()
      .map((a: Html) => a.text().trim())
      .filter((a: string) => !!a);
    manga.status = this.transMangaStatus(html.select(".info>p:nth-child(5)>span>span").text().trim());
    manga.rating = MangaContentRating.NSFW;
    manga.viewer = MangaViewer.Scroll;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `http://www.mxshm.site/book/${mangaId}`;
    const html = this.getHTML(url);
    const list = html.select("#detail-list-select>li>a").array();
    const chapters: Chapter[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.attr("href").split("/").pop();
      const title = item.text().trim();
      const chapter = new Chapter(id, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = `http://www.mxshm.site/chapter/${id}`;
      chapters.push(chapter);
    }

    chapters.reverse();
    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const url = `http://www.mxshm.site/chapter/${chapterId}`;
    const html = this.getHTML(url);
    const list = html.select(".comicpage>div>img").array();
    const pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page(i);
      page.url = item.attr("data-original");
      pages.push(page);
    }

    return pages;
  }
}

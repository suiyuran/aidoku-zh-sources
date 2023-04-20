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

export class Isekai extends Source {
  genExploreURL(page: number): string {
    return `https://isekai.ch/comic/page/${page as i32}`;
  }

  genSearchURL(query: string, page: number): string {
    return `https://isekai.ch/page/${page as i32}/?cat=1&s=${encodeURI(query)}`;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
    }

    const url = query === "" ? this.genExploreURL(page) : this.genSearchURL(query, page);
    const html = this.getHTML(url);
    const list = html.select(".card").array();
    const hasMore = true;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item
        .select("a")
        .attr("href")
        .split("/")
        .filter((a: string) => !!a)
        .pop();
      const title = item.select("a>p").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.select("a>div").attr("style").replace("background-image:url(", "").replace(")", "");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://isekai.ch/${mangaId}`;
    const html = this.getHTML(url);
    const title = html.select(".card>.card-header").text().trim();
    const manga = new Manga(mangaId, title);
    manga.cover_url = html
      .select(".card>.card-body>div>div:nth-child(1)>div>div>div")
      .attr("style")
      .replace("background-image:url(", "")
      .replace(")", "");
    manga.author = "";
    manga.artist = "";
    manga.description = "";
    manga.url = url;
    manga.categories = html.select("meta[name='keywords']").attr("content").split(",");
    manga.status = MangaStatus.Unknown;
    manga.rating = MangaContentRating.NSFW;
    manga.viewer = MangaViewer.RTL;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://isekai.ch/${mangaId}`;
    const chapter = new Chapter(mangaId, "第 1 话");
    chapter.chapter = 1 as f32;
    chapter.url = url;
    return [chapter];
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://isekai.ch/${chapterId}`;
    const html = this.getHTML(url);
    const list = html.select("img[data-original]").array();
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

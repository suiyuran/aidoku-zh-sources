#![expect(clippy::unwrap_used)]

use super::*;
use aidoku_test::aidoku_test;

#[aidoku_test]
fn manga_31164() {
	let chapters: Vec<Chapter> = serde_json::from_str::<Root>(
		r#"{"code":1,"result":{"lastPage":true,"pageNumber":1,"pageSize":1000,"totalRow":9,"totalPage":1,"list":[{"id":2639178,"title":"第12話","sort":9,"score":0,"create_time":1750567096},{"id":2638676,"title":"第11話","sort":8,"score":0,"create_time":1750225251},{"id":2638572,"title":"第10話","sort":7,"score":0,"create_time":1750141870},{"id":2638571,"title":"第09話","sort":6,"score":0,"create_time":1750141826},{"id":2637978,"title":"第08話","sort":5,"score":0,"create_time":1749621830},{"id":2637904,"title":"第07話","sort":4,"score":0,"create_time":1749537750},{"id":2637677,"title":"第06話","sort":3,"score":0,"create_time":1749272604},{"id":2637676,"title":"第03-05話","sort":2,"score":0,"create_time":1749272596},{"id":2637675,"title":"第01-02話","sort":1,"score":0,"create_time":1749272587}],"history":[]},"msg":"ok","succ":true}"#,
	)
	.unwrap()
	.into();
	assert_eq!(chapters.len(), 9);
	assert_eq!(
		*chapters.first().unwrap(),
		Chapter {
			key: "2639178".into(),
			chapter_number: Some(12.0),
			date_uploaded: Some(1_750_567_096),
			url: Some("https://boylove.cc/home/book/capter/id/2639178".into()),
			..Default::default()
		}
	);
	assert_eq!(
		*chapters.last().unwrap(),
		Chapter {
			key: "2637675".into(),
			title: Some("第01-02話".into()),
			chapter_number: Some(1.0),
			date_uploaded: Some(1_749_272_587),
			url: Some("https://boylove.cc/home/book/capter/id/2637675".into()),
			..Default::default()
		}
	);
}

use std::collections::HashMap;

/// ref: https://weather.tsukumijima.net/primary_area.xml
/// ref: https://www.gsi.go.jp/kihonjohochousa/gazetteer.html
fn make_db() -> HashMap<String, String> {
  let mut map = HashMap::new();
  map.insert("稚内".to_string(), "011000".to_string());
  map.insert("旭川".to_string(), "012010".to_string());
  map.insert("留萌".to_string(), "012020".to_string());

  map.insert("wakkanai".to_string(), "011000".to_string());
  map.insert("asahikawa".to_string(), "012010".to_string());
  map.insert("rumoi".to_string(), "012020".to_string());

  map.insert("網走".to_string(), "013010".to_string());
  map.insert("北見".to_string(), "013020".to_string());
  map.insert("紋別".to_string(), "013030".to_string());
  map.insert("根室".to_string(), "014010".to_string());
  map.insert("釧路".to_string(), "014020".to_string());
  map.insert("帯広".to_string(), "014030".to_string());

  map.insert("abashiri".to_string(), "013010".to_string());
  map.insert("kitami".to_string(), "013020".to_string());
  map.insert("monbetsu".to_string(), "013030".to_string());
  map.insert("nemuro".to_string(), "014010".to_string());
  map.insert("kushiro".to_string(), "014020".to_string());
  map.insert("obihiro".to_string(), "014030".to_string());

  map.insert("室蘭".to_string(), "015010".to_string());
  map.insert("浦河".to_string(), "015020".to_string());

  map.insert("muroran".to_string(), "015010".to_string());
  map.insert("urakawa".to_string(), "015020".to_string());

  map.insert("札幌".to_string(), "016010".to_string());
  map.insert("岩見沢".to_string(), "016020".to_string());
  map.insert("倶知安".to_string(), "016030".to_string());

  map.insert("sapporo".to_string(), "016010".to_string());
  map.insert("iwamizawa".to_string(), "016020".to_string());
  map.insert("kutchan".to_string(), "016030".to_string());

  map.insert("函館".to_string(), "017010".to_string());
  map.insert("江差".to_string(), "017020".to_string());

  map.insert("hakodate".to_string(), "017010".to_string());
  map.insert("esashi".to_string(), "017020".to_string());

  map.insert("青森".to_string(), "020010".to_string());
  map.insert("むつ".to_string(), "020020".to_string());
  map.insert("八戸".to_string(), "020030".to_string());

  map.insert("aomori".to_string(), "020010".to_string());
  map.insert("mutsu".to_string(), "020020".to_string());
  map.insert("hachinohe".to_string(), "020030".to_string());

  map.insert("盛岡".to_string(), "030010".to_string());
  map.insert("宮古".to_string(), "030020".to_string());
  map.insert("大船渡".to_string(), "030030".to_string());

  map.insert("morioka".to_string(), "030010".to_string());
  map.insert("miyako".to_string(), "030020".to_string());
  map.insert("ofunato".to_string(), "030030".to_string());

  map.insert("仙台".to_string(), "040010".to_string());
  map.insert("白石".to_string(), "040020".to_string());

  map.insert("sendai".to_string(), "040010".to_string());
  map.insert("shiroishi".to_string(), "040020".to_string());

  map.insert("秋田".to_string(), "050010".to_string());
  map.insert("横手".to_string(), "050020".to_string());

  map.insert("akita".to_string(), "050010".to_string());
  map.insert("yokote".to_string(), "050020".to_string());

  map.insert("山形".to_string(), "060010".to_string());
  map.insert("米沢".to_string(), "060020".to_string());
  map.insert("酒田".to_string(), "060030".to_string());
  map.insert("新庄".to_string(), "060040".to_string());

  map.insert("yamagata".to_string(), "060010".to_string());
  map.insert("yonezawa".to_string(), "060020".to_string());
  map.insert("sakata".to_string(), "060030".to_string());
  map.insert("sinjo".to_string(), "060040".to_string());

  map.insert("福島".to_string(), "070010".to_string());
  map.insert("小名浜".to_string(), "070020".to_string());
  map.insert("若松".to_string(), "070030".to_string());

  map.insert("fukushima".to_string(), "070010".to_string());
  map.insert("onahama".to_string(), "070020".to_string());
  map.insert("wakamatsu".to_string(), "070030".to_string());

  map.insert("水戸".to_string(), "080010".to_string());
  map.insert("土浦".to_string(), "080020".to_string());

  map.insert("mito".to_string(), "080010".to_string());
  map.insert("tsuchiura".to_string(), "080020".to_string());

  map.insert("宇都宮".to_string(), "090010".to_string());
  map.insert("大田原".to_string(), "090020".to_string());

  map.insert("utsunomiya".to_string(), "090010".to_string());
  map.insert("otawara".to_string(), "090020".to_string());

  map.insert("前橋".to_string(), "100010".to_string());
  map.insert("みなかみ".to_string(), "100020".to_string());

  map.insert("maebashi".to_string(), "100010".to_string());
  map.insert("minakami".to_string(), "100020".to_string());

  map.insert("さいたま".to_string(), "110010".to_string());
  map.insert("熊谷".to_string(), "110020".to_string());
  map.insert("秩父".to_string(), "110030".to_string());

  map.insert("saitama".to_string(), "110010".to_string());
  map.insert("kumagaya".to_string(), "110020".to_string());
  map.insert("chichibu".to_string(), "110030".to_string());

  map.insert("千葉".to_string(), "120010".to_string());
  map.insert("銚子".to_string(), "120020".to_string());
  map.insert("館山".to_string(), "120030".to_string());

  map.insert("chiba".to_string(), "120010".to_string());
  map.insert("choshi".to_string(), "120020".to_string());
  map.insert("tateyama".to_string(), "120030".to_string());

  map.insert("東京".to_string(), "130010".to_string());
  map.insert("大島".to_string(), "130020".to_string());
  map.insert("八丈島".to_string(), "130030".to_string());
  map.insert("父島".to_string(), "130040".to_string());

  map.insert("tokyo".to_string(), "130010".to_string());
  map.insert("oshima".to_string(), "130020".to_string());
  map.insert("hachijojima".to_string(), "130030".to_string());
  map.insert("chichijima".to_string(), "130040".to_string());

  map.insert("横浜".to_string(), "140010".to_string());
  map.insert("小田原".to_string(), "140020".to_string());

  map.insert("yokohama".to_string(), "140010".to_string());
  map.insert("odawara".to_string(), "140020".to_string());

  map.insert("新潟".to_string(), "150010".to_string());
  map.insert("長岡".to_string(), "150020".to_string());
  map.insert("高田".to_string(), "150030".to_string());
  map.insert("相川".to_string(), "150040".to_string());

  map.insert("niigata".to_string(), "150010".to_string());
  map.insert("nagaoka".to_string(), "150020".to_string());
  map.insert("takada".to_string(), "150030".to_string());
  map.insert("aikawa".to_string(), "150040".to_string());

  map.insert("富山".to_string(), "160010".to_string());
  map.insert("伏木".to_string(), "160020".to_string());

  map.insert("toyama".to_string(), "160010".to_string());
  map.insert("fushiki".to_string(), "160020".to_string());

  map.insert("金沢".to_string(), "170010".to_string());
  map.insert("輪島".to_string(), "170020".to_string());

  map.insert("kanazawa".to_string(), "170010".to_string());
  map.insert("wajima".to_string(), "170020".to_string());

  map.insert("福井".to_string(), "180010".to_string());
  map.insert("敦賀".to_string(), "180020".to_string());

  map.insert("fukui".to_string(), "180010".to_string());
  map.insert("tsuruga".to_string(), "180020".to_string());

  map.insert("甲府".to_string(), "190010".to_string());
  map.insert("河口湖".to_string(), "190020".to_string());

  map.insert("kofu".to_string(), "190010".to_string());
  map.insert("kawaguchiko".to_string(), "190020".to_string());

  map.insert("長野".to_string(), "200010".to_string());
  map.insert("松本".to_string(), "200020".to_string());
  map.insert("飯田".to_string(), "200030".to_string());

  map.insert("nagano".to_string(), "200010".to_string());
  map.insert("matsumoto".to_string(), "200020".to_string());
  map.insert("iida".to_string(), "200030".to_string());

  map.insert("岐阜".to_string(), "210010".to_string());
  map.insert("高山".to_string(), "210020".to_string());

  map.insert("gifu".to_string(), "210010".to_string());
  map.insert("takayama".to_string(), "210020".to_string());

  map.insert("静岡".to_string(), "220010".to_string());
  map.insert("網代".to_string(), "220020".to_string());
  map.insert("三島".to_string(), "220030".to_string());
  map.insert("浜松".to_string(), "220040".to_string());

  map.insert("shizuoka".to_string(), "220010".to_string());
  map.insert("ajiro".to_string(), "220020".to_string());
  map.insert("mishima".to_string(), "220030".to_string());
  map.insert("hamamatsu".to_string(), "220040".to_string());

  map.insert("名古屋".to_string(), "230010".to_string());
  map.insert("豊橋".to_string(), "230020".to_string());

  map.insert("nagoya".to_string(), "230010".to_string());
  map.insert("toyohashi".to_string(), "230020".to_string());

  map.insert("津".to_string(), "240010".to_string());
  map.insert("尾鷲".to_string(), "240020".to_string());

  map.insert("tsu".to_string(), "240010".to_string());
  map.insert("owase".to_string(), "240020".to_string());

  map.insert("大津".to_string(), "250010".to_string());
  map.insert("彦根".to_string(), "250020".to_string());

  map.insert("ootsu".to_string(), "250010".to_string());
  map.insert("hikone".to_string(), "250020".to_string());

  map.insert("京都".to_string(), "260010".to_string());
  map.insert("舞鶴".to_string(), "260020".to_string());

  map.insert("kyoto".to_string(), "260010".to_string());
  map.insert("maizuru".to_string(), "260020".to_string());

  map.insert("大阪".to_string(), "270000".to_string());

  map.insert("osaka".to_string(), "270000".to_string());

  map.insert("神戸".to_string(), "280010".to_string());
  map.insert("豊岡".to_string(), "280020".to_string());

  map.insert("kobe".to_string(), "280010".to_string());
  map.insert("toyooka".to_string(), "280020".to_string());

  map.insert("奈良".to_string(), "290010".to_string());
  map.insert("風屋".to_string(), "290020".to_string());

  map.insert("nara".to_string(), "290010".to_string());
  map.insert("kazeya".to_string(), "290020".to_string());

  map.insert("和歌山".to_string(), "300010".to_string());
  map.insert("潮岬".to_string(), "300020".to_string());

  map.insert("wakayama".to_string(), "300010".to_string());
  map.insert("shionomisaki".to_string(), "300020".to_string());

  map.insert("鳥取".to_string(), "310010".to_string());
  map.insert("米子".to_string(), "310020".to_string());

  map.insert("tottori".to_string(), "310010".to_string());
  map.insert("yonago".to_string(), "310020".to_string());

  map.insert("松江".to_string(), "320010".to_string());
  map.insert("浜田".to_string(), "320020".to_string());
  map.insert("西郷".to_string(), "320030".to_string());

  map.insert("matsue".to_string(), "320010".to_string());
  map.insert("hamada".to_string(), "320020".to_string());
  map.insert("nishigo".to_string(), "320030".to_string());

  map.insert("岡山".to_string(), "330010".to_string());
  map.insert("津山".to_string(), "330020".to_string());

  map.insert("okayama".to_string(), "330010".to_string());
  map.insert("tsuyama".to_string(), "330020".to_string());

  map.insert("広島".to_string(), "340010".to_string());
  map.insert("庄原".to_string(), "340020".to_string());

  map.insert("hiroshima".to_string(), "340010".to_string());
  map.insert("shobara".to_string(), "340020".to_string());

  map.insert("下関".to_string(), "350010".to_string());
  map.insert("山口".to_string(), "350020".to_string());
  map.insert("柳井".to_string(), "350030".to_string());
  map.insert("萩".to_string(), "350040".to_string());

  map.insert("shimonoseki".to_string(), "350010".to_string());
  map.insert("yamaguchi".to_string(), "350020".to_string());
  map.insert("yanai".to_string(), "350030".to_string());
  map.insert("hagi".to_string(), "350040".to_string());

  map.insert("徳島".to_string(), "360010".to_string());
  map.insert("日和佐".to_string(), "360020".to_string());

  map.insert("tokushima".to_string(), "360010".to_string());
  map.insert("hiwasa".to_string(), "360020".to_string());

  map.insert("高松".to_string(), "370000".to_string());

  map.insert("takamatsu".to_string(), "370000".to_string());

  map.insert("松山".to_string(), "380010".to_string());
  map.insert("新居浜".to_string(), "380020".to_string());
  map.insert("宇和島".to_string(), "380030".to_string());

  map.insert("matsuyama".to_string(), "380010".to_string());
  map.insert("niihama".to_string(), "380020".to_string());
  map.insert("uwajima".to_string(), "380030".to_string());

  map.insert("高知".to_string(), "390010".to_string());
  map.insert("室戸岬".to_string(), "390020".to_string());
  map.insert("清水".to_string(), "390030".to_string());

  map.insert("kochi".to_string(), "390010".to_string());
  map.insert("murotozaki".to_string(), "390020".to_string());
  map.insert("simizu".to_string(), "390030".to_string());

  map.insert("福岡".to_string(), "400010".to_string());
  map.insert("八幡".to_string(), "400020".to_string());
  map.insert("飯塚".to_string(), "400030".to_string());
  map.insert("久留米".to_string(), "400040".to_string());

  map.insert("fukuoka".to_string(), "400010".to_string());
  map.insert("yawata".to_string(), "400020".to_string());
  map.insert("iizuka".to_string(), "400030".to_string());
  map.insert("kurume".to_string(), "400040".to_string());

  map.insert("佐賀".to_string(), "410010".to_string());
  map.insert("伊万里".to_string(), "410020".to_string());

  map.insert("saga".to_string(), "410010".to_string());
  map.insert("imari".to_string(), "410020".to_string());

  map.insert("長崎".to_string(), "420010".to_string());
  map.insert("佐世保".to_string(), "420020".to_string());
  map.insert("厳原".to_string(), "420030".to_string());
  map.insert("福江".to_string(), "420040".to_string());

  map.insert("nagasaki".to_string(), "420010".to_string());
  map.insert("sasebo".to_string(), "420020".to_string());
  map.insert("izuhara".to_string(), "420030".to_string());
  map.insert("fukue".to_string(), "420040".to_string());

  map.insert("熊本".to_string(), "430010".to_string());
  map.insert("阿蘇乙姫".to_string(), "430020".to_string());
  map.insert("牛深".to_string(), "430030".to_string());
  map.insert("人吉".to_string(), "430040".to_string());

  map.insert("kumamoto".to_string(), "430010".to_string());
  map.insert("asootohime".to_string(), "430020".to_string());
  map.insert("ushibuka".to_string(), "430030".to_string());
  map.insert("hitoyoshi".to_string(), "430040".to_string());

  map.insert("大分".to_string(), "440010".to_string());
  map.insert("中津".to_string(), "440020".to_string());
  map.insert("日田".to_string(), "440030".to_string());
  map.insert("佐伯".to_string(), "440040".to_string());

  map.insert("ooita".to_string(), "440010".to_string());
  map.insert("nakatsu".to_string(), "440020".to_string());
  map.insert("hita".to_string(), "440030".to_string());
  map.insert("saiki".to_string(), "440040".to_string());

  map.insert("宮崎".to_string(), "450010".to_string());
  map.insert("延岡".to_string(), "450020".to_string());
  map.insert("都城".to_string(), "450030".to_string());
  map.insert("高千穂".to_string(), "450040".to_string());

  map.insert("miyazaki".to_string(), "450010".to_string());
  map.insert("nobeoka".to_string(), "450020".to_string());
  map.insert("miyakonojo".to_string(), "450030".to_string());
  map.insert("takachiho".to_string(), "450040".to_string());

  map.insert("鹿児島".to_string(), "460010".to_string());
  map.insert("鹿屋".to_string(), "460020".to_string());
  map.insert("種子島".to_string(), "460030".to_string());
  map.insert("名瀬".to_string(), "460040".to_string());

  map.insert("kagoshima".to_string(), "460010".to_string());
  map.insert("kanoya".to_string(), "460020".to_string());
  map.insert("tanegashima".to_string(), "460030".to_string());
  map.insert("naze".to_string(), "460040".to_string());

  map.insert("那覇".to_string(), "471010".to_string());
  map.insert("名護".to_string(), "471020".to_string());
  map.insert("久米島".to_string(), "471030".to_string());
  map.insert("南大東".to_string(), "472000".to_string());
  map.insert("宮古島".to_string(), "473000".to_string());
  map.insert("石垣島".to_string(), "474010".to_string());
  map.insert("与那国島".to_string(), "474020".to_string());

  map.insert("naha".to_string(), "471010".to_string());
  map.insert("nago".to_string(), "471020".to_string());
  map.insert("kumejima".to_string(), "471030".to_string());
  map.insert("minamidaito".to_string(), "472000".to_string());
  map.insert("miyakojima".to_string(), "473000".to_string());
  map.insert("ishigakijima".to_string(), "474010".to_string());
  map.insert("yonagunijima".to_string(), "474020".to_string());

  map
}

pub fn find_city_id(city: &str) -> Option<String> {
  let db = make_db();
  db.get(city).map(|s| s.to_string().to_lowercase())
}

# 天気取得用API


GETを用いて天気を取得します。


## 必要なパラメータ

クエリパラメータで

- 天気を取得したい日付
- 天気を取得したい地点

の二つの情報を渡します。どちらも必須です。

日付は`date`で、地点は`city`で指定します。


## 例


```sh
curl -GET "localhost:55555/api/v1/weather?city=tokyo&date=2022-02-11"

curl -GET "localhost:55555/api/v1/weather?city=%E6%9D%B1%E4%BA%AC&date=2022-02-11"
```


## 返ってくるデータ


### 成功時

`200 OK`と共に

- `ok`: bool
- `date`: String
- `city`: String
- `weather`: String
- `chance_of_rain`: json
- `copyright`: json

の6つのフィールドを持つjsonが返ってきます。

`ok`は`true`が入っています。

`date`はyyyy-mm-dd形式の日付が文字列で入っています。

`city`はその地点の名前が日本語表記で入っています。

`weather`はその日の天気が文字列で入っています。

ここまでが必ず入っているフィールドです。

`chance_of_rain`と`copyright`は入っているかどうかは保証されません。

`chance_of_rain`は時間帯ごとの降水確率のデータが入っています。

`copyright`は取得したデータに関するcopyrightを表示します。


レスポンス例：

```json
{
  "ok": true,
  "date": "2022-02-11",
  "city": "東京",
  "weather": "晴一時雪か雨",
  "chance_of_rain": {
    "T00_06": "50%",
    "T06_12": "10%",
    "T12_18": "10%",
    "T18_24": "10%"
  },
  "copyright": {
    "image": {
      "height": 120,
      "link": "https://weather.tsukumijima.net/",
      "title": "天気予報 API（livedoor 天気互換）",
      "url": "https://weather.tsukumijima.net/logo.png",
      "width": 120
    },
    "link": "https://weather.tsukumijima.net/",
    "provider": [
      {
        "link": "https://www.jma.go.jp/jma/",
        "name": "気象庁 Japan Meteorological Agency",
        "note": "気象庁 HP にて配信されている天気予報を JSON データへ編集しています。"
      }
    ],
    "title": "(C) 天気予報 API（livedoor 天気互換）"
  }
}
```


### 失敗時

`400 Bad Request`のエラーと共に

- `ok`: bool
- `msg`: String

の二つのフィールドを持つjsonが返ってきます。

`ok`は`false`が入っています。

`msg`にはエラーメッセージが入っており、

- `invalid_city_name`（対応する地点が存在しなかった）
- `date_not_found`（予報の中に目的の日付が含まれていなかった）

の2つが現在存在しています。

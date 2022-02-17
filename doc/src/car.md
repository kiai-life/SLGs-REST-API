# 車関係のAPI

## 車のID登録API

JSON形式でデータを送り、登録します

## 必要なパラメータ

ヘッダでは

- `x-slgs-user-id`でユーザーのID
- `x-slgs-user-password`でユーザーのパスワード

を与えます

JSONデータのフィールドは

- `id`: String（車に振るID）
- `car_model`: String（車種）
- `fuel_type`: String（ガソリンの種類）
- `manufacture`: String（メーカー）

が全て必須です


例：

```sh
curl -X POST -H "Content-Type: application/json" -H "x-slgs-user-id:xxxx" -H "x-slgs-user-password:yyyy" -d "{\"id\": \"car1\", \"car_model\": \"ランサーセディア\", \"fuel_type\": \"ハイオク\", \"manufacture\":\"三菱\"}" "localhost:55555/api/v1/car/register"
```


## 給油時の記録をするAPI

JSON形式でデータを送り、走行距離や給油量を記録します

## 必要なパラメータ

ヘッダでは

- `x-slgs-user-id`でユーザーのID
- `x-slgs-user-password`でユーザーのパスワード

を与えます

JSONデータのフィールドは

- `id`: String（給油した車のID）
- `mileage`: f32（走行距離）
- `amount_of_fuel`（給油量）

が必須です。


- `date`: Option<String>（日付、RFC3339形式のみ）
- `price_per_liter`: Option<f64>（ガソリンのLあたりの値段）

がオプションです。


例：

```sh
curl -X POST -H "Content-Type: application/json" -H "x-slgs-user-id:xxxx" -H "x-slgs-user-password:yyyy" -d "{\"id\": \"car1\", \"mileage\": 327.1, \"amount_of_fuel\":30.5, \"date\": \"2022-02-17T23:59:59+09:00\"}"  "localhost:55555/api/v1/car/refuel"
```

# In-app packages builder
Tool to generate in-app itmsp/csv package (iTunesConnect, Play Market stores)

#### An example json config (products.json):
```
{
  "android": {
    "bundle_id": "com.company.app"
    "currency_rate": 65.0
  },
  "ios": {
    "bundle_id": "com.company.app",
    "app_store_id": "111111111",
    "team_id": "222222222",
    "app_name": "App Name",
    "token": "app-metadata-token"
  },
  "products": [
    {
      "id": "heropack1",
      "price": 2,
      "consumable": false,
      "image_path": "tests/gold1.jpg",
      "price_variants": [1, 2, 3],
      "locales": {
        "en_US": {
          "title": "Heroes Pack",
          "description": "Heroes Pack"
        },
        "ru_RU": {
          "title": "Набор Героев",
          "description": "Набор Героев"
        }
      }
    }
  ]
}
```

#### Play Market csv generate:
```inapp_builder -f tests/products.json -o out -p android```

#### App Store itmsp generate:
```inapp_builder -f tests/products.json -o out -p ios```

Use Application Loader application to upload generated in-apps to the App Store 

### Config details:

#### Play Market
```
  "android": {
    "bundle_id": "com.company.app"
    "currency_rate": 65.0
  }
```
| Parameter | Description |
|---|---|
|bundle_id|A package name of the application |
|currency_rate|A rate to convert price tier from the config to Play Market default price |

#### App Store
```
  "ios": {
    "bundle_id": "com.company.app",
    "app_store_id": "111111111",
    "team_id": "222222222",
    "app_name": "App Name",
    "token": "app-metadata-token"
  },
```
| Parameter | Description |
|---|---|
|bundle_id|A package name of the application |
|app_store_id|ID the application in the App Store |
|team_id|ID of App Store team (development account ID)|
|app_name| Application Display Name |
|token| A token to create a metadata (see below)|

**How to get a token**:

Run on terminal (OS X):

```iTMSTransporter -m lookupMetadata -u {login} -p "{password}" -destination .``` 

Then open the file 111111111.itmsp/metadata.xml and find the line:
```
<metadata_token>5685189406560-f38766f68765157ad0fc5f3a3a...</metadata_token>
```

#### Product Description
```
{
  "id": "heropack1",
  "price": 2,
  "consumable": false,
  "image_path": "tests/gold1.jpg",
  "price_variants": [1, 2, 3],
  "locales": {
    "en_US": {
      "title": "Heroes Pack",
      "description": "Heroes Pack"
    },
    "ru_RU": {
      "title": "Набор Героев",
      "description": "Набор Героев"
    }
  }
}
```
| Parameter | Description |
|---|---|
|id| Unical ID of the product |
|price|Price in USD|
|consumable| true/false. Type on product (used only in the App Store)|
|image_path| Optional parameter. Path to image with in-app screenshot (used only in the App Store)|
|price_variants| Optional parameter. Product price variants. Will be generated few products with postfix of price (*com.company.app.heropack1_1*, *com.company.app.heropack1_2*, ...) 
|locales|Title and description of product. Support few localizations|

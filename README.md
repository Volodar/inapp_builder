# In-app packages builder
Tool to generate in-app itmsp/csv package (iTunesConnect, Play Market stores)

### An example json config (products.json):
```
{
  "android": {
    "config_currency": "usd",
    "store_currency": "rub",
    "bundle_id": "com.company.app"
  },
  "ios": {
    "app_store_id": "111111111",
    "team_id": "222222222",
    "bundle_id": "com.company.app",
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

### Play Market csv generate:
```inapp_builder -f tests/config.json -o out -p android```

### App Store itmsp generate:
```inapp_builder -f tests/config.json -o out -p ios```

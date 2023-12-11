sea-orm-cli migrate -u "sqlite://temp.sqlite?mode=rwc"
sea-orm-cli generate entity -u "sqlite://temp.sqlite?mode=rwc" -o ./src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --enum-extra-attributes 'serde(rename_all = "camelCase")'
rm ./temp.sqlite
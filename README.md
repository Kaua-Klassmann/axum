# Repositório usado para servir de "colinha"

## Comando para iniciar as configurações de migration

```
sea-orm-cli migrate init -d migration
```

## Comando para criar migration de uma tabela

```
sea-orm-cli migrate generate create_table_<TABLE_NAME>
```

## Comando para gerar as entities

```
sea-orm-cli generate entity -o entity/src -u <DATABASE_URL>
```

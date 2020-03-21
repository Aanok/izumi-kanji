A toy utility to learn kanji. Built for personal use during the Japanese language courses held by [Associazione Culturale Izumi](https://izumimassa.com/). Also as baby's first introduction to Rust.

## Build
Simply

```
cargo build --release
```

## Usage
### Invocation

```
izumi-kanji [ all | latest | id ] CSV_FILE
```

Where:
- **all** will loop randomized queries on all records in the file.
- **latest** will loop randomized queries only on records sharing the same id with the first record.
- **id** will loop randomized queries only on records with the provided id.

If neither of these three arguments is passed, `latest` is assumed.

### CSV file
The program expects to be passed a CSV file where each record describes a kanji expression you're trying to learn, formatted as follows:

```
id, latin, kanji, kana
```

Where:
- **id** is some arbitrary unique identifier for a week's assignment. May be integers or dates.
- **latin** is the translation of the kanji expression in your language of choice.
- **kanji** is the kanji expression you're trying to learn.
- **kana** is the hiragana or katakana phonetic transcription of the kanji expression.

The CSV file should not have a header.

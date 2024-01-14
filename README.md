## ubungen is a command-line interface program that helps to learn German words and phrases. 

### Spaced repetition logics: 
1. The entry is selected randomly 
1. Each entry has to have 5 correct attempts. Each entry is recorded into journal_log.json as well as the number of the correct attempts for each entry. The entries from the journal_log.json file are selected randomly (not yet implemented). 

### How to use this program: 
1. Run ubungen program with the parameter "w" to learn German words
```console
   ./ubungen w
```
If using cargo: 
```console
   cargo run -- w
```

2. Run ubungen program with the parameter "p" to learn German phrases
```console
   ./ubungen p
```
If using cargo: 
```console
   cargo run -- p
```
3. Run ubungen program with the parameter "?" to get help
```console
   ./ubungen ?
```
If using cargo: 
```console
   cargo run -- ?
```
If the keyboard does not have the specific letters used in German language, such as "ß" or "ü", one can use the following table to type these characters: 

Type: ctrl + shift + u, then type the code below, which corresponds to the character you want to type: 

| Character | Code    |
| :----:    | :----:  |
| Ä         | 00c4    |
| ä         | 00e4    |
| Ö         | 00d6    |
| ö         | 00f6    |
| Ü         | 00dc    |
| ü         | 00fc    |
| ß         | 00df    |
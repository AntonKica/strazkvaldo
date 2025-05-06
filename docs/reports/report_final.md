# Info o projekte:
- Meno a priezvisko: Anton Kica
- Názov projektu: Strážca kvality domu
- Link na repozitár: [https://github.com/AntonKica/strazkvaldo](https://github.com/AntonKica/strazkvaldo)
- Link na verejnú inštanciu projektu: [https://www.strazkvaldo.chodiacidotaznik.xyz/ui](https://www.strazkvaldo.chodiacidotaznik.xyz/ui)

# Info o reportovanej verzii:
- Tag: final

# Info k testovaniu:     
- admin:admin123
- jankohrasko:jankohrasko

# Postup, ako rozbehať vývojové prostredie 
Potrebné: PosgtreSQL 17

- strazkvaldo-ui: `npm install; npm run dev`, plus README v UI
- strazkvaldo-backend: `cargo build; cargo run`, plus Makefile pre setup databázy, potrebné nastaviť premenné `DATABASE_URL=dburl` a `ENVIRONMENT=dev`

# Stav implementácie:
| funkcionality | stav |
| ---- | --- |
| jednorázové aktivity | implementované |
| opakované aktivity | implementované |
| miestnosti | implementované |
| číselníky | implementované |
| nadchádzajúce aktivity | implementované |
| ukončené aktivity | implementované |
| prihlasovanie a užívatelia | spravené, spravuje administrátor |
| notifikácie a upozornenia | neimplementované |

# Retrospektíva:
Retrospektívne by som priebežnú implementáciu robil priadnejšie. Nechcel som "overengineerovať" kód, ale copy-pasta ma spätne dohnala.
Možno ešte kúsok lepšie si premyslieť funkcionalitu, lebo notifíkacie a upozornenia som si nebol istý, ako integrovať. Ešte nemám
dobrý error handling, ale to je nadlho.

So svojou prácou som viac-menej spokojný. Moja apka nemá to také premakané UI ako iné projekty (viď Kyberkuchárka), na druhú stranu som
chcel niečo prosté a jednoduché. Som rád, že sa mi podarilo nejak spojiť Javascript Svelte Kit a Rust Actix Web frameworky.

Tiež som rád, že sa mi podarilo vymyslieť prostý a jednoduchý mechanizmus generovania plánovaných aktivít. Totiž ešte len nedávno som
si nevedel predstaviť, ako to vyriešiť.
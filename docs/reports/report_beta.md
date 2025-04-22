# Info o projekte:
- Meno a priezvisko: Anton Kica
- Názov projektu: Strážca kvality domu
- Link na repozitár: [https://github.com/AntonKica/strazkvaldo](https://github.com/AntonKica/strazkvaldo)
- Link na verejnú inštanciu projektu: [https://www.strazkvaldo.chodiacidotaznik.xyz/ui](https://www.strazkvaldo.chodiacidotaznik.xyz/ui)

# Info o reportovanej verzii:
- Tag: beta

# Info k testovaniu:     

admin:admin124

jankohrasko:jankohrasko

# Postup, ako rozbehať vývojové prostredie 
Potrebné: PosgtreSQL 17

- strazkvaldo-ui: `npm install; npm run dev`, plus README v UI
- strazkvaldo-backend: `cargo build; cargo run`, plus Makefile pre setup databázy, potrebné nastaviť premenné `DATABASE_URL=dburl` a `ENVIRONMENT=dev`

# Stav implementácie:
| funkcionality | stav |
| ---- | --- |
| jednorázové aktivity | chýba delete |
| opakované aktivity | chýba delete |
| miestnosti | chýba delete |
| číselníky | chýba aktualizácia |
| nadchádzajúce aktivity | chýba užívateľské UI pre správu |
| ukončené aktivity | chýba užívateľské UI pre správu |
| prihlasovanie a užívatelia | spravené, spravuje administrátor |
| notifikácie a upozornenia | neimplementované |

# Časový plán:
| týždeň | od | do | funkcionalita |
| ------ | -- | -- | ------------- |
| 10 | 21.04 | 27.04 | implementácia plnohodnotného užívateľského rozhrania, implementácia delete funkcionality |
| 11 (finálna verzia)| 28.04 | 04.05 | implementácia notifikácií, upozornení, drobných features, ladenie a testovanie funkčnosti |

# Problémy:
Ako sa nabaľovali fíčury, zanedbaná implementácia (napr. string vs datetime timestamp) ma dobehli vo vývoji a bolo ich treba prepracovať, aby vývoj bol ako-tak udržateľný.
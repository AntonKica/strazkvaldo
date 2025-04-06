# Info o projekte:
- Meno a priezvisko: Anton Kica
- Názov projektu: Strážca kvality domu
- Link na repozitár: [https://github.com/AntonKica/strazkvaldo](https://github.com/AntonKica/strazkvaldo)

# Info o reportovanej verzii:  
<!-- Upraviť podľa aktuálneho týždňa, reporty začínajú 4. týždeň semestra. Upraviť aj názov reportu. -->
- Tag: week7
- Obdobie: 7. týždeň, 31.03. - 06.04.2025 

# Plán:
implementácia entity používateľa
implementácia administrátorského rozhrania a autentifikácie používateľov

# Vykonaná práca:
| commit | message |
| ------ | ------- |
| d813a19 | [week7] Added app_user CRUD and auth filter chain |
| fa32542 | [week7] Added app_user schema |
| 3415f5e | [week7] Added cargo packages for basic authentification |
| 49a4acd | [week7] Added user edit page for admin |
| 282c150 | [week7] Added user view page for admin |
| 249c282 | [week7] Added common user fetch for code |
| b59f99c | [week7] Added more attributes to admin's user list page |
| df0d76b | [week7] Added app_user_role_to_string to lib |
| fbaea97 | [week7] Added user create page for admin |
| 72ebef2 | [week7] Updated user page to fetch data from server and display correct data |
| 6082e2c | [week7] Added localhost proxy config |
| fbe27f1 | [week7] Changed package versio and refactored unused imports and variables |

# Zdôvodnenie rozdielov medzi plánom a vykonanou prácou:
prekvapivo sa podarilo temer všetko, iba na ui nie je filter pre user/admin

# Plán na ďalší týždeň:
úprava užívateľského rozhrania podľa roly
konfigurácia a príprava produkčného prostredia

# Problémy:
Je toho veľa na robenie, chcel by som dostať zaplatené za to.

# Zmeny v špecifikácii:
Zmenil som MySQL na PostgreSQL, lebo je typovo bohatší a upravil som link na hosting.
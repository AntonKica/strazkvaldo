# Info o projekte:
- Meno a priezvisko: Anton Kica
- Názov projektu: Strážca kvality domu
- Link na repozitár: [https://github.com/AntonKica/strazkvaldo](https://github.com/AntonKica/strazkvaldo)

# Info o reportovanej verzii:  
<!-- Upraviť podľa aktuálneho týždňa, reporty začínajú 4. týždeň semestra. Upraviť aj názov reportu. -->
- Tag: week9
- Obdobie: 9. týždeň, 14.04. - 22.04.2025 

# Plán:
implementácia nastavení, hlásení a upozornení

# Vykonaná práca:
| commit | message |
| ------ | ------- |
| 21f2ddc | [week9] Added upcoming activities, finished activities, cronjob for generating finished activities for a day and moved web::data to arc |
| 51d25fe | [week9] Removed start and end dates from RepeatingActivity and added periodicity unit |
| b7ae358 | [week9] Moved app-user-role and criticality-type enums to application enums and created periodicityt enum and repeated activity pages |
| 91c4395 | [week9] Added full crud for OneTimeActivity |
| 4e2aaf6 | [week9] Moved enums from constants to db |
| 33b61ac | [week9] Split activities to onetimeactivity and added room view and edit |
| 2cc84a4 | [week9] Created CRUD for room on backend |
| 3b91bb8 | [week9] Addded default values into database creation script |
| 28cf10c | [week9] Refactored handlers to separate folder and removed unused imports |
| ecc2dbe | [week9] Updated admin service paths |
| 1c4c21e | [week9] Implemented login functionality on ui |
| 20b05c1 | [week9] Added default user on database creation |
| df61449 | [week9] Added role handling for service routes |
| a74659e | [week9] Removed debug logging |

# Zdôvodnenie rozdielov medzi plánom a vykonanou prácou:
Vyskytlo, že mnohé veci boli nedobre implementované, tak som ich musel prepísať. Navyše, neviem, čo presne chcem s tými upozorneniami docieliť, tak som ich nechal tak.

# Plán na ďalší týždeň:
implementácia nastavení, hlásení, upozornení a skrášľovanie UI

# Problémy:
Zanedbaná implementácia a Rust konštrukty.

# Zmeny v špecifikácii:
Žiadne.


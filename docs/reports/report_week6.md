# Info o projekte:
- Meno a priezvisko: Anton Kica
- Názov projektu: Strážca kvality domu
- Link na repozitár: [https://github.com/AntonKica/strazkvaldo](https://github.com/AntonKica/strazkvaldo)

# Info o reportovanej verzii:  
<!-- Upraviť podľa aktuálneho týždňa, reporty začínajú 4. týždeň semestra. Upraviť aj názov reportu. -->
- Tag: week6
- Obdobie: 6. týždeň, 24.03. - 30.03.2025 

# Plán:
implementácia CRUD pre entity aktivít na backend-e

# Vykonaná práca:
| commit | message |
| ------ | ------- |
| 4dc172a | [week6] Added schemas, models and models for repeated activity |
| 11b5145 | [week6] Spllit up handler to separate handler file for one time activity |
| 56ca1f0 | [week6] Added repeated_activity table to migrations up/down scripts |
| 4def46e | [week6] Initialized Actix and created CRUD for onetimeactivity |
| 07bdfc0 | [week6] Added migration scripts and makefile for database setup |
| 395814b | [week6] Updated cargo dependencies, added sqlx and required features for PostgreSQL |

# Zdôvodnenie rozdielov medzi plánom a vykonanou prácou:
Implementácia CRUD pre entity bola časovo náročná pre prvú entity, pre druhu i repetitívna. Zvyšné entity som ešte neimplementoval,
ešte uvidím, ako sa vyvinie zvyšok, aby som zbytočne neprepisoval veci. 

# Plán na ďalší týždeň:
implementácia entity používateľa
implementácia administrátorského rozhrania a autentifikácie používateľov

# Problémy:
Pomerne repetitívny kód na v CRUD, ešte uvidím, ako by to bolo dobré abstrahovať.

# Zmeny v špecifikácii:
Zatiaľ žiadne.

